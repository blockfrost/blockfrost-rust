use reqwest::{
    multipart::{Form, Part},
    Client,
};
use serde::{Deserialize, Serialize};

use crate::{error::process_error_response, utils::build_header_map, IpfsSettings};

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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IpfsAdd {
    /// Name of the file.
    pub name: String,
    /// IPFS hash of the file.
    pub ipfs_hash: String,
    /// IPFS node size in Bytes.
    pub size: String,
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
}
