//! Ergonomic wrappers for Chrome Extension APIs.
//!
//! This crate provides idiomatic Rust wrappers around the raw `nexum-chrome-sys` bindings,
//! similar to how [`gloo`](https://crates.io/crates/gloo) wraps `web-sys`.
//!
//! # Features
//!
//! Each Chrome API namespace is feature-gated. Enable the features you need:
//!
//! ```toml
//! [dependencies]
//! nexum-chrome-gloo = { version = "0.1", features = ["tabs", "runtime", "alarms"] }
//! ```
//!
//! # Event Listeners
//!
//! Use the [`events`] module for RAII-style event listener management:
//!
//! ```rust,ignore
//! use nexum_chrome_gloo::events::EventListener;
//!
//! // Listener is automatically removed when dropped
//! let listener = EventListener::new(&chrome.tabs.on_updated(), |tab_id, info, tab| {
//!     // Handle event
//! })?;
//!
//! // Keep listener alive indefinitely
//! listener.forget();
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use nexum_chrome_gloo::tabs::{self, Tab};
//!
//! async fn get_current_tab() -> Result<Option<Tab>, wasm_bindgen::JsValue> {
//!     tabs::get_active_tab().await
//! }
//! ```

// Core utilities (always available)
pub mod events;

// Feature-gated API modules
#[cfg(feature = "alarms")]
pub mod alarms;

#[cfg(feature = "runtime")]
pub mod runtime;

#[cfg(feature = "tabs")]
pub mod tabs;
