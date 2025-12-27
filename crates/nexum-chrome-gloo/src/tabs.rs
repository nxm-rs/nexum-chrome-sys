//! Chrome Tabs API helpers.
//!
//! Provides ergonomic wrappers for working with browser tabs,
//! including querying, messaging, and tracking tab state.
//!
//! # Example
//!
//! ```rust,ignore
//! use nexum_chrome_gloo::tabs::{self, QueryQueryInfo, Tab};
//!
//! async fn notify_active_tab(message: JsValue) -> Result<(), JsValue> {
//!     if let Some(tab) = tabs::get_active_tab().await? {
//!         tabs::send_message(tab.get_id().unwrap(), message).await?;
//!     }
//!     Ok(())
//! }
//! ```

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

// Re-export types from sys crate
pub use nexum_chrome_sys::tabs::{OnActivatedActiveInfo, OnUpdatedChangeInfo, QueryQueryInfo, Tab};

/// Query tabs matching the given criteria.
///
/// Returns all tabs that match the query parameters. If no parameters
/// are specified, returns all tabs.
///
/// # Arguments
///
/// * `query` - The query parameters to filter tabs
///
/// # Returns
///
/// A `JsValue` containing an array of matching tabs.
pub async fn query(query: &QueryQueryInfo) -> Result<JsValue, JsValue> {
    let promise = nexum_chrome_sys::tabs::query(query.clone().into());
    JsFuture::from(promise).await
}

/// Get the currently active tab in the current window.
///
/// # Returns
///
/// `Ok(Some(tab))` if there is an active tab, `Ok(None)` if not
/// (which should rarely happen).
pub async fn get_active_tab() -> Result<Option<Tab>, JsValue> {
    let query = QueryQueryInfo::new();
    query.set_active(true);
    query.set_current_window(true);

    let promise = nexum_chrome_sys::tabs::query(query.into());
    let result = JsFuture::from(promise).await?;

    let tabs = js_sys::Array::from(&result);
    if tabs.length() > 0 {
        Ok(Some(tabs.get(0).unchecked_into()))
    } else {
        Ok(None)
    }
}

/// Get a tab by its ID.
///
/// # Arguments
///
/// * `tab_id` - The ID of the tab to retrieve
///
/// # Returns
///
/// The `Tab` object for the specified tab.
pub async fn get(tab_id: i32) -> Result<Tab, JsValue> {
    let promise = nexum_chrome_sys::tabs::get(tab_id);
    let result = JsFuture::from(promise).await?;
    Ok(result.unchecked_into())
}

/// Send a message to a tab by ID.
///
/// # Arguments
///
/// * `tab_id` - The ID of the target tab
/// * `message` - The message to send
///
/// # Returns
///
/// The response from the content script, if any.
pub async fn send_message(tab_id: i32, message: JsValue) -> Result<JsValue, JsValue> {
    let promise = nexum_chrome_sys::tabs::send_message(tab_id, message, None);
    JsFuture::from(promise).await
}
