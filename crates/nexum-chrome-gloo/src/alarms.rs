//! Chrome Alarms API helpers.
//!
//! Provides ergonomic wrappers for scheduling and managing alarms
//! in Chrome extensions.
//!
//! # Example
//!
//! ```rust,ignore
//! use nexum_chrome_gloo::alarms::{self, Alarm, AlarmCreateInfo};
//!
//! // Create a periodic alarm
//! let mut info = AlarmCreateInfo::new();
//! info.set_period_in_minutes(5.0);
//! alarms::create("my-alarm", &info);
//!
//! // Check if alarm exists
//! if let Some(alarm) = alarms::get("my-alarm").await? {
//!     println!("Alarm scheduled for: {}", alarm.get_scheduled_time());
//! }
//! ```

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

// Re-export types from sys crate
pub use nexum_chrome_sys::alarms::{Alarm, AlarmCreateInfo};

/// Creates an alarm with the given name and configuration.
///
/// If an alarm with the same name already exists, it will be replaced.
///
/// # Arguments
///
/// * `name` - A name to identify this alarm
/// * `info` - Configuration for when the alarm should fire
///
/// # Note
///
/// Chrome limits alarms to at most once every 30 seconds in production.
/// There is no limit when the extension is loaded unpacked for development.
pub fn create(name: &str, info: &AlarmCreateInfo) {
    let _ = nexum_chrome_sys::alarms::create(Some(name.to_string()), info.clone());
}

/// Retrieves an alarm by name.
///
/// # Arguments
///
/// * `name` - The name of the alarm to retrieve
///
/// # Returns
///
/// `Ok(Some(alarm))` if the alarm exists, `Ok(None)` if not found,
/// or `Err` if an error occurred.
pub async fn get(name: &str) -> Result<Option<Alarm>, JsValue> {
    let promise = nexum_chrome_sys::alarms::get(Some(name.to_string()));
    let result = JsFuture::from(promise).await?;
    if result.is_undefined() || result.is_null() {
        Ok(None)
    } else {
        Ok(Some(result.unchecked_into()))
    }
}

/// Clears an alarm by name.
///
/// # Arguments
///
/// * `name` - The name of the alarm to clear
///
/// # Returns
///
/// `Ok(true)` if the alarm was found and cleared, `Ok(false)` if no alarm
/// with that name was found.
pub async fn clear(name: &str) -> Result<bool, JsValue> {
    let promise = nexum_chrome_sys::alarms::clear(Some(name.to_string()));
    let result = JsFuture::from(promise).await?;
    Ok(result.as_bool().unwrap_or(false))
}

/// Clears all alarms.
///
/// # Returns
///
/// `Ok(true)` if any alarms were cleared, `Ok(false)` if there were no alarms.
pub async fn clear_all() -> Result<bool, JsValue> {
    let promise = nexum_chrome_sys::alarms::clear_all();
    let result = JsFuture::from(promise).await?;
    Ok(result.as_bool().unwrap_or(false))
}
