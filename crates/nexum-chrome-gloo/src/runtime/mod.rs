//! Chrome Runtime API helpers.
//!
//! Provides ergonomic wrappers for Chrome runtime APIs, including
//! port-based messaging between extension components.
//!
//! # Port Messaging
//!
//! The [`port`] module provides RAII helpers for working with Chrome's
//! long-lived message connections.
//!
//! ```rust,ignore
//! use nexum_chrome_gloo::runtime::port::{self, PortListener};
//! use nexum_chrome_sys::runtime::Port;
//!
//! fn setup_port(port: &Port) -> Result<PortListener, JsValue> {
//!     PortListener::new(
//!         port,
//!         |msg, _port| { /* handle message */ },
//!         |_port| { /* handle disconnect */ },
//!     )
//! }
//! ```

pub mod port;
