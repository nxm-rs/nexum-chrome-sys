//! Chrome Event listener utilities.
//!
//! Provides RAII wrappers for Chrome extension event listeners, similar to
//! how [`gloo-events`](https://docs.rs/gloo-events) wraps DOM events.
//!
//! Chrome events use `addListener`/`removeListener` rather than the DOM's
//! `addEventListener`/`removeEventListener`. This module provides ergonomic
//! wrappers that automatically remove listeners when dropped.
//!
//! # Event Listener Types
//!
//! Different Chrome events pass different numbers of arguments to their callbacks:
//!
//! - [`EventListener1`]: Single argument (e.g., `chrome.alarms.onAlarm`)
//! - [`EventListener2`]: Two arguments (e.g., `port.onMessage` with message, port)
//! - [`EventListener3`]: Three arguments (e.g., `chrome.runtime.onMessage`)
//!
//! # Example
//!
//! ```rust,ignore
//! use nexum_chrome_gloo::events::EventListener3;
//! use wasm_bindgen::prelude::*;
//!
//! // The listener is automatically removed when `listener` is dropped
//! let listener = EventListener3::new(
//!     &chrome.runtime.on_message(),
//!     |message, sender, send_response| {
//!         // Handle message
//!     },
//! )?;
//!
//! // To keep the listener alive indefinitely, use forget()
//! listener.forget();
//! ```

use js_sys::{Function, Reflect};
use wasm_bindgen::prelude::*;

/// Macro to generate EventListener types for different callback arities.
macro_rules! define_event_listener {
    (
        $(#[$meta:meta])*
        $name:ident ( $($arg:ident),* )
    ) => {
        $(#[$meta])*
        #[must_use = "event listener will be removed when dropped, call .forget() to prevent this"]
        pub struct $name {
            event: JsValue,
            callback: Option<Closure<dyn FnMut($($arg),*)>>,
            callback_ref: Function,
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("event", &self.event)
                    .field("forgotten", &self.callback.is_none())
                    .finish()
            }
        }

        impl $name {
            /// Create a new event listener.
            ///
            /// # Arguments
            ///
            /// * `event` - The Chrome event object (e.g., `chrome.tabs.onUpdated`)
            /// * `callback` - The callback to invoke when the event fires
            ///
            /// # Returns
            ///
            /// Returns `Ok(EventListener)` if the listener was added successfully,
            /// or `Err` if adding the listener failed.
            pub fn new<F>(event: &JsValue, callback: F) -> Result<Self, JsValue>
            where
                F: FnMut($($arg),*) + 'static,
            {
                let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut($($arg),*)>);
                let callback_ref: Function = closure.as_ref().unchecked_ref::<Function>().clone();

                let add_listener =
                    Reflect::get(event, &JsValue::from_str("addListener"))?.dyn_into::<Function>()?;
                add_listener.call1(event, &callback_ref)?;

                Ok(Self {
                    event: event.clone(),
                    callback: Some(closure),
                    callback_ref,
                })
            }

            /// Prevent the listener from being removed when this struct is dropped.
            ///
            /// After calling this method, the listener will remain active indefinitely
            /// (until the extension is unloaded).
            pub fn forget(mut self) {
                if let Some(closure) = self.callback.take() {
                    closure.forget();
                }
                std::mem::forget(self);
            }

            /// Get a reference to the underlying Chrome event object.
            pub fn event(&self) -> &JsValue {
                &self.event
            }

            /// Get a reference to the callback function.
            pub fn callback(&self) -> &Function {
                &self.callback_ref
            }

            /// Check if this listener has been forgotten.
            pub fn is_forgotten(&self) -> bool {
                self.callback.is_none()
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                if self.callback.is_some() {
                    if let Ok(remove_listener) =
                        Reflect::get(&self.event, &JsValue::from_str("removeListener"))
                    {
                        if let Ok(func) = remove_listener.dyn_into::<Function>() {
                            let _ = func.call1(&self.event, &self.callback_ref);
                        }
                    }
                }
            }
        }
    };
}

define_event_listener! {
    /// An RAII wrapper for Chrome event listeners with single-argument callbacks.
    ///
    /// Use this for events like:
    /// - `chrome.alarms.onAlarm` - receives (alarm)
    /// - `port.onDisconnect` - receives (port)
    ///
    /// When this struct is dropped, the event listener is automatically removed.
    EventListener1(JsValue)
}

define_event_listener! {
    /// An RAII wrapper for Chrome event listeners with two-argument callbacks.
    ///
    /// Use this for events like:
    /// - `port.onMessage` - receives (message, port)
    ///
    /// When this struct is dropped, the event listener is automatically removed.
    EventListener2(JsValue, JsValue)
}

define_event_listener! {
    /// An RAII wrapper for Chrome event listeners with three-argument callbacks.
    ///
    /// Use this for events like:
    /// - `chrome.runtime.onMessage` - receives (message, sender, sendResponse)
    /// - `chrome.tabs.onUpdated` - receives (tabId, changeInfo, tab)
    ///
    /// When this struct is dropped, the event listener is automatically removed.
    EventListener3(JsValue, JsValue, JsValue)
}
