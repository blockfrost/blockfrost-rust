pub(super) mod endpoints; // Will be reexported
mod settings;

use std::future::Future;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde_json::from_str as serde_from_str;

use crate::error::process_error_response;
use crate::url::Url;
pub use settings::*;

/// SDK version being used.
pub const USER_AGENT: &str = concat!("blockfrost-rust/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Clone)]
pub struct BlockFrostApi {
    settings: Settings,
    client: reqwest::Client,
}

impl BlockFrostApi {
    /// Create a [`BlockFrostApi`] with custom settings.
    ///
    /// # Panics
    ///
    /// This function might panic if `project_id` could not be converted into a [`HeaderValue`] with
    /// the function [`HeaderValue::from_str`].
    ///
    /// [`HeaderValue`]: (reqwest::header::HeaderValue)
    /// [`HeaderValue::from_str`]: (reqwest::header::HeaderValue::from_str)
    pub fn new(project_id: impl AsRef<str>, settings: Settings) -> Self {
        let mut headers = HeaderMap::new();

        let project_id = project_id.as_ref();
        let mut project_id = HeaderValue::from_str(project_id)
            .unwrap_or_else(|_| panic!("Could not parse given project_id '{}' into HeaderValue", project_id));
        project_id.set_sensitive(true);

        let user_agent = HeaderValue::from_static(USER_AGENT);

        headers.insert("project_id", project_id);
        headers.insert("User-Agent", user_agent);

        let client = Client::builder().default_headers(headers).build().unwrap();
        Self { settings, client }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    // Url endpoint example: "/blocks"
    fn get_from_endpoint<T>(&self, url_endpoint: &str) -> impl Future<Output = crate::Result<T>> + Send
    where
        T: serde::de::DeserializeOwned,
    {
        let Url(url) = Url::from_endpoint(self.settings(), url_endpoint);
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

            let debug_info = format!("{}: {}", url, text);
            eprintln!("debug_info: {}", debug_info);

            if !status_code.is_success() {
                return Err(process_error_response(&text, status_code));
            }
            // This gon have to be removed
            Ok(serde_from_str::<T>(&text)?)
        }
    }
}
