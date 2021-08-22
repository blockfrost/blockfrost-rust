use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Health {
    is_healthy: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Root {
    url: String,
    version: String,
}
