pub(super) mod endpoints;

use crate::{
    request::send_get_request, url::Url, utils::build_header_map,
    utils::create_client_with_project_id, BlockFrostSettings, BlockfrostError, Pagination,
};
use reqwest::ClientBuilder;

#[derive(Debug, Clone)]
pub struct BlockFrostApi {
    base_url: String,
    settings: BlockFrostSettings,
    client: reqwest::Client,
}

impl BlockFrostApi {
    pub fn new(project_id: &str, settings: BlockFrostSettings) -> Self {
        let client = create_client_with_project_id(project_id);
        let base_url = Url::get_base_url_from_project_id(project_id);

        Self {
            settings,
            client,
            base_url,
        }
    }

    pub fn new_with_client(
        project_id: &str,
        settings: BlockFrostSettings,
        client_builder: ClientBuilder,
    ) -> reqwest::Result<Self> {
        let base_url = Url::get_base_url_from_project_id(project_id);

        client_builder
            .default_headers(build_header_map(project_id))
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
        &self,
        url_endpoint: &str,
        pagination: Pagination,
    ) -> Result<Vec<T>, BlockfrostError>
    where
        T: for<'de> serde::Deserialize<'de> + serde::de::DeserializeOwned,
    {
        let url = Url::from_paginated_endpoint(self.base_url.as_str(), url_endpoint, pagination)?;

        send_get_request(&self.client, url, self.settings.retry_settings).await
    }
}
