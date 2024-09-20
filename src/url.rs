use crate::{
    pagination::Pagination, CARDANO_MAINNET_URL, CARDANO_PREPROD_URL, CARDANO_PREVIEW_URL,
};
use std::error::Error;
use url::{form_urlencoded, Url as UrlI};

#[derive(Clone, Debug)]
pub struct Url;

impl Url {
    pub fn from_endpoint(base_url: &str, endpoint_url: &str) -> Result<String, Box<dyn Error>> {
        let url = Self::create_base_url(base_url, endpoint_url)?;

        Ok(url.to_string())
    }

    pub fn from_paginated_endpoint(
        base_url: &str, endpoint_url: &str, pagination: Pagination,
    ) -> Result<String, Box<dyn Error>> {
        let mut url = Self::create_base_url(base_url, endpoint_url)?;
        let mut query_pairs = form_urlencoded::Serializer::new(String::new());

        query_pairs.append_pair("page", pagination.page.to_string().as_str());
        query_pairs.append_pair("count", pagination.count.to_string().as_str());
        query_pairs.append_pair("order", pagination.order_to_string().as_str());

        let query = query_pairs.finish();

        url.set_query(Some(&query));

        Ok(url.to_string())
    }

    pub fn generate_batch(
        url: &str, batch_size: usize, start: usize, pagination: Pagination,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let mut result = Vec::new();
        let mut url = UrlI::parse(url)?;

        for page in start..batch_size {
            let mut query_pairs = form_urlencoded::Serializer::new(String::new());

            query_pairs.append_pair("page", page.to_string().as_str());
            query_pairs.append_pair("count", pagination.count.to_string().as_str());
            query_pairs.append_pair("order", pagination.order_to_string().as_str());

            let query = query_pairs.finish();

            url.set_query(Some(&query));

            result.push(url.to_string());
        }

        Ok(result)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_endpoint() {
        let base_url = "http://example.com";
        let endpoint_url = "api/data";
        let expected_url = "http://example.com/api/data";

        let result = Url::from_endpoint(base_url, endpoint_url).unwrap();
        assert_eq!(result, expected_url);
    }
}
