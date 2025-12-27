//! Namespace code generator - web-sys style.
//!
//! Generates wasm-bindgen bindings following the web-sys pattern:
//! - Enums use string discriminants with #[wasm_bindgen]
//! - Dictionaries are opaque JS Object wrappers with getter/setter methods
//! - No serde - data stays in JS

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

use super::types::GenContext;
use crate::schema::NamespaceSpec;

impl NamespaceSpec {
    /// Generate Rust code for this namespace.
    pub fn generate_code(&self) -> Result<String> {
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
