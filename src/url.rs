use crate::{Pagination, CARDANO_MAINNET_URL, CARDANO_PREPROD_URL, CARDANO_PREVIEW_URL};
use std::error::Error;
use url::{form_urlencoded, Url as UrlI};

#[derive(Clone, Debug)]
pub struct Url(pub String);

impl Url {
    pub fn from_endpoint(base_url: &str, endpoint_url: &str) -> Result<String, Box<dyn Error>> {
        let url = Self::create_base_url(base_url, endpoint_url)?;

        Ok(url.to_string())
    }

    pub fn from_paginated_endpoint(
        base_url: &str,
        endpoint_url: &str,
        pagination: Pagination,
    ) -> Result<String, Box<dyn Error>> {
        let mut url = Self::create_base_url(base_url, endpoint_url)?;

        let mut query_pairs = form_urlencoded::Serializer::new(String::new());

        query_pairs.append_pair("page", pagination.page.to_string().as_str());
        query_pairs.append_pair("count", pagination.count.to_string().as_str());
        query_pairs.append_pair("count", pagination.count.to_string().as_str());

        let query = query_pairs.finish();

        url.set_query(Some(&query));

        Ok(url.to_string())
    }

    pub fn get_base_url_from_project_id(project_id: &str) -> String {
        match project_id {
            id if id.starts_with("mainnet") => CARDANO_MAINNET_URL,
            id if id.starts_with("preview") => CARDANO_PREVIEW_URL,
            id if id.starts_with("preprod") => CARDANO_PREPROD_URL,
            _ => CARDANO_MAINNET_URL,
        }
        .to_string()
    }

    fn create_base_url(base_url: &str, endpoint_url: &str) -> Result<reqwest::Url, Box<dyn Error>> {
        let mut url = UrlI::parse(base_url)?;
        let endpoint = endpoint_url.strip_prefix('/').unwrap_or(endpoint_url);

        if !url.path().ends_with('/') {
            url.set_path(&format!("{}/", url.path()));
        }

        Ok(url.join(endpoint)?)
    }
}
