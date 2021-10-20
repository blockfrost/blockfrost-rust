// Useful for testing our types against JSON examples in the documentation.
macro_rules! test_example {
    { $test_name:tt, $schema_name:ty, $text:expr } => {
        #[test]
        fn $test_name() {
            let parsed = serde_json::from_str::<$schema_name>($text);
            // Expected not to crash
            parsed.unwrap();
        }
    };
}
