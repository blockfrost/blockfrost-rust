use crate::USER_AGENT;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};
use serde_json::{from_str as json_from, Value as JsonValue};
use std::{collections::HashMap, str::FromStr};

pub(crate) fn try_formatting_json(text: &str) -> serde_json::Result<String> {
    let json = json_from::<JsonValue>(text)?;

    serde_json::to_string_pretty(&json)
}

pub(crate) fn create_client_with_project_id(
    project_id: impl AsRef<str>, headers: &HashMap<String, String>,
) -> Client {
    let header_map = build_header_map(project_id.as_ref(), headers);
    // Safety: This unwrap is guaranteed to never fail if we only call .default_headers()
    Client::builder()
        .default_headers(header_map)
        .build()
        .unwrap()
}

pub(crate) fn build_header_map(project_id: &str, headers: &HashMap<String, String>) -> HeaderMap {
    let mut header_map = HeaderMap::new();
    let mut project_id = HeaderValue::from_str(project_id).unwrap_or_else(|_| {
        panic!(
            "Cannot create reqwest::Client because given project_id '{project_id}'cannot be parsed as HeaderValue",
        )
    });
    project_id.set_sensitive(true);
    let user_agent = HeaderValue::from_static(USER_AGENT);

    header_map.insert("project_id", project_id);
    header_map.insert("User-Agent", user_agent);
    for (key, val) in headers.iter() {
        let (Ok(key), Ok(val)) = (HeaderName::from_str(key), HeaderValue::from_str(val)) else {
            panic!("Header \"{key}: {val}\" is invalid")
        };
        header_map.insert(key, val);
    }
    header_map
}
