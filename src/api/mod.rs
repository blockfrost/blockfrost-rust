// Will be reexported by the parent module.
pub(super) mod endpoints;
pub(super) mod lister;

use crate::{
    request::send_get_request, url::Url, utils::build_header_map,
    utils::create_client_with_project_id, BlockFrostSettings,
};
use reqwest::ClientBuilder;
use std::future::Future;

/// Provides methods for making requests to the [BlockFrost API](https://docs.blockfrost.io).
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
    /// [`HeaderValue`]: reqwest::header::HeaderValue
    /// [`HeaderValue::from_str`]: reqwest::header::HeaderValue::from_str
    pub fn new(project_id: impl AsRef<str>, settings: BlockFrostSettings) -> Self {
        let client = create_client_with_project_id(project_id.as_ref());
        Self { settings, client }
    }

    /// Create a [`BlockFrostApi`] with [custom settings](BlockFrostSettings) and [custom client](ClientBuilder).
    ///
    /// This function is a more flexible version of [`BlockFrostApi::new`], you can customize every
    /// field of the [`ClientBuilder`] argument, however, note that the [`HeaderMap`] will be
    /// [overwritten](ClientBuilder::default_headers) by a map with the given `project_id`.
    ///
    /// If `client_builder` argument is equivalent to `Client::builder()` or `ClientBuilder::new()`,
    /// this function returns the same as [`BlockFrostApi::new`] without the extra argument.
    ///
    /// # Panics
    ///
    /// This function might panic if `project_id` could not be converted into a [`HeaderValue`] with
    /// the function [`HeaderValue::from_str`].
    ///
    /// [`HeaderMap`]: reqwest::header::HeaderMap
    /// [`HeaderValue`]: reqwest::header::HeaderValue
    /// [`HeaderValue::from_str`]: reqwest::header::HeaderValue::from_str
    pub fn new_with_client(
        project_id: impl AsRef<str>,
        settings: BlockFrostSettings,
        client_builder: ClientBuilder,
    ) -> reqwest::Result<Self> {
        client_builder
            .default_headers(build_header_map(project_id.as_ref()))
            .build()
            .map(|client| Self { settings, client })
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
        send_get_request(&self.client, url, self.settings.retry_settings)
    }
}
