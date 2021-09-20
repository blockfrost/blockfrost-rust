use serde_json::Value as JsonValue;

pub(crate) fn try_formatting_json(text: &str) -> serde_json::Result<String> {
    let json = serde_json::from_str::<JsonValue>(text)?;
    serde_json::to_string_pretty(&json)
}
