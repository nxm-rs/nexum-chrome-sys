//! Chrome Extension API bindings for Rust/WebAssembly.
//!
//! This crate provides Rust bindings to Chrome Extension APIs using wasm-bindgen.
//! Each API namespace is feature-gated - enable the features you need in your Cargo.toml.
//!
//! # Example
//!
//! ```toml
//! [dependencies]
//! nexum-chrome-sys = { version = "0.1", features = ["tabs", "runtime", "storage"] }
//! ```

mod features;
#[allow(unused_imports)]
pub use features::*;
