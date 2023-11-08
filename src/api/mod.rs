// Will be reexported by the parent module.
pub(super) mod endpoints;

use crate::{
    request::send_get_request, url::Url, utils::build_header_map,
    utils::create_client_with_project_id, BlockFrostSettings, BlockfrostError, Pagination,
    CARDANO_MAINNET_URL, CARDANO_PREPROD_URL, CARDANO_PREVIEW_URL,
};
use reqwest::ClientBuilder;
use std::future::Future;

/// Provides methods for making requests to the [BlockFrost API](https://docs.blockfrost.io).
#[derive(Debug, Clone)]
pub struct BlockFrostApi {
    pub base_url: String,
    pub settings: BlockFrostSettings,
    client: reqwest::Client,
}

impl BlockFrostApi {
    pub fn new(project_id: &str, settings: BlockFrostSettings) -> Self {
        let client = create_client_with_project_id(project_id);
        let base_url = match project_id {
            id if id.starts_with("mainnet") => CARDANO_MAINNET_URL,
            id if id.starts_with("preview") => CARDANO_PREVIEW_URL,
            id if id.starts_with("preprod") => CARDANO_PREPROD_URL,
            _ => CARDANO_MAINNET_URL,
        }
        .to_string();

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
        let base_url = match project_id {
            id if id.starts_with("mainnet") => CARDANO_MAINNET_URL,
            id if id.starts_with("preview") => CARDANO_PREVIEW_URL,
            id if id.starts_with("preprod") => CARDANO_PREPROD_URL,
            _ => CARDANO_MAINNET_URL,
        }
        .to_string();

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
                Err(e) => {
                    // Convert the error directly to your `error::Error` type.
                    Err(e.into())
                },
            }
        }
    }
}
