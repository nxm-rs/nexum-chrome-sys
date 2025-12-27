//! Type generation for enums, dictionaries, and type aliases.

use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::utils::{clean_html, make_ident, make_type_ident, to_enum_variant};
use crate::schema::{EnumValue, PrimitiveType, TypeSpec};

/// Context for code generation within a namespace.
pub struct GenContext {
    /// The namespace name
    pub ns_name: String,
    /// Set of enum type IDs in this namespace (used to detect enum refs)
    enum_types: std::collections::HashSet<String>,
}

impl GenContext {
    pub fn new(ns_name: &str, ns_spec: &crate::schema::NamespaceSpec) -> Self {
        let mut enum_types = std::collections::HashSet::new();
        if let Some(types) = &ns_spec.types {
            for t in types {
                if let Some(id) = &t.id
                    && t.enum_.is_some()
                {
                    enum_types.insert(id.clone());
                }
            }
        }
        Self {
            ns_name: ns_name.to_string(),
            enum_types,
        }
    }

    /// Check if a type ID refers to an enum in this namespace
    pub fn is_enum(&self, type_id: &str) -> bool {
        self.enum_types.contains(type_id)
    }
}

/// Information about a mapped type, including any feature requirements.
pub struct TypeInfo {
    /// The Rust type tokens
    pub ty: TokenStream,
    /// Feature required for this type (for cross-namespace refs)
    pub feature: Option<String>,
    /// Whether this is a reference type (needs & in setter)
    pub is_ref_type: bool,
}

impl TypeInfo {
    pub fn simple(ty: TokenStream) -> Self {
        let is_ref = !is_pass_by_value(&ty);
        Self {
            ty,
            feature: None,
            is_ref_type: is_ref,
        }
    }

    pub fn with_feature(ty: TokenStream, feature: String, is_ref_type: bool) -> Self {
        Self {
            ty,
            feature: Some(feature),
            is_ref_type,
        }
    }
}

/// Check if a type should be passed by value in setters.
fn is_pass_by_value(ty: &TokenStream) -> bool {
    let s = ty.to_string();
    // Primitives and String are passed by value
    matches!(s.as_str(), "bool" | "i32" | "f64" | "String")
}

/// Map a TypeSpec to a JS-compatible Rust type with feature info.
pub fn map_type_to_js(ctx: &GenContext, type_spec: &TypeSpec) -> TypeInfo {
    // Check for $ref first
    if let Some(ref_name) = &type_spec.ref_ {
        if let Some((namespace, type_name)) = ref_name.split_once('.') {
            // Check if this is a self-reference (same namespace)
            if namespace == ctx.ns_name {
                // Self-reference: use the full ref name for the type ident
                // Type IDs like "declarativeWebRequest.RequestCookie" become
                // DeclarativeWebRequestRequestCookie when generated
                let type_ident = make_type_ident(&ref_name.to_upper_camel_case());
                // Check if it's an enum (enums are pass-by-value)
                let is_enum = ctx.is_enum(ref_name);
                return TypeInfo {
                    ty: quote! { #type_ident },
                    feature: None,
                    is_ref_type: !is_enum,
                };
            }
            // Cross-namespace reference: runtime.Port -> super::runtime::Port
            let feature = namespace.to_snake_case().replace('.', "_");
            let ns_ident = format_ident!("{}", feature);
            let type_ident = make_type_ident(&type_name.to_upper_camel_case());
            // Cross-namespace types are passed by value (enums are Copy, objects impl Clone)
            return TypeInfo::with_feature(
                quote! { super::#ns_ident::#type_ident },
                feature,
                false, // is_ref_type = false (pass by value)
            );
        } else {
            // Local reference (no namespace prefix)
            let type_ident = make_type_ident(&ref_name.to_upper_camel_case());
            // Check if it's an enum (enums are pass-by-value)
            let is_enum = ctx.is_enum(ref_name);
            return TypeInfo {
                ty: quote! { #type_ident },
                feature: None,
                is_ref_type: !is_enum,
            };
        }
    }

    // Check for choices (union types)
    if type_spec.choices.is_some() {
        return TypeInfo::simple(quote! { JsValue });
    }

    // Map primitive types
    let ty = match type_spec.type_.as_ref() {
        Some(PrimitiveType::Boolean) => quote! { bool },
        Some(PrimitiveType::Integer) => quote! { i32 },
        Some(PrimitiveType::Number) | Some(PrimitiveType::Double) => quote! { f64 },
        Some(PrimitiveType::String) => quote! { String },
        Some(PrimitiveType::Array) => quote! { Array },
        Some(PrimitiveType::Object) => quote! { Object },
        Some(PrimitiveType::Function) => quote! { Function },
        Some(PrimitiveType::Binary) => quote! { ::js_sys::ArrayBuffer },
        Some(PrimitiveType::Any) => quote! { JsValue },
        _ => quote! { JsValue },
    };
    TypeInfo::simple(ty)
}

/// Generate a type definition (enum or dictionary).
pub fn generate_type(ctx: &GenContext, type_spec: &TypeSpec) -> Option<TokenStream> {
    let id = type_spec.id.as_ref()?;

    // Skip internal types
    if id.starts_with('_') {
        return None;
    }

    // Check if it's an enum
    if let Some(enum_values) = &type_spec.enum_ {
        return Some(generate_enum(id, enum_values, type_spec));
    }

    // Check if it's an object type (dictionary)
    if type_spec.type_.as_ref() == Some(&PrimitiveType::Object) {
        return Some(generate_dictionary(ctx, id, type_spec));
    }

    // Check if it's an array type (generate type alias)
    if type_spec.type_.as_ref() == Some(&PrimitiveType::Array) {
        return Some(generate_array_alias(id, type_spec));
    }

    None
}

/// Generate a type alias for an array type.
fn generate_array_alias(id: &str, type_spec: &TypeSpec) -> TokenStream {
    let type_name = make_type_ident(&id.to_upper_camel_case());
    let doc = type_spec
        .description
        .as_deref()
        .map(clean_html)
        .unwrap_or_default();

    quote! {
        #[doc = #doc]
        pub type #type_name = Array;
    }
}

/// Generate an enum type (web-sys style with string discriminants).
fn generate_enum(id: &str, values: &[EnumValue], type_spec: &TypeSpec) -> TokenStream {
    let enum_name = make_type_ident(&id.to_upper_camel_case());

    let doc = type_spec
        .description
        .as_deref()
        .map(clean_html)
        .unwrap_or_default();

    let variants: Vec<TokenStream> = values
        .iter()
        .filter_map(|v| {
            let (name, desc) = match v {
                EnumValue::String(s) => (s.clone(), None),
                EnumValue::Number(_) => return None, // Skip numeric enums
                EnumValue::Named(n) => (n.name.clone(), n.description.clone()),
            };

            let variant_name = to_enum_variant(&name);
            let variant_ident = format_ident!("{}", variant_name);
            let js_name = &name;

            let doc_attr = desc.map(|d| {
                let d = clean_html(&d);
                quote! { #[doc = #d] }
            });

            // web-sys style: VariantName = "js_value"
            Some(quote! {
                #doc_attr
                #variant_ident = #js_name
            })
        })
        .collect();

    if variants.is_empty() {
        return TokenStream::new();
    }

    quote! {
        #[wasm_bindgen]
        #[doc = #doc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum #enum_name {
            #(#variants),*
        }
    }
}

/// Generate a dictionary type (web-sys style with opaque JS Object wrapper).
fn generate_dictionary(ctx: &GenContext, id: &str, type_spec: &TypeSpec) -> TokenStream {
    let type_name = make_type_ident(&id.to_upper_camel_case());
    let type_name_str = id.to_upper_camel_case();

    let doc = type_spec
        .description
        .as_deref()
        .map(clean_html)
        .unwrap_or_default();

    let properties = type_spec.properties.as_ref();

    // Collect all property names to detect collisions
    // A collision occurs when property "setValue" (snake: set_value) collides with
    // the setter for property "value" (set_value)
    let prop_names: std::collections::HashSet<String> = properties
        .map(|props| props.keys().map(|k| k.to_snake_case()).collect())
        .unwrap_or_default();

    // Generate getters and setters for each property (sorted for deterministic output)
    let accessors: Vec<TokenStream> = properties
        .map(|props| {
            let mut sorted: Vec<_> = props.iter().collect();
            sorted.sort_by_key(|(k, _)| *k);
            sorted
                .into_iter()
                .flat_map(|(name, prop)| {
                    let js_name = name;
                    let rust_getter = format_ident!("get_{}", name.to_snake_case());
                    let rust_setter = format_ident!("set_{}", name.to_snake_case());
                    let type_info = map_type_to_js(ctx, prop);
                    let is_optional = prop.optional.unwrap_or(false);

                    let getter_doc = format!("Get the `{}` field of this object.", name);
                    let setter_doc = format!("Change the `{}` field of this object.", name);

                    let field_type = &type_info.ty;
                    let return_type = if is_optional {
                        quote! { Option<#field_type> }
                    } else {
                        field_type.clone()
                    };

                    // For setters, we take the value by reference for complex types
                    let setter_param_type = if type_info.is_ref_type {
                        quote! { &#field_type }
                    } else {
                        field_type.clone()
                    };

                    // Add feature gate if needed
                    let feature_gate = type_info.feature.as_ref().map(|f| {
                        quote! { #[cfg(feature = #f)] }
                    });

                    vec![
                        // Getter
                        quote! {
                            #feature_gate
                            #[doc = #getter_doc]
                            #[wasm_bindgen(method, getter = #js_name)]
                            pub fn #rust_getter(this: &#type_name) -> #return_type;
                        },
                        // Setter
                        quote! {
                            #feature_gate
                            #[doc = #setter_doc]
                            #[wasm_bindgen(method, setter = #js_name)]
                            pub fn #rust_setter(this: &#type_name, val: #setter_param_type);
                        },
                    ]
                })
                .collect()
        })
        .unwrap_or_default();

    // Generate builder methods for the impl block (sorted for deterministic output)
    // Skip methods that would collide with getters/setters of other properties
    let builder_methods: Vec<TokenStream> = properties
        .map(|props| {
            let mut sorted: Vec<_> = props.iter().collect();
            sorted.sort_by_key(|(k, _)| *k);
            sorted
                .into_iter()
                .filter_map(|(name, prop)| {
                    let snake_name = name.to_snake_case();

                    // Check for collision: if this property's builder method name
                    // matches set_<other> or get_<other> for another property
                    let would_collide = if let Some(suffix) = snake_name.strip_prefix("set_") {
                        prop_names.contains(suffix)
                    } else if let Some(suffix) = snake_name.strip_prefix("get_") {
                        prop_names.contains(suffix)
                    } else {
                        false
                    };

                    if would_collide {
                        return None; // Skip this builder method
                    }

                    let method_name = make_ident(&snake_name);
                    let setter_name = format_ident!("set_{}", snake_name);
                    let type_info = map_type_to_js(ctx, prop);

                    let deprecated_msg = format!("Use `set_{}()` instead.", snake_name);

                    let field_type = &type_info.ty;
                    let param_type = if type_info.is_ref_type {
                        quote! { &#field_type }
                    } else {
                        field_type.clone()
                    };

                    // Add feature gate if needed
                    let feature_gate = type_info.feature.as_ref().map(|f| {
                        quote! { #[cfg(feature = #f)] }
                    });

                    Some(quote! {
                        #feature_gate
                        #[deprecated = #deprecated_msg]
                        pub fn #method_name(&mut self, val: #param_type) -> &mut Self {
                            self.#setter_name(val);
                            self
                        }
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let new_doc = format!("Construct a new `{}`.", type_name_str);

    quote! {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(extends = ::js_sys::Object, js_name = #type_name_str)]
            #[derive(Debug, Clone, PartialEq, Eq)]
            #[doc = #doc]
            pub type #type_name;

            #(#accessors)*
        }

        impl #type_name {
            #[doc = #new_doc]
            pub fn new() -> Self {
                #[allow(unused_mut)]
                let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
                ret
            }

            #(#builder_methods)*
        }

        impl Default for #type_name {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}
