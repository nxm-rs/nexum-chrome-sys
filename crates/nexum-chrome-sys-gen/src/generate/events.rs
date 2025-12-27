//! Event binding generation.

use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::utils::clean_html;
use crate::schema::EventSpec;

impl EventSpec {
    /// Generate a wasm_bindgen event listener binding.
    pub fn generate_event(&self, js_namespace: &[&str]) -> Option<TokenStream> {
        let name = &self.name;

        // Skip nodoc events
        if self.is_nodoc() {
            return None;
        }

        let doc = self
            .description
            .as_deref()
            .map(clean_html)
            .unwrap_or_default();

        // Event namespace includes the event name
        let event_ns: Vec<&str> = js_namespace
            .iter()
            .copied()
            .chain(std::iter::once(name.as_str()))
            .collect();

        let add_listener_name = format_ident!("{}_add_listener", name.to_snake_case());

        Some(quote! {
            #[doc = #doc]
            #[wasm_bindgen(js_namespace = [#(#event_ns),*], js_name = "addListener")]
            pub fn #add_listener_name(callback: &Function);
        })
    }
}
