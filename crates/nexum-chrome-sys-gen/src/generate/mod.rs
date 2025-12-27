//! Code generation for Chrome Extension API bindings.

mod namespace;

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

/// Run the code generator.
pub fn run(api_data: &ProcessedApiData, config: &Config) -> Result<()> {
    // Create output directory
    let features_dir = config.output_dir.join("src").join("features");
    fs::create_dir_all(&features_dir)
        .with_context(|| format!("Failed to create {}", features_dir.display()))?;

    // Collect namespaces to generate
    let mut namespaces: Vec<_> = api_data.api.keys().collect();
    namespaces.sort();

    // Filter namespaces
    let namespaces: Vec<_> = namespaces
        .into_iter()
        .filter(|ns| {
            // Skip internal namespaces
            if ns.contains("Internal") || ns.contains("Private") {
                return false;
            }

            // Check only filter
            if let Some(only) = &config.only_namespaces
                && !only.iter().any(|o| o == *ns)
            {
                return false;
            }

            // Check skip filter
            if let Some(skip) = &config.skip_namespaces
                && skip.iter().any(|s| s == *ns)
            {
                return false;
            }

            true
        })
        .collect();

    println!("Generating {} namespaces...", namespaces.len());

    // Track generated features
    let mut features: BTreeSet<String> = BTreeSet::new();

    // Generate each namespace
    for ns_name in &namespaces {
        let ns_spec = api_data.api.get(*ns_name).unwrap();

        // Check if namespace has any exportable content
        let has_types = ns_spec.types.as_ref().is_some_and(|t| {
            t.iter()
                .any(|ts| ts.id.as_ref().is_some_and(|id| !id.starts_with('_')))
        });
        let has_functions = ns_spec.functions.as_ref().is_some_and(|f| !f.is_empty());
        let has_events = ns_spec.events.as_ref().is_some_and(|e| !e.is_empty());

        if !has_types && !has_functions && !has_events {
            println!("  Skipping empty: {}", ns_name);
            continue;
        }

        // Generate the namespace file
        let code = namespace::generate(ns_name, ns_spec)?;

        // Convert namespace to snake_case filename
        let file_name = to_snake_case(ns_name);
        let file_path = features_dir.join(format!("gen_{}.rs", file_name));

        fs::write(&file_path, &code)
            .with_context(|| format!("Failed to write {}", file_path.display()))?;

        features.insert(file_name);
        println!("  Generated: {}", ns_name);
    }

    // Generate mod.rs
    let mod_content = generate_mod_rs(&features);
    let mod_path = features_dir.join("mod.rs");
    fs::write(&mod_path, &mod_content)
        .with_context(|| format!("Failed to write {}", mod_path.display()))?;

    println!("Generated mod.rs with {} features", features.len());

    // Update Cargo.toml with features and dependencies
    let cargo_path = config.output_dir.join("Cargo.toml");
    update_cargo_toml(&cargo_path, &features)?;

    println!("Updated Cargo.toml with {} features", features.len());

    Ok(())
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

    content
}

/// Update Cargo.toml with generated features and required dependencies.
fn update_cargo_toml(cargo_path: &PathBuf, features: &BTreeSet<String>) -> Result<()> {
    let content = fs::read_to_string(cargo_path)
        .with_context(|| format!("Failed to read {}", cargo_path.display()))?;

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
            if in_features {
                in_features = false;
            }
            if in_dependencies {
                in_dependencies = false;
            }

            // Check for [features] section
            if trimmed == "[features]" {
                in_features = true;
                // Write our generated features section
                new_content.push_str(&generate_features_section(features));
                features_written = true;
                continue;
            }

            // Check for [dependencies] section
            if trimmed == "[dependencies]" {
                in_dependencies = true;
                new_content.push_str(line);
                new_content.push('\n');
                // Write required dependencies
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

    fs::write(cargo_path, new_content)
        .with_context(|| format!("Failed to write {}", cargo_path.display()))?;

    Ok(())
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
