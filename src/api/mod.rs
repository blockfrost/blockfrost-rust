pub(super) mod endpoints;
use crate::{
    pagination::{CursorPagination, Pagination},
    request::{fetch_all_pages, send_get_request},
    url::Url,
    utils::build_header_map,
    utils::create_client_with_project_id,
    BlockFrostSettings, BlockfrostError,
};
use reqwest::ClientBuilder;

#[derive(Debug, Clone)]
pub struct BlockfrostAPI {
    base_url: String,
    settings: BlockFrostSettings,
    client: reqwest::Client,
}

impl BlockfrostAPI {
    pub fn new(project_id: &str, settings: BlockFrostSettings) -> Self {
        let client = create_client_with_project_id(project_id, &settings.headers);
        let base_url = settings
            .base_url
            .clone()
            .unwrap_or_else(|| Url::get_base_url_from_project_id(project_id));

        Self {
            settings,
            client,
            base_url,
        }
    }

    pub fn new_with_client(
        project_id: &str, settings: BlockFrostSettings, client_builder: ClientBuilder,
    ) -> reqwest::Result<Self> {
        let base_url = settings
            .base_url
            .clone()
            .unwrap_or_else(|| Url::get_base_url_from_project_id(project_id));

        client_builder
            .default_headers(build_header_map(project_id, &settings.headers))
            .build()
            .map(|client| Self {
                settings,
                client,
                base_url,
            })
    }

    async fn call_endpoint<T>(&self, url_endpoint: &str) -> Result<T, BlockfrostError>
    where
        T: for<'de> serde::Deserialize<'de> + serde::de::DeserializeOwned,
    {
        let url = Url::from_endpoint(self.base_url.as_str(), url_endpoint)?;

        send_get_request(&self.client, url, self.settings.retry_settings).await
    }

    async fn call_paged_endpoint<T>(
        &self, url_endpoint: &str, pagination: Pagination,
    ) -> Result<Vec<T>, BlockfrostError>
    where
        T: for<'de> serde::Deserialize<'de> + serde::de::DeserializeOwned,
    {
        let url = Url::from_paginated_endpoint(self.base_url.as_str(), url_endpoint, pagination)?;

        if pagination.fetch_all {
            fetch_all_pages(
                &self.client,
                &url,
                self.settings.retry_settings,
                pagination,
                10,
            )
            .await
        } else {
            send_get_request(&self.client, url, self.settings.retry_settings).await
        }
    }

    async fn call_cursor_paged_endpoint<T>(
        &self, url_endpoint: &str, pagination: CursorPagination,
    ) -> Result<Vec<T>, BlockfrostError>
    where
        T: for<'de> serde::Deserialize<'de> + serde::de::DeserializeOwned,
    {
        let url =
            Url::from_cursor_paginated_endpoint(self.base_url.as_str(), url_endpoint, &pagination)?;

        if pagination.pagination.fetch_all {
            fetch_all_pages(
                &self.client,
                &url,
                self.settings.retry_settings,
                pagination.pagination,
                10,
            )
            .await
        } else {
            send_get_request(&self.client, url, self.settings.retry_settings).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pagination::Pagination;
    use httpmock::{Method::GET, MockServer};

    #[tokio::test]
    async fn transaction_endpoints_send_cursor_range() {
        let server = MockServer::start();
        let mut settings = BlockFrostSettings::new();
        settings.base_url = Some(server.base_url());
        let api = BlockfrostAPI::new("test", settings);

        let account_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/accounts/stake_test/transactions")
                .query_param("from", "8929261")
                .query_param("to", "9999269:10");
            then.status(200)
                .header("Content-Type", "application/json")
                .body("[]");
        });
        let address_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/addresses/addr_test/transactions")
                .query_param("from", "8929261")
                .query_param("to", "9999269:10");
            then.status(200)
                .header("Content-Type", "application/json")
                .body("[]");
        });
        let asset_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/assets/asset_test/transactions")
                .query_param("from", "8929261")
                .query_param("to", "9999269:10");
            then.status(200)
                .header("Content-Type", "application/json")
                .body("[]");
        });

        let pagination = Pagination::default().with_range("8929261", "9999269:10");
        api.accounts_transactions("stake_test", pagination.clone())
            .await
            .unwrap();
        api.addresses_transactions("addr_test", pagination.clone())
            .await
            .unwrap();
        api.assets_transactions("asset_test", pagination)
            .await
            .unwrap();

        account_mock.assert();
        address_mock.assert();
        asset_mock.assert();
    }
}
