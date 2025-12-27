//! Port message helpers.
//!
//! Provides ergonomic wrappers for Chrome Port event listeners and messaging.
//! Ports allow long-lived connections between different parts of an extension.
//!
//! # Example
//!
//! ```rust,ignore
//! use nexum_chrome_gloo::runtime::port::{self, PortListener};
//! use nexum_chrome_sys::runtime::Port;
//! use wasm_bindgen::prelude::*;
//!
//! fn setup_port_handlers(port: &Port) -> Result<PortListener, JsValue> {
//!     PortListener::new(
//!         port,
//!         |msg, port| {
//!             web_sys::console::log_1(&msg);
//!         },
//!         |port| {
//!             web_sys::console::log_1(&"Port disconnected".into());
//!         },
//!     )
//! }
//! ```

use nexum_chrome_sys::runtime::Port;
use wasm_bindgen::prelude::*;

use crate::events::{EventListener1, EventListener2};

/// An RAII wrapper for Port event listeners.
///
/// This struct manages both `onMessage` and `onDisconnect` listeners for a Port.
/// When dropped, both listeners are automatically removed.
#[must_use = "port listeners will be removed when dropped, call .forget() to prevent this"]
pub struct PortListener {
    on_message: EventListener2,
    on_disconnect: EventListener1,
}

impl std::fmt::Debug for PortListener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PortListener")
            .field("on_message", &self.on_message)
            .field("on_disconnect", &self.on_disconnect)
            .finish()
    }
}

impl PortListener {
    /// Create new listeners for a Port's message and disconnect events.
    ///
    /// # Arguments
    ///
    /// * `port` - The port to listen on
    /// * `on_message` - Callback invoked when a message is received (receives message, port)
    /// * `on_disconnect` - Callback invoked when the port disconnects (receives port)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let listener = PortListener::new(
    ///     &port,
    ///     |message, port| {
    ///         // Handle incoming message
    ///     },
    ///     |port| {
    ///         // Handle disconnection
    ///     },
    /// )?;
    ///
    /// // Keep listeners alive
    /// listener.forget();
    /// ```
    pub fn new<M, D>(port: &Port, on_message: M, on_disconnect: D) -> Result<Self, JsValue>
    where
        M: FnMut(JsValue, JsValue) + 'static,
        D: FnMut(JsValue) + 'static,
    {
        let on_message_listener = EventListener2::new(&port.on_message(), on_message)?;
        let on_disconnect_listener = EventListener1::new(&port.on_disconnect(), on_disconnect)?;

        Ok(Self {
            on_message: on_message_listener,
            on_disconnect: on_disconnect_listener,
        })
    }

    /// Prevent both listeners from being removed when this struct is dropped.
    pub fn forget(self) {
        self.on_message.forget();
        self.on_disconnect.forget();
    }
}

/// Adds a listener for the `onDisconnect` event on a Port.
///
/// Returns an RAII guard that removes the listener when dropped.
///
/// # Arguments
///
/// * `port` - The port to listen on
/// * `callback` - Function to call when the port disconnects (receives port)
pub fn on_disconnect<F>(port: &Port, callback: F) -> Result<EventListener1, JsValue>
where
    F: FnMut(JsValue) + 'static,
{
    EventListener1::new(&port.on_disconnect(), callback)
}

/// Adds a listener for the `onMessage` event on a Port.
///
/// Returns an RAII guard that removes the listener when dropped.
///
/// # Arguments
///
/// * `port` - The port to listen on
/// * `callback` - Function to call when a message is received (receives message, port)
pub fn on_message<F>(port: &Port, callback: F) -> Result<EventListener2, JsValue>
where
    F: FnMut(JsValue, JsValue) + 'static,
{
    EventListener2::new(&port.on_message(), callback)
}

/// Posts a message through a Port.
///
/// The message is delivered to the other end's `onMessage` listener.
///
/// # Arguments
///
/// * `port` - The port to send the message through
/// * `message` - The message to send (will be serialized as JSON)
pub fn post_message(port: &Port, message: JsValue) -> Result<(), JsValue> {
    port.get_post_message().call1(port, &message)?;
    Ok(())
}
