pub(super) mod endpoints;

use crate::{
    request::send_get_request, url::Url, utils::build_header_map,
    utils::create_client_with_project_id, BlockFrostSettings, BlockfrostError, Pagination,
};
use reqwest::ClientBuilder;
use std::future::Future;

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

    fn call_endpoint<T>(
        &self,
        url_endpoint: &str,
    ) -> impl Future<Output = Result<T, BlockfrostError>> + Send
    where
        T: serde::de::DeserializeOwned,
    {
        let url = Url::from_endpoint(self.base_url.clone(), url_endpoint);
        send_get_request(&self.client, url, self.settings.retry_settings)
    }

    pub fn call_paged_endpoint<T>(
        &self,
        url_endpoint: &str,
        pagination: Pagination,
    ) -> impl Future<Output = Result<T, BlockfrostError>> + Send
    where
        T: serde::de::DeserializeOwned + 'static,
    {
        async move {
            let url_result =
                Url::from_paginated_endpoint(self.base_url.clone(), url_endpoint, pagination);

            match url_result {
                Ok(url) => send_get_request(&self.client, url, self.settings.retry_settings).await,
                Err(e) => Err(e.into()),
            }
        }
    }
}
