//! Namespace code generator - web-sys style.
//!
//! Generates wasm-bindgen bindings following the web-sys pattern:
//! - Enums use string discriminants with #[wasm_bindgen]
//! - Dictionaries are opaque JS Object wrappers with getter/setter methods
//! - No serde - data stays in JS

use anyhow::Result;
use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::quote;

use super::types::GenContext;
use crate::schema::{NamespaceSpec, PrimitiveType, TypeSpec};

impl NamespaceSpec {
    /// Generate Rust code for this namespace.
    pub fn generate_code(&self) -> Result<String> {
        // Extract inline types from events and functions first
        let inline_types = self.extract_inline_types();

        let ctx = GenContext::new(&self.namespace, self);
        let mut tokens = TokenStream::new();

        // File header
        tokens.extend(generate_header());

        // Generate types (enums and dictionaries)
        if let Some(types) = &self.types {
            for type_spec in types {
                if let Some(type_tokens) = type_spec.generate_type(&ctx) {
                    tokens.extend(type_tokens);
                }
            }
        }

        // Generate inline types extracted from events and functions
        for type_spec in &inline_types {
            if let Some(type_tokens) = type_spec.generate_type(&ctx) {
                tokens.extend(type_tokens);
            }
        }

        // Generate extern "C" block for functions and events
        tokens.extend(self.generate_extern_block(&ctx));

        // Format the output with prettyplease, then rustfmt for final polish
        let file = syn::parse2::<syn::File>(tokens.clone());
        let formatted = match file {
            Ok(file) => prettyplease::unparse(&file),
            Err(e) => {
                eprintln!("Warning: syn parse error for {}: {}", self.namespace, e);
                tokens.to_string()
            }
        };

        // Run rustfmt for final formatting
        Ok(run_rustfmt(&formatted).unwrap_or(formatted))
    }

    /// Generate the extern "C" block with wasm_bindgen bindings.
    fn generate_extern_block(&self, ctx: &GenContext) -> TokenStream {
        let mut items = Vec::new();
        let js_namespace = ctx.js_namespace();

        // Generate function bindings
        if let Some(functions) = &self.functions {
            for func in functions {
                if let Some(func_tokens) = func.generate_function(ctx, &js_namespace) {
                    items.push(func_tokens);
                }
            }
        }

        // Generate event bindings
        if let Some(events) = &self.events {
            for event in events {
                if let Some(event_tokens) = event.generate_event(&js_namespace) {
                    items.push(event_tokens);
                }
            }
        }

        if items.is_empty() {
            return TokenStream::new();
        }

        quote! {
            #[wasm_bindgen]
            extern "C" {
                #(#items)*
            }
        }
    }

    /// Extract inline object types from event and function parameters.
    ///
    /// These are anonymous object types defined inline in parameters that
    /// should be generated as named dictionary types.
    fn extract_inline_types(&self) -> Vec<TypeSpec> {
        let mut inline_types = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();

        // Collect existing type IDs to avoid conflicts
        if let Some(types) = &self.types {
            for t in types {
                if let Some(id) = &t.id {
                    seen_ids.insert(id.clone());
                }
            }
        }

        // Extract from events
        if let Some(events) = &self.events {
            for event in events {
                if let Some(params) = &event.parameters {
                    for param in params {
                        if let Some(inline_type) =
                            Self::extract_inline_object(&event.name, param, &mut seen_ids)
                        {
                            inline_types.push(inline_type);
                        }
                    }
                }
            }
        }

        // Extract from functions
        if let Some(functions) = &self.functions {
            for func in functions {
                if let Some(func_name) = &func.name {
                    if let Some(params) = &func.parameters {
                        for param in params {
                            if let Some(inline_type) =
                                Self::extract_inline_object(func_name, param, &mut seen_ids)
                            {
                                inline_types.push(inline_type);
                            }
                        }
                    }
                }
            }
        }

        inline_types
    }

    /// Extract an inline object type from a parameter, if applicable.
    ///
    /// Returns a TypeSpec with a synthetic ID if the parameter is an inline object.
    fn extract_inline_object(
        parent_name: &str,
        param: &TypeSpec,
        seen_ids: &mut std::collections::HashSet<String>,
    ) -> Option<TypeSpec> {
        // Only extract inline objects (type: object with properties, no $ref)
        if param.type_.as_ref() != Some(&PrimitiveType::Object) {
            return None;
        }
        if param.ref_.is_some() {
            return None;
        }
        if param.properties.is_none() || param.properties.as_ref().is_some_and(|p| p.is_empty()) {
            return None;
        }

        // Generate a synthetic ID from parent name + param name
        let param_name = param.name.as_ref()?;
        let synthetic_id = format!(
            "{}{}",
            parent_name.to_upper_camel_case(),
            param_name.to_upper_camel_case()
        );

        // Avoid duplicates
        if seen_ids.contains(&synthetic_id) {
            return None;
        }
        seen_ids.insert(synthetic_id.clone());

        // Create a new TypeSpec with the synthetic ID
        Some(TypeSpec {
            id: Some(synthetic_id),
            type_: Some(PrimitiveType::Object),
            properties: param.properties.clone(),
            description: param.description.clone(),
            ..Default::default()
        })
    }
}

/// Generate file header with imports.
fn generate_header() -> TokenStream {
    // Imports are alphabetically ordered to match rustfmt
    quote! {
        #![allow(unused_imports)]
        #![allow(clippy::all)]
        use js_sys::{Array, Function, Object, Promise};
        use wasm_bindgen::prelude::*;
    }
}

/// Run rustfmt on the given source code.
fn run_rustfmt(source: &str) -> Option<String> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut child = Command::new("rustfmt")
        .args(["--edition", "2024", "--emit", "stdout"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;

    child.stdin.take()?.write_all(source.as_bytes()).ok()?;
    let output = child.wait_with_output().ok()?;

    if output.status.success() {
        String::from_utf8(output.stdout).ok()
    } else {
        None
    }
}
