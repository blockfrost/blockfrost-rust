use reqwest::{
    multipart::{Form, Part},
    Client,
};
use serde::{Deserialize, Serialize};

use crate::{error::process_error_response, utils::build_header_map, Integer, IpfsSettings};

/// Provides methods for making requests to the
/// [`IPFS API`](https://docs.blockfrost.io/#tag/IPFS-Add).
#[derive(Debug, Clone)]
pub struct IpfsApi {
    client: reqwest::Client,
    pub settings: IpfsSettings,
}

impl IpfsApi {
    /// Create a [`IpfsApi`] with [`custom settings`](IpfsSettings).
    ///
    /// # Panics
    ///
    /// This function might panic if `project_id` could not be converted into a [`HeaderValue`] with
    /// the function [`HeaderValue::from_str`].
    ///
    /// [`HeaderValue`]: (reqwest::header::HeaderValue)
    /// [`HeaderValue::from_str`]: (reqwest::header::HeaderValue::from_str)
    pub fn new(project_id: impl AsRef<str>, settings: IpfsSettings) -> Self {
        let header_map = build_header_map(project_id.as_ref());
        let client = Client::builder().default_headers(header_map).build().unwrap();
        Self { client, settings }
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
    pub async fn add(&self, file_contents: Vec<u8>) -> crate::Result<IpfsAdd> {
        let url = self.settings.network_address.clone() + "/ipfs/add";

        let part = Part::bytes(file_contents);
        let form = Form::new().part("file", part);

        let request = self.client.post(&url).multipart(form);
        let response = request.send().await?;

        let status_code = response.status();
        let text = response.text().await?;

        if !status_code.is_success() {
            return Err(process_error_response(&text, status_code));
        }

        Ok(serde_json::from_str(&text)?)
    }

    /// Retrieve an object from the IFPS gateway.
    ///
    /// Useful if you do not want to rely on a public gateway, such as <ipfs.blockfrost.dev>.
    ///
    /// OpenAPI endpoint reference: [`/ipfs/gateway/{IPFS_path}`].
    ///
    /// [`/ipfs/gateway/{IPFS_path}`]: https://docs.blockfrost.io/#tag/IPFS-Gateway
    pub async fn gateway(&self, ipfs_path: &str) -> crate::Result<Vec<u8>> {
        let url = self.settings.network_address.clone()
            + &format!("/ipfs/gateway/{IPFS_path}", IPFS_path = ipfs_path);

        let response = self.client.get(&url).send().await?;
        let status_code = response.status();
        let bytes = response.bytes().await?;

        if !status_code.is_success() {
            // Safety:
            //   The error responses are guaranteed to be UTF-8.
            let text = std::str::from_utf8(&bytes).unwrap();
            return Err(process_error_response(text, status_code));
        }

        Ok(bytes.to_vec())
    }

    /// Pinned objects are counted in your user storage quota.
    ///
    /// OpenAPI endpoint reference: [`/ipfs/pin/add/{IPFS_path}`].
    ///
    /// [`/ipfs/pin/add/{IPFS_path}`]: https://docs.blockfrost.io/#tag/IPFS-Pins/paths/~1ipfs~1pin~1add~1{IPFS_path}/post
    pub async fn pin_add(&self, ipfs_path: &str) -> crate::Result<IpfsPinUpdate> {
        let url = self.settings.network_address.clone()
            + &format!("/ipfs/pin/add/{IPFS_path}", IPFS_path = ipfs_path);

        let response = self.client.post(&url).send().await?;
        let status_code = response.status();
        let text = response.text().await?;
        if !status_code.is_success() {
            return Err(process_error_response(&text, status_code));
        }

        Ok(serde_json::from_str(&text)?)
    }

    /// List objects pinned to local storage.
    ///
    /// OpenAPI endpoint reference: [`/ipfs/pin/list`].
    ///
    /// [`/ipfs/pin/list`]: https://docs.blockfrost.io/#tag/IPFS-Pins/paths/~1ipfs~1pin~1list~1/get
    pub async fn pin_list(&self) -> crate::Result<Vec<IpfsPinList>> {
        let url = self.settings.network_address.clone() + "/ipfs/pin/list";

        let response = self.client.get(&url).send().await?;
        let status_code = response.status();
        let text = response.text().await?;
        if !status_code.is_success() {
            return Err(process_error_response(&text, status_code));
        }

        Ok(serde_json::from_str(&text)?)
    }

    /// Get information about locally pinned IPFS object.
    ///
    /// OpenAPI endpoint reference: [`/ipfs/pin/list/{IPFS_path}`].
    ///
    /// [`/ipfs/pin/list/{IPFS_path}`]: https://docs.blockfrost.io/#tag/IPFS-Pins/paths/~1ipfs~1pin~1list~1{IPFS_path}/get
    pub async fn pin_list_by_id(&self, ipfs_path: &str) -> crate::Result<IpfsPinList> {
        let url = self.settings.network_address.clone()
            + &format!("/ipfs/pin/list/{IPFS_path}", IPFS_path = ipfs_path);

        let response = self.client.get(&url).send().await?;
        let status_code = response.status();
        let text = response.text().await?;
        if !status_code.is_success() {
            return Err(process_error_response(&text, status_code));
        }

        Ok(serde_json::from_str(&text)?)
    }

    /// Remove pinned objects from local storage.
    ///
    /// OpenAPI endpoint reference: [`/ipfs/pin/remove/{IPFS_path}`].
    ///
    /// [`/ipfs/pin/remove/{IPFS_path}`]: https://docs.blockfrost.io/#tag/IPFS-Pins/paths/~1ipfs~1pin~1remove~1{IPFS_path}/post
    pub async fn pin_remove(&self, ipfs_path: &str) -> crate::Result<IpfsPinUpdate> {
        let url = self.settings.network_address.clone()
            + &format!("/ipfs/pin/remove/{IPFS_path}", IPFS_path = ipfs_path);

        let response = self.client.post(&url).send().await?;
        let status_code = response.status();
        let text = response.text().await?;
        if !status_code.is_success() {
            return Err(process_error_response(&text, status_code));
        }

        Ok(serde_json::from_str(&text)?)
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

#[cfg(test)]
mod tests {
    use super::*;

    test_example! { test_ipfs_add, IpfsAdd, r#"
    {
      "name": "README.md",
      "ipfs_hash": "QmZbHqiCxKEVX7QfijzJTkZiSi3WEVTcvANgNAWzDYgZDr",
      "size": "125297"
    }
    "# }

    test_example! { test_ipfs_pin_add, IpfsPinUpdate, r#"
    {
      "ipfs_hash": "QmPojRfAXYAXV92Dof7gtSgaVuxEk64xx9CKvprqu9VwA8",
      "state": "queued"
    }
    "# }

    test_example! { test_ipfs_pin_list_by_id, IpfsPinList, r#"
    {
      "time_created": 1615551024,
      "time_pinned": 1615551024,
      "ipfs_hash": "QmdVMnULrY95mth2XkwjxDtMHvzuzmvUPTotKE1tgqKbCx",
      "size": "1615551024",
      "state": "pinned"
    }
    "# }
}
