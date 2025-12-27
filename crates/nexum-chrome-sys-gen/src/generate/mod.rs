//! Code generation for Chrome Extension API bindings.

mod events;
mod functions;
mod namespace;
mod types;
mod utils;

use anyhow::{Context, Result};
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use crate::schema::ProcessedApiData;

/// Configuration for code generation.
pub struct Config {
    /// Output directory for generated files
    pub output_dir: PathBuf,
    /// Only generate these namespaces (if Some)
    pub only_namespaces: Option<Vec<String>>,
    /// Skip these namespaces
    pub skip_namespaces: Option<Vec<String>>,
}

/// Required dependencies for generated code.
const REQUIRED_DEPS: &[(&str, &str)] = &[
    ("js-sys", "js-sys.workspace = true"),
    ("wasm-bindgen", "wasm-bindgen.workspace = true"),
];

impl Config {
    /// Run the code generator with this configuration.
    pub fn run(&self, api_data: &ProcessedApiData) -> Result<()> {
        // Create output directory
        let features_dir = self.output_dir.join("src").join("features");
        fs::create_dir_all(&features_dir)
            .with_context(|| format!("Failed to create {}", features_dir.display()))?;

        // Collect and filter namespaces
        let namespaces = self.filter_namespaces(api_data);
        println!("Generating {} namespaces...", namespaces.len());

        // Track generated features
        let mut features: BTreeSet<String> = BTreeSet::new();

        // Generate each namespace
        for ns_name in &namespaces {
            let ns_spec = api_data.api.get(*ns_name).unwrap();

            // Check if namespace has any exportable content
            if !ns_spec.has_exportable_content() {
                println!("  Skipping empty: {}", ns_name);
                continue;
            }

            // Generate the namespace file
            let code = ns_spec.generate_code()?;

            // Convert namespace to snake_case filename
            let file_name = to_snake_case(ns_name);
            let file_path = features_dir.join(format!("gen_{}.rs", file_name));

            fs::write(&file_path, &code)
                .with_context(|| format!("Failed to write {}", file_path.display()))?;

            features.insert(file_name);
            println!("  Generated: {}", ns_name);
        }

        // Generate mod.rs
        self.write_mod_rs(&features_dir, &features)?;

        // Update Cargo.toml with features and dependencies
        self.update_cargo_toml(&features)?;

        Ok(())
    }

    /// Filter namespaces based on configuration.
    fn filter_namespaces<'a>(&self, api_data: &'a ProcessedApiData) -> Vec<&'a String> {
        let mut namespaces: Vec<_> = api_data.api.keys().collect();
        namespaces.sort();

        namespaces
            .into_iter()
            .filter(|ns| {
                // Skip internal namespaces
                if ns.contains("Internal") || ns.contains("Private") {
                    return false;
                }

                // Check only filter
                if let Some(only) = &self.only_namespaces
                    && !only.iter().any(|o| o == *ns)
                {
                    return false;
                }

                // Check skip filter
                if let Some(skip) = &self.skip_namespaces
                    && skip.iter().any(|s| s == *ns)
                {
                    return false;
                }

                true
            })
            .collect()
    }

    /// Write the mod.rs file.
    fn write_mod_rs(
        &self,
        features_dir: &std::path::Path,
        features: &BTreeSet<String>,
    ) -> Result<()> {
        let mod_content = generate_mod_rs(features);
        let mod_path = features_dir.join("mod.rs");
        fs::write(&mod_path, &mod_content)
            .with_context(|| format!("Failed to write {}", mod_path.display()))?;
        println!("Generated mod.rs with {} features", features.len());
        Ok(())
    }

    /// Update Cargo.toml with features and dependencies.
    fn update_cargo_toml(&self, features: &BTreeSet<String>) -> Result<()> {
        let cargo_path = self.output_dir.join("Cargo.toml");
        let content = fs::read_to_string(&cargo_path)
            .with_context(|| format!("Failed to read {}", cargo_path.display()))?;

        let new_content = rewrite_cargo_toml(&content, features);

        fs::write(&cargo_path, new_content)
            .with_context(|| format!("Failed to write {}", cargo_path.display()))?;

        println!("Updated Cargo.toml with {} features", features.len());
        Ok(())
    }
}

/// Convert a namespace name to snake_case.
fn to_snake_case(name: &str) -> String {
    use heck::ToSnakeCase;
    // Handle dotted namespaces like "app.window" -> "app_window"
    name.replace('.', "_").to_snake_case()
}

/// Generate mod.rs content with feature gates.
fn generate_mod_rs(features: &BTreeSet<String>) -> String {
    let mut content = String::new();
    content.push_str("//! Generated Chrome Extension API bindings.\n");
    content.push_str("//!\n");
    content.push_str("//! This module is auto-generated. Do not edit manually.\n");
    content.push_str("//!\n");
    content.push_str("//! Each API namespace is exposed as a separate module.\n");
    content.push_str("//! Use like: `nexum_chrome_sys::tabs::query(...)`\n\n");

    for feature in features {
        content.push_str(&format!("#[cfg(feature = \"{}\")]\n", feature));
        content.push_str(&format!("mod gen_{};\n", feature));
        content.push_str(&format!("#[cfg(feature = \"{}\")]\n", feature));
        content.push_str(&format!("pub mod {} {{\n", feature));
        content.push_str(&format!("    pub use super::gen_{}::*;\n", feature));
        content.push_str("}\n\n");
    }

    // Remove trailing newline to match rustfmt
    content.trim_end().to_string() + "\n"
}

/// Rewrite Cargo.toml content with generated features and required dependencies.
fn rewrite_cargo_toml(content: &str, features: &BTreeSet<String>) -> String {
    let mut new_content = String::new();
    let mut in_features = false;
    let mut in_dependencies = false;
    let mut features_written = false;
    let mut deps_updated = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Detect section headers
        if trimmed.starts_with('[') {
            // End previous section processing
            in_features = false;
            in_dependencies = false;

            // Check for [features] section
            if trimmed == "[features]" {
                in_features = true;
                new_content.push_str(&generate_features_section(features));
                features_written = true;
                continue;
            }

            // Check for [dependencies] section
            if trimmed == "[dependencies]" {
                in_dependencies = true;
                new_content.push_str(line);
                new_content.push('\n');
                for (_name, dep_line) in REQUIRED_DEPS {
                    new_content.push_str(dep_line);
                    new_content.push('\n');
                }
                deps_updated = true;
                continue;
            }
        }

        // Skip lines in [features] section (we replace it entirely)
        if in_features {
            continue;
        }

        // Skip existing dependency lines that we're replacing
        if in_dependencies {
            let is_our_dep = REQUIRED_DEPS
                .iter()
                .any(|(name, _)| trimmed.starts_with(name));
            if is_our_dep {
                continue;
            }
        }

        new_content.push_str(line);
        new_content.push('\n');
    }

    // If no [features] section existed, add one
    if !features_written {
        new_content.push('\n');
        new_content.push_str(&generate_features_section(features));
    }

    // If no [dependencies] section existed, add one
    if !deps_updated {
        new_content.push_str("\n[dependencies]\n");
        for (_, dep_line) in REQUIRED_DEPS {
            new_content.push_str(dep_line);
            new_content.push('\n');
        }
    }

    new_content
}

/// Generate the [features] section content.
fn generate_features_section(features: &BTreeSet<String>) -> String {
    let mut content = String::new();
    content.push_str("[features]\n");
    content.push_str("default = []\n");

    for feature in features {
        content.push_str(&format!("{} = []\n", feature));
    }

    content.push('\n');
    content
}
