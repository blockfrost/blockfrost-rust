// Will be reexported by the parent module.
pub(super) mod endpoints;

pub mod lister;

use std::future::Future;

use reqwest::Client;

use crate::{error::process_error_response, url::Url, utils::build_header_map, BlockFrostSettings};

/// Provides methods for making requests to the [`BlockFrost API`](https://docs.blockfrost.io).
#[derive(Debug, Clone)]
pub struct BlockFrostApi {
    pub settings: BlockFrostSettings,
    client: reqwest::Client,
}

impl BlockFrostApi {
    /// Create a [`BlockFrostApi`] with [`custom settings`](BlockFrostSettings).
    ///
    /// # Panics
    ///
    /// This function might panic if `project_id` could not be converted into a [`HeaderValue`] with
    /// the function [`HeaderValue::from_str`].
    ///
    /// [`HeaderValue`]: (reqwest::header::HeaderValue)
    /// [`HeaderValue::from_str`]: (reqwest::header::HeaderValue::from_str)
    pub fn new(project_id: impl AsRef<str>, settings: BlockFrostSettings) -> Self {
        let header_map = build_header_map(project_id.as_ref());
        let client = Client::builder().default_headers(header_map).build().unwrap();

        Self { settings, client }
    }

    // Url endpoint example: "/blocks"
    fn get_from_endpoint<T>(
        &self,
        url_endpoint: &str,
    ) -> impl Future<Output = crate::Result<T>> + Send
    where
        T: serde::de::DeserializeOwned,
    {
        let Url(url) = Url::from_endpoint(&self.settings, url_endpoint);
        Self::get_from_url(self, url)
    }

    // Url here is the full url
    fn get_from_url<T>(&self, url: String) -> impl Future<Output = crate::Result<T>> + Send
    where
        T: serde::de::DeserializeOwned,
    {
        let response_future = self.client.get(&url).send();

        async move {
            let response = response_future.await?;

            let status_code = response.status();
            let text = response.text().await?;

            // let debug_info = format!("{}: {}", url, text);
            // eprintln!("debug_info: {}", debug_info);

            if !status_code.is_success() {
                return Err(process_error_response(&text, status_code));
            }
            // This gon have to be removed
            Ok(serde_json::from_str::<T>(&text)?)
        }
    }
}
