//! Namespace code generator - web-sys style.
//!
//! Generates wasm-bindgen bindings following the web-sys pattern:
//! - Enums use string discriminants with #[wasm_bindgen]
//! - Dictionaries are opaque JS Object wrappers with getter/setter methods
//! - No serde - data stays in JS

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

use super::functions::{GenerateEvent, GenerateFunction};
use super::types::{GenContext, generate_type};
use crate::schema::NamespaceSpec;

/// Generate Rust code for a namespace.
pub fn generate(ns_name: &str, ns_spec: &NamespaceSpec) -> Result<String> {
    let ctx = GenContext::new(ns_name, ns_spec);
    let mut tokens = TokenStream::new();

    // File header
    tokens.extend(generate_header());

    // Generate types (enums and dictionaries)
    if let Some(types) = &ns_spec.types {
        for type_spec in types {
            if let Some(type_tokens) = generate_type(&ctx, type_spec) {
                tokens.extend(type_tokens);
            }
        }
    }

    // Generate extern "C" block for functions and events
    let extern_block = generate_extern_block(&ctx, ns_spec);
    tokens.extend(extern_block);

    // Format the output
    let file = syn::parse2::<syn::File>(tokens.clone());
    match file {
        Ok(file) => Ok(prettyplease::unparse(&file)),
        Err(e) => {
            eprintln!("Warning: syn parse error for {}: {}", ns_name, e);
            Ok(tokens.to_string())
        }
    }
}

/// Generate file header with imports.
fn generate_header() -> TokenStream {
    quote! {
        #![allow(unused_imports)]
        #![allow(clippy::all)]
        use wasm_bindgen::prelude::*;
        use js_sys::{Array, Function, Object, Promise};
    }
}

/// Generate the extern "C" block with wasm_bindgen bindings for functions and events.
fn generate_extern_block(ctx: &GenContext, ns_spec: &NamespaceSpec) -> TokenStream {
    let mut items = Vec::new();

    // Build js_namespace path for functions
    let js_namespace: Vec<&str> = std::iter::once("chrome")
        .chain(ctx.ns_name.split('.'))
        .collect();

    // Generate function bindings
    if let Some(functions) = &ns_spec.functions {
        for func in functions {
            if let Some(func_tokens) = func.generate(ctx, &js_namespace) {
                items.push(func_tokens);
            }
        }
    }

    // Generate event bindings
    if let Some(events) = &ns_spec.events {
        for event in events {
            if let Some(event_tokens) = event.generate(&js_namespace) {
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
