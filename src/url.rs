use crate::Pagination;
use std::error::Error;
use url::{form_urlencoded, Url as UrlI};

#[derive(Clone, Debug)]
pub(crate) struct Url(pub String);

impl Url {
    pub fn from_endpoint(base_url: String, endpoint_url: &str) -> String {
        base_url + endpoint_url
    }

    pub fn from_paginated_endpoint(
        base_url: String,
        endpoint_url: &str,
        pagination: Pagination,
    ) -> Result<String, Box<dyn Error>> {
        let mut url = UrlI::parse(base_url.as_str())?;

        url.join(endpoint_url)?;

        let mut query_pairs = form_urlencoded::Serializer::new(String::new());

        query_pairs.append_pair("page", pagination.page.to_string().as_str());
        query_pairs.append_pair("count", pagination.count.to_string().as_str());
        query_pairs.append_pair("count", pagination.count.to_string().as_str());

        let query = query_pairs.finish();

        url.set_query(Some(&query));

        Ok(url.to_string())
    }
}

// fn create_query_parameters_suffix(parameters: &QueryParameters, page: Option<u32>) -> String {
//     fn append_parameter(string: &mut String, parameter_name: &str, parameter: impl AsRef<str>) {
//         if string.is_empty() {
//             // First parameter comes after a question mark
//             string.push('?');
//         } else {
//             // Separator between parameters
//             string.push('&');
//         }
//         string.push_str(parameter_name);
//         string.push('=');
//         string.push_str(parameter.as_ref());
//     }

//     let QueryParameters {
//         count,
//         order,
//         from,
//         to,
//         ..
//     } = &parameters;
//     let mut string = String::new();

//     if let Some(count) = count {
//         append_parameter(&mut string, "count", count.to_string());
//     }
//     if let Some(order) = order {
//         append_parameter(&mut string, "order", order.to_string());
//     }
//     if let Some(from) = from {
//         append_parameter(&mut string, "from", from);
//     }
//     if let Some(to) = to {
//         append_parameter(&mut string, "to", to);
//     }
//     if let Some(page) = page {
//         append_parameter(&mut string, "page", page.to_string());
//     }

//     string
// }
