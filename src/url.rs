//! Internal type to help building the URLs for the requests.

#[derive(Clone, Debug)]
pub(crate) struct Url(pub String);

impl Url {
    pub(crate) fn from_endpoint(base_url: String, endpoint_url: &str) -> String {
        base_url.clone() + endpoint_url
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
