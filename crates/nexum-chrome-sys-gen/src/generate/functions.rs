//! Function binding generation.

use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::quote;

use super::types::GenContext;
use super::utils::{clean_html, make_ident};
use crate::schema::TypeSpec;

impl TypeSpec {
    /// Generate a wasm_bindgen function binding.
    pub fn generate_function(
        &self,
        ctx: &GenContext,
        js_namespace: &[&str],
    ) -> Option<TokenStream> {
        let name = self.name.as_ref()?;

        // Skip nodoc functions
        if self.is_nodoc() {
            return None;
        }

        let rust_name = make_ident(&name.to_snake_case());
        let js_name = name;

        let doc = self
            .description
            .as_deref()
            .map(clean_html)
            .unwrap_or_default();

        // Collect all required features for this function
        let mut required_features: Vec<String> = Vec::new();

        // Generate parameters
        let params: Vec<TokenStream> = self
            .parameters
            .as_ref()
            .map(|params| {
                params
                    .iter()
                    .filter_map(|p| {
                        let param_name = p.name.as_ref()?;
                        let param_name = make_ident(&param_name.to_snake_case());
                        let type_info = p.to_type_info(ctx);
                        let is_optional = p.optional.unwrap_or(false);

                        // Collect feature requirement
                        if let Some(feature) = &type_info.feature
                            && !required_features.contains(feature)
                        {
                            required_features.push(feature.clone());
                        }

                        let param_type = &type_info.ty;
                        let ty = if is_optional {
                            quote! { Option<#param_type> }
                        } else {
                            param_type.clone()
                        };

                        Some(quote! { #param_name: #ty })
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Check for async returns (Promise support)
        let returns_promise = self.returns_async.is_some();
        let return_type = if returns_promise {
            quote! { -> Promise }
        } else if let Some(ret) = &self.returns {
            let type_info = ret.to_type_info(ctx);
            // Collect feature requirement from return type
            if let Some(feature) = &type_info.feature
                && !required_features.contains(feature)
            {
                required_features.push(feature.clone());
            }
            let ty = &type_info.ty;
            quote! { -> #ty }
        } else {
            TokenStream::new()
        };

        // Generate feature gate if any cross-namespace types are used
        let feature_gate = if required_features.is_empty() {
            None
        } else if required_features.len() == 1 {
            let f = &required_features[0];
            Some(quote! { #[cfg(feature = #f)] })
        } else {
            // Multiple features: require all of them
            let features: Vec<TokenStream> = required_features
                .iter()
                .map(|f| quote! { feature = #f })
                .collect();
            Some(quote! { #[cfg(all(#(#features),*))] })
        };

        Some(quote! {
            #feature_gate
            #[doc = #doc]
            #[wasm_bindgen(js_namespace = [#(#js_namespace),*], js_name = #js_name)]
            pub fn #rust_name(#(#params),*) #return_type;
        })
    }
}
