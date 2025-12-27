//! Utility functions for code generation.

use heck::ToUpperCamelCase;
use quote::format_ident;

/// Rust keywords that need to be escaped with r# prefix or renamed.
pub const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];

/// Keywords that cannot be raw identifiers and need to be renamed.
pub const NON_RAW_KEYWORDS: &[&str] = &["self", "Self", "super", "crate"];

/// Convert a string to a valid Rust enum variant name.
pub fn to_enum_variant(s: &str) -> String {
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

/// Create an identifier, using raw identifier syntax for Rust keywords.
pub fn make_ident(name: &str) -> proc_macro2::Ident {
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
pub fn make_type_ident(name: &str) -> proc_macro2::Ident {
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
pub fn clean_html(s: &str) -> String {
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
