//! Namespace code generator - web-sys style.
//!
//! Generates wasm-bindgen bindings following the web-sys pattern:
//! - Enums use string discriminants with #[wasm_bindgen]
//! - Dictionaries are opaque JS Object wrappers with getter/setter methods
//! - No serde - data stays in JS

use anyhow::Result;
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;

use crate::schema::{EnumValue, EventSpec, NamespaceSpec, PrimitiveType, TypeSpec};

/// Context for code generation within a namespace.
struct GenContext<'a> {
    /// The namespace name
    ns_name: &'a str,
    /// Set of enum type IDs in this namespace (used to detect enum refs)
    enum_types: HashSet<String>,
}

impl<'a> GenContext<'a> {
    fn new(ns_name: &'a str, ns_spec: &NamespaceSpec) -> Self {
        let mut enum_types = HashSet::new();
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
            ns_name,
            enum_types,
        }
    }

    /// Check if a type ID refers to an enum in this namespace
    fn is_enum(&self, type_id: &str) -> bool {
        self.enum_types.contains(type_id)
    }
}

/// Generate Rust code for a namespace.
pub fn generate(ns_name: &str, ns_spec: &NamespaceSpec) -> Result<String> {
    let ctx = GenContext::new(ns_name, ns_spec);
    let mut tokens = TokenStream::new();

    // File header
    tokens.extend(generate_header(ns_spec));

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
fn generate_header(_ns_spec: &NamespaceSpec) -> TokenStream {
    quote! {
        #![allow(unused_imports)]
        #![allow(clippy::all)]
        use wasm_bindgen::prelude::*;
        use js_sys::{Array, Function, Object, Promise};
    }
}

/// Generate a type definition (enum or dictionary).
fn generate_type(ctx: &GenContext, type_spec: &TypeSpec) -> Option<TokenStream> {
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

    // Generate getters and setters for each property
    let accessors: Vec<TokenStream> = properties
        .map(|props| {
            props
                .iter()
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

    // Generate builder methods for the impl block
    // Skip methods that would collide with getters/setters of other properties
    let builder_methods: Vec<TokenStream> = properties
        .map(|props| {
            props
                .iter()
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
            if let Some(func_tokens) = generate_function(ctx, &js_namespace, func) {
                items.push(func_tokens);
            }
        }
    }

    // Generate event bindings
    if let Some(events) = &ns_spec.events {
        for event in events {
            if let Some(event_tokens) = generate_event(&js_namespace, event) {
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

/// Generate a function binding.
fn generate_function(
    ctx: &GenContext,
    js_namespace: &[&str],
    func: &TypeSpec,
) -> Option<TokenStream> {
    let name = func.name.as_ref()?;

    // Skip nodoc functions
    if func.nodoc.as_ref().map(|n| n.is_true()).unwrap_or(false) {
        return None;
    }

    let rust_name = make_ident(&name.to_snake_case());
    let js_name = name;

    let doc = func
        .description
        .as_deref()
        .map(clean_html)
        .unwrap_or_default();

    // Collect all required features for this function
    let mut required_features: Vec<String> = Vec::new();

    // Generate parameters
    let params: Vec<TokenStream> = func
        .parameters
        .as_ref()
        .map(|params| {
            params
                .iter()
                .filter_map(|p| {
                    let param_name = p.name.as_ref()?;
                    let param_name = make_ident(&param_name.to_snake_case());
                    let type_info = map_type_to_js(ctx, p);
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
    let returns_promise = func.returns_async.is_some();
    let return_type = if returns_promise {
        quote! { -> Promise }
    } else if let Some(ret) = &func.returns {
        let type_info = map_type_to_js(ctx, ret);
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

/// Generate an event binding.
fn generate_event(js_namespace: &[&str], event: &EventSpec) -> Option<TokenStream> {
    let name = &event.name;

    // Skip nodoc events
    if event.nodoc.as_ref().map(|n| n.is_true()).unwrap_or(false) {
        return None;
    }

    let doc = event
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

/// Information about a mapped type, including any feature requirements.
struct TypeInfo {
    /// The Rust type tokens
    ty: TokenStream,
    /// Feature required for this type (for cross-namespace refs)
    feature: Option<String>,
    /// Whether this is a reference type (needs & in setter)
    is_ref_type: bool,
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

/// Map a TypeSpec to a JS-compatible Rust type with feature info.
fn map_type_to_js(ctx: &GenContext, type_spec: &TypeSpec) -> TypeInfo {
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

/// Check if a type should be passed by value in setters.
fn is_pass_by_value(ty: &TokenStream) -> bool {
    let s = ty.to_string();
    // Primitives and String are passed by value
    matches!(s.as_str(), "bool" | "i32" | "f64" | "String")
}

/// Convert a string to a valid Rust enum variant name.
fn to_enum_variant(s: &str) -> String {
    let s = s.replace(['-', '.'], "_");
    let result = s.to_upper_camel_case();

    // Prepend underscore if starts with a digit
    if result
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        format!("_{}", result)
    } else {
        result
    }
}

/// Rust keywords that need to be escaped with r# prefix or renamed.
const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];

/// Keywords that cannot be raw identifiers and need to be renamed.
const NON_RAW_KEYWORDS: &[&str] = &["self", "Self", "super", "crate"];

/// Create an identifier, using raw identifier syntax for Rust keywords.
fn make_ident(name: &str) -> proc_macro2::Ident {
    let name = if name
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        format!("_{}", name)
    } else {
        name.to_string()
    };

    if NON_RAW_KEYWORDS.contains(&name.as_str()) {
        format_ident!("{}_", name)
    } else if RUST_KEYWORDS.contains(&name.as_str()) {
        syn::Ident::new_raw(&name, proc_macro2::Span::call_site())
    } else {
        format_ident!("{}", name)
    }
}

/// Create an identifier for type names.
fn make_type_ident(name: &str) -> proc_macro2::Ident {
    let name = if name
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        format!("_{}", name)
    } else {
        name.to_string()
    };

    format_ident!("{}", name)
}

/// Clean HTML tags from documentation strings.
fn clean_html(s: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;

    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(c),
            _ => {}
        }
    }

    result.split_whitespace().collect::<Vec<_>>().join(" ")
}
