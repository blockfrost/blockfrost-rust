use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value as JsonValue;

use crate::USER_AGENT;

pub(crate) fn try_formatting_json(text: &str) -> serde_json::Result<String> {
    let json = serde_json::from_str::<JsonValue>(text)?;
    serde_json::to_string_pretty(&json)
}

pub(crate) fn build_header_map(project_id: &str) -> HeaderMap {
    let mut header_map = HeaderMap::new();

    let mut project_id = HeaderValue::from_str(project_id).unwrap_or_else(|_| {
        panic!("Could not parse given project_id '{}' into HeaderValue", project_id)
    });
    project_id.set_sensitive(true);
    let user_agent = HeaderValue::from_static(USER_AGENT);

    header_map.insert("project_id", project_id);
    header_map.insert("User-Agent", user_agent);
    header_map
}
