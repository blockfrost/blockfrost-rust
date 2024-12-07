pub(super) mod endpoints;
use crate::{
    pagination::Pagination,
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
        let base_url = Url::get_base_url_from_project_id(project_id);

        Self {
            settings,
            client,
            base_url,
        }
    }

    pub fn new_with_client(
        project_id: &str, settings: BlockFrostSettings, client_builder: ClientBuilder,
    ) -> reqwest::Result<Self> {
        let base_url = Url::get_base_url_from_project_id(project_id);

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
        self.call_paged_endpoint_batched(url_endpoint, pagination, 10)
            .await
    }

    async fn call_paged_endpoint_unbatched<T>(
        &self, url_endpoint: &str, pagination: Pagination,
    ) -> Result<Vec<T>, BlockfrostError>
    where
        T: for<'de> serde::Deserialize<'de> + serde::de::DeserializeOwned,
    {
        self.call_paged_endpoint_batched(url_endpoint, pagination, 1)
            .await
    }

    async fn call_paged_endpoint_batched<T>(
        &self, url_endpoint: &str, pagination: Pagination, batch_size: usize,
    ) -> Result<Vec<T>, BlockfrostError>
    where
        T: for<'de> serde::Deserialize<'de> + serde::de::DeserializeOwned,
    {
        let url = Url::from_paginated_endpoint(self.base_url.as_str(), url_endpoint, pagination)?;

        if pagination.fetch_all {
            fetch_all_pages(
                &self.client,
                url,
                self.settings.retry_settings,
                pagination,
                batch_size,
            )
            .await
        } else {
            send_get_request(&self.client, url, self.settings.retry_settings).await
        }
    }
}
