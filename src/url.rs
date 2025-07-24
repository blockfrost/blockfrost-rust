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
        let url = UrlI::parse(url)?;

        for page in start..(start + batch_size) {
            let mut query_pairs = form_urlencoded::Serializer::new(String::new());

            query_pairs.append_pair("page", page.to_string().as_str());
            query_pairs.append_pair("count", pagination.count.to_string().as_str());
            query_pairs.append_pair("order", pagination.order_to_string().as_str());

            let query = query_pairs.finish();

            let mut url = url.clone();

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
    use crate::pagination::{Order, Pagination};
    use crate::{CARDANO_MAINNET_URL, CARDANO_PREPROD_URL, CARDANO_PREVIEW_URL};
    use rstest::rstest;

    #[rstest]
    #[case("http://example.com", "api/data", "http://example.com/api/data")]
    #[case("http://example.com/", "/api/data", "http://example.com/api/data")]
    #[case(
        "http://example.com/basepath",
        "endpoint",
        "http://example.com/basepath/endpoint"
    )]
    fn test_from_endpoint_success(
        #[case] base_url: &str, #[case] endpoint_url: &str, #[case] expected: &str,
    ) {
        let result = Url::from_endpoint(base_url, endpoint_url).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("not a url", "api", true)]
    #[case("http://example.com", "api", false)]
    fn test_from_endpoint_error(
        #[case] base_url: &str, #[case] endpoint_url: &str, #[case] should_err: bool,
    ) {
        let result = Url::from_endpoint(base_url, endpoint_url);

        assert_eq!(result.is_err(), should_err);
    }

    #[rstest]
    #[case(
        "http://example.com",
        "api/items",
        2,
        5,
        Order::Desc,
        "http://example.com/api/items?page=2&count=5&order=desc"
    )]
    #[case(
        "https://foo.bar",
        "data",
        1,
        10,
        Order::Asc,
        "https://foo.bar/data?page=1&count=10&order=asc"
    )]
    fn test_from_paginated_endpoint(
        #[case] base_url: &str, #[case] endpoint_url: &str, #[case] page: usize,
        #[case] count: usize, #[case] order: Order, #[case] expected: &str,
    ) {
        let pagination = Pagination {
            page,
            count,
            order,
            fetch_all: false,
        };
        let result = Url::from_paginated_endpoint(base_url, endpoint_url, pagination).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("http://example.com/api/data", 3, 1, 10, Order::Asc,
           vec![
               "http://example.com/api/data?page=1&count=10&order=asc",
               "http://example.com/api/data?page=2&count=10&order=asc",
               "http://example.com/api/data?page=3&count=10&order=asc",
           ])]
    fn test_generate_batch(
        #[case] base: &str, #[case] batch_size: usize, #[case] page_start: usize,
        #[case] count: usize, #[case] order: Order, #[case] expected: Vec<&str>,
    ) {
        let pagination = Pagination {
            page: 0,
            count,
            order,
            fetch_all: false,
        };
        let urls = Url::generate_batch(base, batch_size, page_start, pagination).unwrap();
        let expected: Vec<String> = expected.into_iter().map(String::from).collect();
        assert_eq!(urls, expected);
    }

    #[rstest]
    #[case(
            "http://example.com/api/data",
            0,
            1,
            100,
            Order::Asc,
            vec![]
        )]
    #[case(
            "http://example.com/api/data",
            2,
            10,
            50,
            Order::Desc,
            vec![
                "http://example.com/api/data?page=10&count=50&order=desc",
                "http://example.com/api/data?page=11&count=50&order=desc"
            ]
        )]
    #[case(
            "https://test.net/resources",
            3,
            5,
            25,
            Order::Asc,
            vec![
                "https://test.net/resources?page=5&count=25&order=asc",
                "https://test.net/resources?page=6&count=25&order=asc",
                "https://test.net/resources?page=7&count=25&order=asc"
            ]
        )]
    fn test_generate_batch_extended(
        #[case] base: &str, #[case] batch_size: usize, #[case] page_start: usize,
        #[case] count: usize, #[case] order: Order, #[case] expected: Vec<&str>,
    ) {
        let pagination = Pagination {
            page: 0,
            count,
            order,
            fetch_all: false,
        };
        let urls = Url::generate_batch(base, batch_size, page_start, pagination).unwrap();
        let expected: Vec<String> = expected.into_iter().map(String::from).collect();

        assert_eq!(
            urls, expected,
            "Failed for base: {base}, batch_size: {batch_size}, page_start: {page_start}",
        );
    }

    #[test]
    fn test_get_base_url_from_project_id() {
        let cases = vec![
            ("mainnet123", CARDANO_MAINNET_URL),
            ("previewABC", CARDANO_PREVIEW_URL),
            ("preprodXYZ", CARDANO_PREPROD_URL),
            ("unknown", CARDANO_MAINNET_URL),
        ];

        for (project_id, expected) in cases {
            let url = Url::get_base_url_from_project_id(project_id);
            assert_eq!(url, expected.to_string(), "for project_id {project_id}");
        }
    }
}
