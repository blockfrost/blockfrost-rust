use serde::{Deserialize, Serialize};

macro_rules! test_schema {
    { $test_name:tt, $schema_name:tt, $text:expr } => {
        #[test]
        fn $test_name() {
            let parsed = serde_json::from_str::<$schema_name>($text);
            assert!(parsed.is_ok());
        }
    };
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Health {
    is_healthy: bool,
}

test_schema! { test_health, Health, r#"
{
  "is_healthy": true
}
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Root {
    url: String,
    version: String,
}

test_schema! { test_root, Root, r#"
{
  "url": "https://blockfrost.io/",
  "version": "0.1.0"
}
"# }
