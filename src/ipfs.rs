use crate::{
    error::{json_error, process_error_response, reqwest_error},
    request::{send_request, send_request_unprocessed},
    utils::{build_header_map, create_client_with_project_id},
    BlockfrostError, Integer, IpfsSettings, RetrySettings, IPFS_URL,
};
use blockfrost_openapi::models::_ipfs_pin_list__ipfs_path__get_200_response::IpfsPinListIpfsPathGet200Response;
use reqwest::{
    multipart::{Form, Part},
    ClientBuilder,
};
use serde::{Deserialize, Serialize};
use serde_json::from_str as json_from;

/// Provides methods for making requests to the
/// [IPFS API](https://docs.blockfrost.io/#tag/IPFS-Add).
#[derive(Debug, Clone)]
pub struct BlockfrostIPFS {
    pub base_url: String,
    client: reqwest::Client,
    pub settings: IpfsSettings,
}

impl BlockfrostIPFS {
    /// Create a [`IpfsApi`] with [`custom settings`](IpfsSettings).
    ///
    /// # Panics
    ///
    /// This function might panic if `project_id` could not be converted into a [`HeaderValue`] with
    /// the function [`HeaderValue::from_str`].
    ///
    /// [`HeaderValue`]: reqwest::header::HeaderValue
    /// [`HeaderValue::from_str`]: reqwest::header::HeaderValue::from_str
    pub fn new(project_id: &str, settings: IpfsSettings) -> Self {
        let client = create_client_with_project_id(project_id);

        Self {
            client,
            settings,
            base_url: IPFS_URL.to_string(),
        }
    }

    /// Create a [`IpfsApi`] with [custom settings](IpfsSettings) and [custom client](ClientBuilder).
    ///
    /// This function is a more flexible version of [`IpfsApi::new`], you can customize every
    /// field of the [`ClientBuilder`] argument, however, note that the [`HeaderMap`] will be
    /// [overwritten](ClientBuilder::default_headers) by a map with the given `project_id`.
    ///
    /// If `client_builder` argument is equivalent to `Client::builder()` or `ClientBuilder::new()`,
    /// this function returns the same as [`IpfsApi::new`] without the extra argument.
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
        settings: IpfsSettings,
        client_builder: ClientBuilder,
    ) -> reqwest::Result<Self> {
        client_builder
            .default_headers(build_header_map(project_id.as_ref()))
            .build()
            .map(|client| Self {
                settings,
                client,
                base_url: IPFS_URL.to_string(),
            })
    }

    /// Adding a file to `IPFS`.
    ///
    /// Note that you need to `/ipfs/pin/add` an object to avoid it being garbage collected.
    ///
    /// This usage is being counted in your user account quota.
    ///
    /// OpenAPI endpoint reference: [`/ipfs/add`].
    ///
    /// [`/ipfs/add`]: https://docs.blockfrost.io/#tag/IPFS-Add/paths/~1ipfs~1add/post
    pub async fn add(&self, file_contents: Vec<u8>) -> Result<IpfsAdd, BlockfrostError> {
        let url = self.base_url.clone() + "/ipfs/add";

        let part = Part::bytes(file_contents);
        let form = Form::new().part("file", part);

        let request = self.client.post(&url).multipart(form);

        let (status, text) = send_request(request, self.settings.retry_settings)
            .await
            .map_err(|reason| reqwest_error(&url, reason))?;

        if !status.is_success() {
            return Err(process_error_response(&text, status, &url));
        }

        json_from(&text).map_err(|reason| json_error(url, text, reason))
    }

    /// Retrieve an object from the IFPS gateway.
    ///
    /// Useful if you do not want to rely on a public gateway, such as <ipfs.blockfrost.dev>.
    ///
    /// OpenAPI endpoint reference: [`/ipfs/gateway/{IPFS_path}`].
    ///
    /// [`/ipfs/gateway/{IPFS_path}`]: https://docs.blockfrost.io/#tag/IPFS-Gateway
    pub async fn gateway(&self, ipfs_path: &str) -> Result<Vec<u8>, BlockfrostError> {
        let url =
            self.base_url.clone() + &format!("/ipfs/gateway/{IPFS_path}", IPFS_path = ipfs_path);

        let request = self.client.get(&url);

        let response = send_request_unprocessed(request, self.retry_settings())
            .await
            .map_err(|reason| reqwest_error(&url, reason))?;
        let status = response.status();

        if !status.is_success() {
            let text = response
                .text()
                .await
                .map_err(|reason| reqwest_error(&url, reason))?;
            Err(process_error_response(&text, status, &url))
        } else {
            let bytes = response
                .bytes()
                .await
                .map_err(|reason| reqwest_error(&url, reason))?;
            Ok(bytes.to_vec())
        }
    }

    /// Pinned objects are counted in your user storage quota.
    ///
    /// OpenAPI endpoint reference: [`/ipfs/pin/add/{IPFS_path}`].
    ///
    /// [`/ipfs/pin/add/{IPFS_path}`]: https://docs.blockfrost.io/#tag/IPFS-Pins/paths/~1ipfs~1pin~1add~1{IPFS_path}/post
    pub async fn pin_add(&self, ipfs_path: &str) -> Result<IpfsPinUpdate, BlockfrostError> {
        let url =
            self.base_url.clone() + &format!("/ipfs/pin/add/{IPFS_path}", IPFS_path = ipfs_path);

        let request = self.client.post(&url);
        let (status, text) = send_request(request, self.settings.retry_settings)
            .await
            .map_err(|reason| reqwest_error(&url, reason))?;
        if !status.is_success() {
            return Err(process_error_response(&text, status, &url));
        }

        json_from(&text).map_err(|reason| json_error(url, text, reason))
    }

    /// List objects pinned to local storage.
    ///
    /// OpenAPI endpoint reference: [`/ipfs/pin/list`].
    ///
    /// [`/ipfs/pin/list`]: https://docs.blockfrost.io/#tag/IPFS-Pins/paths/~1ipfs~1pin~1list~1/get
    pub async fn pin_list(
        &self,
    ) -> Result<Vec<IpfsPinListIpfsPathGet200Response>, BlockfrostError> {
        let url = self.base_url.clone() + "/ipfs/pin/list";

        let request = self.client.get(&url);
        let (status, text) = send_request(request, self.settings.retry_settings)
            .await
            .map_err(|reason| reqwest_error(&url, reason))?;

        if !status.is_success() {
            return Err(process_error_response(&text, status, &url));
        }

        json_from(&text).map_err(|reason| json_error(url, text, reason))
    }

    /// Get information about locally pinned IPFS object.
    ///
    /// OpenAPI endpoint reference: [`/ipfs/pin/list/{IPFS_path}`].
    ///
    /// [`/ipfs/pin/list/{IPFS_path}`]: https://docs.blockfrost.io/#tag/IPFS-Pins/paths/~1ipfs~1pin~1list~1{IPFS_path}/get
    pub async fn pin_list_by_id(&self, ipfs_path: &str) -> Result<IpfsPinList, BlockfrostError> {
        let url =
            self.base_url.clone() + &format!("/ipfs/pin/list/{IPFS_path}", IPFS_path = ipfs_path);

        let request = self.client.get(&url);
        let (status, text) = send_request(request, self.settings.retry_settings)
            .await
            .map_err(|reason| reqwest_error(&url, reason))?;

        if !status.is_success() {
            return Err(process_error_response(&text, status, &url));
        }

        json_from(&text).map_err(|reason| json_error(url, text, reason))
    }

    /// Remove pinned objects from local storage.
    ///
    /// OpenAPI endpoint reference: [`/ipfs/pin/remove/{IPFS_path}`].
    ///
    /// [`/ipfs/pin/remove/{IPFS_path}`]: https://docs.blockfrost.io/#tag/IPFS-Pins/paths/~1ipfs~1pin~1remove~1{IPFS_path}/post
    pub async fn pin_remove(&self, ipfs_path: &str) -> Result<IpfsPinUpdate, BlockfrostError> {
        let url =
            self.base_url.clone() + &format!("/ipfs/pin/remove/{IPFS_path}", IPFS_path = ipfs_path);

        let request = self.client.post(&url);
        let (status, text) = send_request(request, self.settings.retry_settings)
            .await
            .map_err(|reason| reqwest_error(&url, reason))?;

        if !status.is_success() {
            return Err(process_error_response(&text, status, &url));
        }

        json_from(&text).map_err(|reason| json_error(url, text, reason))
    }

    pub(crate) fn retry_settings(&self) -> RetrySettings {
        self.settings.retry_settings
    }
}

/// Created by [`add`](IpfsApi::add) method.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IpfsAdd {
    /// Name of the file.
    pub name: String,
    /// IPFS hash of the file.
    pub ipfs_hash: String,
    /// IPFS node size in Bytes.
    pub size: String,
}

/// Created by [`pin_add`](IpfsApi::pin_add) method.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IpfsPinUpdate {
    /// IPFS hash of the pinned object.
    pub ipfs_hash: String,
    /// State of the pin action.
    pub state: IpfsPinState,
}

/// Created by [`pin_list`](IpfsApi::pin_list) method.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IpfsPinList {
    /// Creation time of the IPFS object on our backends.
    pub time_created: Integer,
    /// Pin time of the IPFS object on our backends.
    pub time_pinned: Integer,
    /// IPFS hash of the pinned object.
    pub ipfs_hash: String,
    /// Size of the object in Bytes.
    pub size: String,
    /// State of the pinned object, which is `queued` when we are retriving object. If this is
    /// successful the state is changed to `pinned` or `failed` if not. The state `gc` means the
    /// pinned item has been garbage collected due to account being over storage quota or after it
    /// has been moved to `unpinned` state by removing the object pin.
    pub state: IpfsPinState,
}

/// Inner enum for [`IpfsPinUpdate`].
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum IpfsPinState {
    Queued,
    Pinned,
    Unpinned,
    Failed,
    Gc,
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     test_example! { test_ipfs_add, IpfsAdd, r#"
//     {
//       "name": "README.md",
//       "ipfs_hash": "QmZbHqiCxKEVX7QfijzJTkZiSi3WEVTcvANgNAWzDYgZDr",
//       "size": "125297"
//     }
//     "# }

//     test_example! { test_ipfs_pin_add, IpfsPinUpdate, r#"
//     {
//       "ipfs_hash": "QmPojRfAXYAXV92Dof7gtSgaVuxEk64xx9CKvprqu9VwA8",
//       "state": "queued"
//     }
//     "# }

//     test_example! { test_ipfs_pin_list_by_id, IpfsPinList, r#"
//     {
//       "time_created": 1615551024,
//       "time_pinned": 1615551024,
//       "ipfs_hash": "QmdVMnULrY95mth2XkwjxDtMHvzuzmvUPTotKE1tgqKbCx",
//       "size": "1615551024",
//       "state": "pinned"
//     }
//     "# }
// }
