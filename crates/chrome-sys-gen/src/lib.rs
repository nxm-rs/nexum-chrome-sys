//! Chrome Extension API bindings generator.
//!
//! This crate reads the chrome-api.json file produced by chrome-types and
//! generates Rust bindings using wasm-bindgen.

pub mod schema;

use anyhow::{Context, Result};
use schema::ProcessedApiData;
use std::path::Path;

/// Load and parse the chrome-api.json file.
pub fn load_api_data(path: impl AsRef<Path>) -> Result<ProcessedApiData> {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;

    serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_chrome_api() {
        // This test requires the chrome-api.json file to exist
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../chrome-api.json");
        if std::path::Path::new(path).exists() {
            let data = load_api_data(path).expect("Failed to load chrome-api.json");
            assert!(!data.api.is_empty(), "API should have namespaces");
            assert!(data.api.contains_key("tabs"), "Should have tabs API");
            assert!(data.api.contains_key("runtime"), "Should have runtime API");
        }
    }
}
