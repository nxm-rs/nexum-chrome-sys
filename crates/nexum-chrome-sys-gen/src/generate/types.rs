//! Type generation for enums, dictionaries, and type aliases.

use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::utils::{clean_html, make_ident, make_type_ident, to_enum_variant};
use crate::schema::{EnumValue, EventSpec, PrimitiveType, TypeSpec};

/// Information about a serde-compatible type for companion struct generation.
pub struct SerdeTypeInfo {
    /// The Rust type tokens for the serde struct field
    pub ty: TokenStream,
    /// Whether this field should be wrapped in Option (serde skip_serializing_if)
    pub is_optional: bool,
    /// Whether this type can be converted from the wasm-bindgen type
    pub is_convertible: bool,
    /// Expression to convert from the wasm-bindgen getter
    pub conversion_expr: Option<TokenStream>,
}

/// Context for code generation within a namespace.
pub struct GenContext {
    /// The namespace name
    pub ns_name: String,
    /// Set of enum type IDs in this namespace (used to detect enum refs)
    enum_types: std::collections::HashSet<String>,
    /// Set of type IDs that will have serde companions generated
    serde_companion_types: std::collections::HashSet<String>,
}

impl GenContext {
    pub fn new(ns_name: &str, ns_spec: &crate::schema::NamespaceSpec) -> Self {
        let mut enum_types = std::collections::HashSet::new();
        let mut serde_companion_types = std::collections::HashSet::new();

        if let Some(types) = &ns_spec.types {
            for t in types {
                if let Some(id) = &t.id {
                    if t.enum_.is_some() {
                        enum_types.insert(id.clone());
                    }
                    // Track which types will get serde companions
                    if t.should_generate_serde_companion() {
                        // Check if the Data suffix would conflict with an existing type
                        let data_name = format!("{}Data", id.to_upper_camel_case());
                        let conflicts = types.iter().any(|other| {
                            other
                                .id
                                .as_ref()
                                .is_some_and(|other_id| other_id.to_upper_camel_case() == data_name)
                        });
                        if !conflicts {
                            serde_companion_types.insert(id.clone());
                        }
                    }
                }
            }
        }
        Self {
            ns_name: ns_name.to_string(),
            enum_types,
            serde_companion_types,
        }
    }

    /// Check if a type ID refers to an enum in this namespace.
    pub fn is_enum(&self, type_id: &str) -> bool {
        self.enum_types.contains(type_id)
    }

    /// Check if a type will have a serde companion struct generated.
    pub fn has_serde_companion(&self, type_id: &str) -> bool {
        self.serde_companion_types.contains(type_id)
    }

    /// Build the js_namespace path for wasm_bindgen (e.g., ["chrome", "tabs"]).
    pub fn js_namespace(&self) -> Vec<&str> {
        std::iter::once("chrome")
            .chain(self.ns_name.split('.'))
            .collect()
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
    fn simple(ty: TokenStream) -> Self {
        let is_ref = !is_pass_by_value(&ty);
        Self {
            ty,
            feature: None,
            is_ref_type: is_ref,
        }
    }

    fn with_feature(ty: TokenStream, feature: String, is_ref_type: bool) -> Self {
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
    matches!(s.as_str(), "bool" | "i32" | "f64" | "String")
}

impl TypeSpec {
    /// Map this type specification to a JS-compatible Rust type with feature info.
    pub fn to_type_info(&self, ctx: &GenContext) -> TypeInfo {
        // Check for $ref first
        if let Some(ref_name) = &self.ref_ {
            if let Some((namespace, type_name)) = ref_name.split_once('.') {
                // Check if this is a self-reference (same namespace)
                if namespace == ctx.ns_name {
                    let type_ident = make_type_ident(&ref_name.to_upper_camel_case());
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
                return TypeInfo::with_feature(
                    quote! { super::#ns_ident::#type_ident },
                    feature,
                    false,
                );
            } else {
                // Local reference (no namespace prefix)
                let type_ident = make_type_ident(&ref_name.to_upper_camel_case());
                let is_enum = ctx.is_enum(ref_name);
                return TypeInfo {
                    ty: quote! { #type_ident },
                    feature: None,
                    is_ref_type: !is_enum,
                };
            }
        }

        // Check for choices (union types)
        if self.choices.is_some() {
            return TypeInfo::simple(quote! { JsValue });
        }

        // Map primitive types
        let ty = match self.type_.as_ref() {
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

    /// Map this type specification to a serde-compatible Rust type.
    ///
    /// Returns None if the type cannot be represented as a serde type
    /// (e.g., Function, events, or complex JS-only types).
    ///
    /// `self_type_id` is the ID of the type being generated (for detecting self-references).
    pub fn to_serde_type_info(
        &self,
        ctx: &GenContext,
        getter_name: &syn::Ident,
        self_type_id: Option<&str>,
    ) -> Option<SerdeTypeInfo> {
        let is_optional = self.optional.unwrap_or(false);

        // Check for $ref first
        if let Some(ref_name) = &self.ref_ {
            if let Some((namespace, type_name)) = ref_name.split_once('.') {
                // Cross-namespace reference - skip for now (complex feature gating)
                if namespace != ctx.ns_name {
                    return None;
                }
                // Same namespace reference (with explicit namespace prefix)
                let is_enum = ctx.is_enum(type_name);

                if is_enum {
                    // Enums are Copy and can be used directly
                    let type_ident = make_type_ident(&type_name.to_upper_camel_case());
                    return Some(SerdeTypeInfo {
                        ty: quote! { #type_ident },
                        is_optional,
                        is_convertible: true,
                        conversion_expr: Some(quote! { val.#getter_name() }),
                    });
                } else {
                    // Check if the referenced type has a serde companion
                    if !ctx.has_serde_companion(type_name) {
                        return None; // Skip fields referencing types without serde companions
                    }
                    // Check for self-reference (recursive type) - needs Box
                    let is_self_ref = self_type_id.is_some_and(|id| id == type_name);
                    let data_type = format_ident!("{}Data", type_name.to_upper_camel_case());
                    let (ty, conversion) = if is_self_ref {
                        let conversion = if is_optional {
                            quote! { val.#getter_name().as_ref().map(|v| Box::new(v.into())) }
                        } else {
                            quote! { Box::new((&val.#getter_name()).into()) }
                        };
                        (quote! { Box<#data_type> }, conversion)
                    } else {
                        let conversion = if is_optional {
                            quote! { val.#getter_name().as_ref().map(|v| v.into()) }
                        } else {
                            quote! { (&val.#getter_name()).into() }
                        };
                        (quote! { #data_type }, conversion)
                    };
                    return Some(SerdeTypeInfo {
                        ty,
                        is_optional,
                        is_convertible: true,
                        conversion_expr: Some(conversion),
                    });
                }
            } else {
                // Local reference (no namespace prefix)
                let is_enum = ctx.is_enum(ref_name);
                if is_enum {
                    let type_ident = make_type_ident(&ref_name.to_upper_camel_case());
                    return Some(SerdeTypeInfo {
                        ty: quote! { #type_ident },
                        is_optional,
                        is_convertible: true,
                        conversion_expr: Some(quote! { val.#getter_name() }),
                    });
                } else {
                    // Check if the referenced type has a serde companion
                    if !ctx.has_serde_companion(ref_name) {
                        return None; // Skip fields referencing types without serde companions
                    }
                    // Check for self-reference (recursive type) - needs Box
                    let is_self_ref = self_type_id.is_some_and(|id| id == ref_name);
                    let data_type = format_ident!("{}Data", ref_name.to_upper_camel_case());
                    let (ty, conversion) = if is_self_ref {
                        let conversion = if is_optional {
                            quote! { val.#getter_name().as_ref().map(|v| Box::new(v.into())) }
                        } else {
                            quote! { Box::new((&val.#getter_name()).into()) }
                        };
                        (quote! { Box<#data_type> }, conversion)
                    } else {
                        let conversion = if is_optional {
                            quote! { val.#getter_name().as_ref().map(|v| v.into()) }
                        } else {
                            quote! { (&val.#getter_name()).into() }
                        };
                        (quote! { #data_type }, conversion)
                    };
                    return Some(SerdeTypeInfo {
                        ty,
                        is_optional,
                        is_convertible: true,
                        conversion_expr: Some(conversion),
                    });
                }
            }
        }

        // Check for choices (union types) - use JsValue with serde-wasm-bindgen
        if self.choices.is_some() {
            let conversion = if is_optional {
                quote! {
                    val.#getter_name().and_then(|v| serde_wasm_bindgen::from_value(v).ok())
                }
            } else {
                quote! {
                    serde_wasm_bindgen::from_value(val.#getter_name()).unwrap_or_default()
                }
            };
            return Some(SerdeTypeInfo {
                ty: quote! { serde_json::Value },
                is_optional,
                is_convertible: true,
                conversion_expr: Some(conversion),
            });
        }

        // Map primitive types
        match self.type_.as_ref() {
            Some(PrimitiveType::Boolean) => Some(SerdeTypeInfo {
                ty: quote! { bool },
                is_optional,
                is_convertible: true,
                conversion_expr: Some(quote! { val.#getter_name() }),
            }),
            Some(PrimitiveType::Integer) => Some(SerdeTypeInfo {
                ty: quote! { i32 },
                is_optional,
                is_convertible: true,
                conversion_expr: Some(quote! { val.#getter_name() }),
            }),
            Some(PrimitiveType::Number) | Some(PrimitiveType::Double) => Some(SerdeTypeInfo {
                ty: quote! { f64 },
                is_optional,
                is_convertible: true,
                conversion_expr: Some(quote! { val.#getter_name() }),
            }),
            Some(PrimitiveType::String) => Some(SerdeTypeInfo {
                ty: quote! { String },
                is_optional,
                is_convertible: true,
                conversion_expr: Some(quote! { val.#getter_name() }),
            }),
            Some(PrimitiveType::Array) => {
                // For arrays, try to get the item type
                if let Some(items) = &self.items {
                    let item_getter = format_ident!("item"); // placeholder
                    if let Some(item_info) = items.to_serde_type_info(ctx, &item_getter, None) {
                        let item_ty = &item_info.ty;
                        let conversion = if is_optional {
                            quote! {
                                val.#getter_name().map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default())
                            }
                        } else {
                            quote! {
                                serde_wasm_bindgen::from_value(val.#getter_name().into()).unwrap_or_default()
                            }
                        };
                        return Some(SerdeTypeInfo {
                            ty: quote! { Vec<#item_ty> },
                            is_optional,
                            is_convertible: true,
                            conversion_expr: Some(conversion),
                        });
                    }
                }
                // Fallback to generic Vec<serde_json::Value>
                let conversion = if is_optional {
                    quote! {
                        val.#getter_name().map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default())
                    }
                } else {
                    quote! {
                        serde_wasm_bindgen::from_value(val.#getter_name().into()).unwrap_or_default()
                    }
                };
                Some(SerdeTypeInfo {
                    ty: quote! { Vec<serde_json::Value> },
                    is_optional,
                    is_convertible: true,
                    conversion_expr: Some(conversion),
                })
            }
            Some(PrimitiveType::Object) => {
                // Generic object - use serde_json::Value
                let conversion = if is_optional {
                    quote! {
                        val.#getter_name().map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default())
                    }
                } else {
                    quote! {
                        serde_wasm_bindgen::from_value(val.#getter_name().into()).unwrap_or_default()
                    }
                };
                Some(SerdeTypeInfo {
                    ty: quote! { serde_json::Value },
                    is_optional,
                    is_convertible: true,
                    conversion_expr: Some(conversion),
                })
            }
            // Function and other non-serializable types
            Some(PrimitiveType::Function) => None,
            Some(PrimitiveType::Binary) => None,
            Some(PrimitiveType::Any) => {
                let conversion = if is_optional {
                    quote! {
                        val.#getter_name().and_then(|v| serde_wasm_bindgen::from_value(v).ok())
                    }
                } else {
                    quote! {
                        serde_wasm_bindgen::from_value(val.#getter_name()).unwrap_or_default()
                    }
                };
                Some(SerdeTypeInfo {
                    ty: quote! { serde_json::Value },
                    is_optional,
                    is_convertible: true,
                    conversion_expr: Some(conversion),
                })
            }
            _ => None,
        }
    }

    /// Check if this type should have a serde companion struct generated.
    ///
    /// Returns false for types that don't make sense as serializable data:
    /// - Types with events (like Port)
    /// - Types with only function properties
    pub fn should_generate_serde_companion(&self) -> bool {
        // Skip types with events - they're not pure data
        if self.events.as_ref().is_some_and(|e| !e.is_empty()) {
            return false;
        }

        // Must have properties to be useful as data
        let properties = match &self.properties {
            Some(p) if !p.is_empty() => p,
            _ => return false,
        };

        // Check if at least one property is not a function
        properties
            .values()
            .any(|p| p.type_.as_ref() != Some(&PrimitiveType::Function))
    }

    /// Generate a wasm_bindgen type definition (enum, dictionary, or alias).
    pub fn generate_type(&self, ctx: &GenContext) -> Option<TokenStream> {
        let id = self.id.as_ref()?;

        // Skip internal types
        if id.starts_with('_') {
            return None;
        }

        // Dispatch based on type kind
        if self.enum_.is_some() {
            return self.generate_enum_type();
        }

        if self.type_.as_ref() == Some(&PrimitiveType::Object) {
            return Some(self.generate_dictionary_type(ctx));
        }

        if self.type_.as_ref() == Some(&PrimitiveType::Array) {
            return Some(self.generate_array_type());
        }

        None
    }

    /// Generate a type alias for array types.
    fn generate_array_type(&self) -> TokenStream {
        let id = self.id.as_ref().expect("array type must have id");
        let type_name = make_type_ident(&id.to_upper_camel_case());
        let doc = self
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
    fn generate_enum_type(&self) -> Option<TokenStream> {
        let id = self.id.as_ref()?;
        let enum_values = self.enum_.as_ref()?;

        let enum_name = make_type_ident(&id.to_upper_camel_case());
        let doc = self
            .description
            .as_deref()
            .map(clean_html)
            .unwrap_or_default();

        let variants: Vec<TokenStream> = enum_values
            .iter()
            .filter_map(|v| {
                let (name, desc) = match v {
                    EnumValue::String(s) => (s.clone(), None),
                    EnumValue::Number(_) => return None,
                    EnumValue::Named(n) => (n.name.clone(), n.description.clone()),
                };

                let variant_name = to_enum_variant(&name);
                let variant_ident = format_ident!("{}", variant_name);
                let js_name = &name;

                let doc_attr = desc.map(|d| {
                    let d = clean_html(&d);
                    quote! { #[doc = #d] }
                });

                Some(quote! {
                    #doc_attr
                    #variant_ident = #js_name
                })
            })
            .collect();

        if variants.is_empty() {
            return None;
        }

        Some(quote! {
            #[wasm_bindgen]
            #[doc = #doc]
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum #enum_name {
                #(#variants),*
            }
        })
    }

    /// Generate a dictionary type (web-sys style with opaque JS Object wrapper).
    fn generate_dictionary_type(&self, ctx: &GenContext) -> TokenStream {
        let id = self.id.as_ref().expect("dictionary type must have id");
        let type_name = make_type_ident(&id.to_upper_camel_case());
        let type_name_str = id.to_upper_camel_case();

        let doc = self
            .description
            .as_deref()
            .map(clean_html)
            .unwrap_or_default();

        let properties = self.properties.as_ref();

        // Collect property names to detect naming collisions
        let prop_names: std::collections::HashSet<String> = properties
            .map(|props| props.keys().map(|k| k.to_snake_case()).collect())
            .unwrap_or_default();

        // Helper to iterate properties in sorted order for deterministic output
        let sorted_props = || {
            properties.into_iter().flat_map(|props| {
                let mut sorted: Vec<_> = props.iter().collect();
                sorted.sort_by_key(|(k, _)| *k);
                sorted
            })
        };

        let accessors: Vec<TokenStream> = sorted_props()
            .flat_map(|(name, prop)| self.generate_property_accessors(ctx, &type_name, name, prop))
            .collect();

        // Generate event accessors for events defined on this type (e.g., Port.onMessage)
        let event_accessors: Vec<TokenStream> = self
            .events
            .iter()
            .flatten()
            .map(|event| Self::generate_event_accessor(&type_name, event))
            .collect();

        let builder_methods: Vec<TokenStream> = sorted_props()
            .filter_map(|(name, prop)| self.generate_builder_method(ctx, &prop_names, name, prop))
            .collect();

        let new_doc = format!("Construct a new `{}`.", type_name_str);

        // Generate serde companion type if applicable (uses pre-computed set that accounts for conflicts)
        let serde_companion = if ctx.has_serde_companion(id) {
            self.generate_serde_companion_type(ctx, id, &type_name, &doc)
        } else {
            quote! {}
        };

        quote! {
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(extends = ::js_sys::Object, js_name = #type_name_str)]
                #[derive(Debug, Clone, PartialEq, Eq)]
                #[doc = #doc]
                pub type #type_name;

                #(#accessors)*

                #(#event_accessors)*
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

            #serde_companion
        }
    }

    /// Generate a serde-friendly companion struct for a dictionary type.
    fn generate_serde_companion_type(
        &self,
        ctx: &GenContext,
        type_id: &str,
        type_name: &syn::Ident,
        doc: &str,
    ) -> TokenStream {
        let data_type_name = format_ident!("{}Data", type_name);
        let data_doc = if doc.is_empty() {
            format!("Serializable data for `{}`.", type_name)
        } else {
            format!("Serializable data for `{}`. {}", type_name, doc)
        };

        let properties = match &self.properties {
            Some(p) => p,
            None => return quote! {},
        };

        // Sort properties for deterministic output
        let mut sorted_props: Vec<_> = properties.iter().collect();
        sorted_props.sort_by_key(|(k, _)| *k);

        // Collect field definitions and conversion expressions
        let mut fields = Vec::new();
        let mut conversions = Vec::new();

        for (name, prop) in &sorted_props {
            let snake_name = name.to_snake_case();
            let field_name = make_ident(&snake_name);
            let getter_name = format_ident!("get_{}", snake_name);

            // Get serde type info (pass type_id for detecting self-references)
            let serde_info = match prop.to_serde_type_info(ctx, &getter_name, Some(type_id)) {
                Some(info) => info,
                None => continue, // Skip non-serializable fields
            };

            let field_type = &serde_info.ty;
            let conversion_expr = serde_info.conversion_expr.as_ref();

            // Field documentation
            let field_doc = prop
                .description
                .as_deref()
                .map(clean_html)
                .unwrap_or_default();

            if serde_info.is_optional {
                fields.push(quote! {
                    #[doc = #field_doc]
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub #field_name: Option<#field_type>
                });
            } else {
                fields.push(quote! {
                    #[doc = #field_doc]
                    pub #field_name: #field_type
                });
            }

            if let Some(expr) = conversion_expr {
                conversions.push(quote! {
                    #field_name: #expr
                });
            }
        }

        if fields.is_empty() {
            return quote! {};
        }

        quote! {
            #[cfg(feature = "serde")]
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            #[doc = #data_doc]
            pub struct #data_type_name {
                #(#fields),*
            }

            #[cfg(feature = "serde")]
            impl From<&#type_name> for #data_type_name {
                fn from(val: &#type_name) -> Self {
                    Self {
                        #(#conversions),*
                    }
                }
            }
        }
    }

    /// Generate getter and setter for a dictionary property.
    fn generate_property_accessors(
        &self,
        ctx: &GenContext,
        type_name: &syn::Ident,
        name: &str,
        prop: &TypeSpec,
    ) -> Vec<TokenStream> {
        let js_name = name;
        let rust_getter = format_ident!("get_{}", name.to_snake_case());
        let rust_setter = format_ident!("set_{}", name.to_snake_case());
        let type_info = prop.to_type_info(ctx);
        let is_optional = prop.optional.unwrap_or(false);

        let getter_doc = format!("Get the `{}` field of this object.", name);
        let setter_doc = format!("Change the `{}` field of this object.", name);

        let field_type = &type_info.ty;
        let return_type = if is_optional {
            quote! { Option<#field_type> }
        } else {
            field_type.clone()
        };

        let setter_param_type = if type_info.is_ref_type {
            quote! { &#field_type }
        } else {
            field_type.clone()
        };

        let feature_gate = type_info.feature.as_ref().map(|f| {
            quote! { #[cfg(feature = #f)] }
        });

        vec![
            quote! {
                #feature_gate
                #[doc = #getter_doc]
                #[wasm_bindgen(method, getter = #js_name)]
                pub fn #rust_getter(this: &#type_name) -> #return_type;
            },
            quote! {
                #feature_gate
                #[doc = #setter_doc]
                #[wasm_bindgen(method, setter = #js_name)]
                pub fn #rust_setter(this: &#type_name, val: #setter_param_type);
            },
        ]
    }

    /// Generate a builder method for a dictionary property.
    fn generate_builder_method(
        &self,
        ctx: &GenContext,
        prop_names: &std::collections::HashSet<String>,
        name: &str,
        prop: &TypeSpec,
    ) -> Option<TokenStream> {
        let snake_name = name.to_snake_case();

        // Check for collision with getter/setter of another property
        let would_collide = if let Some(suffix) = snake_name.strip_prefix("set_") {
            prop_names.contains(suffix)
        } else if let Some(suffix) = snake_name.strip_prefix("get_") {
            prop_names.contains(suffix)
        } else {
            false
        };

        if would_collide {
            return None;
        }

        let method_name = make_ident(&snake_name);
        let setter_name = format_ident!("set_{}", snake_name);
        let type_info = prop.to_type_info(ctx);

        let deprecated_msg = format!("Use `set_{}()` instead.", snake_name);
        let field_type = &type_info.ty;
        let param_type = if type_info.is_ref_type {
            quote! { &#field_type }
        } else {
            field_type.clone()
        };

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
    }

    /// Generate a getter for an event property on a dictionary type.
    ///
    /// Events on types (like `Port.onMessage`) are Chrome Event objects with
    /// `addListener`, `removeListener`, etc. methods.
    fn generate_event_accessor(type_name: &syn::Ident, event: &EventSpec) -> TokenStream {
        let js_name = &event.name;
        let rust_getter = format_ident!("{}", event.name.to_snake_case());

        let doc = event
            .description
            .as_deref()
            .map(clean_html)
            .unwrap_or_default();

        // Return JsValue since Chrome Events are opaque objects with addListener/removeListener
        quote! {
            #[doc = #doc]
            #[wasm_bindgen(method, getter = #js_name)]
            pub fn #rust_getter(this: &#type_name) -> JsValue;
        }
    }
}
