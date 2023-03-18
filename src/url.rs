//! Internal type to help building the URLs for the requests.

use crate::{BlockFrostSettings, QueryParameters};

#[derive(Clone, Debug)]
pub(crate) struct Url(pub String);

impl Url {
    pub(crate) fn from_endpoint(settings: &BlockFrostSettings, endpoint_url: &str) -> Self {
        let page = settings.query_parameters.page;
        Self::from_endpoint_with_page(settings, endpoint_url, page)
    }

    // Enables us to overwrite/ignore the page argument in the api
    //
    // This is useful when using a lister that increments internally it's page value
    pub(crate) fn from_endpoint_with_page(
        settings: &BlockFrostSettings,
        endpoint_url: &str,
        page: Option<u32>,
    ) -> Self {
        // url := "https://cardano-mainnet.blockfrost.io/api/v0" + "/blocks" + "?page=77&order=desc"
        let url = settings.network_address.clone()
            + endpoint_url
            + &create_query_parameters_suffix(&settings.query_parameters, page);
        Self(url)
    }

    pub(crate) fn from_endpoint_without_parameters(
        settings: &BlockFrostSettings,
        endpoint_url: &str,
    ) -> Self {
        // url := "https://cardano-mainnet.blockfrost.io/api/v0" + "/blocks"
        let url = settings.network_address.clone() + endpoint_url;
        Self(url)
    }
}

fn create_query_parameters_suffix(parameters: &QueryParameters, page: Option<u32>) -> String {
    fn append_parameter(string: &mut String, parameter_name: &str, parameter: impl AsRef<str>) {
        if string.is_empty() {
            // First parameter comes after a question mark
            string.push('?');
        } else {
            // Separator between parameters
            string.push('&');
        }
        string.push_str(parameter_name);
        string.push('=');
        string.push_str(parameter.as_ref());
    }

    let QueryParameters { count, order, from, to, .. } = &parameters;
    let mut string = String::new();

    if let Some(count) = count {
        append_parameter(&mut string, "count", count.to_string());
    }
    if let Some(order) = order {
        append_parameter(&mut string, "order", order.to_string());
    }
    if let Some(from) = from {
        append_parameter(&mut string, "from", from);
    }
    if let Some(to) = to {
        append_parameter(&mut string, "to", to);
    }
    if let Some(page) = page {
        append_parameter(&mut string, "page", page.to_string());
    }

    string
}
