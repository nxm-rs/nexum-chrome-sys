//! Generated Chrome Extension API bindings.
//!
//! This module is auto-generated. Do not edit manually.
//!
//! Each API namespace is exposed as a separate module.
//! Use like: `nexum_chrome_sys::tabs::query(...)`

#[cfg(feature = "action")]
mod gen_action;
#[cfg(feature = "action")]
pub mod action {
    pub use super::gen_action::*;
}

#[cfg(feature = "alarms")]
mod gen_alarms;
#[cfg(feature = "alarms")]
pub mod alarms {
    pub use super::gen_alarms::*;
}

#[cfg(feature = "app")]
mod gen_app;
#[cfg(feature = "app")]
pub mod app {
    pub use super::gen_app::*;
}

#[cfg(feature = "app_runtime")]
mod gen_app_runtime;
#[cfg(feature = "app_runtime")]
pub mod app_runtime {
    pub use super::gen_app_runtime::*;
}

#[cfg(feature = "app_window")]
mod gen_app_window;
#[cfg(feature = "app_window")]
pub mod app_window {
    pub use super::gen_app_window::*;
}

#[cfg(feature = "appview_tag")]
mod gen_appview_tag;
#[cfg(feature = "appview_tag")]
pub mod appview_tag {
    pub use super::gen_appview_tag::*;
}

#[cfg(feature = "audio")]
mod gen_audio;
#[cfg(feature = "audio")]
pub mod audio {
    pub use super::gen_audio::*;
}

#[cfg(feature = "automation")]
mod gen_automation;
#[cfg(feature = "automation")]
pub mod automation {
    pub use super::gen_automation::*;
}

#[cfg(feature = "bluetooth")]
mod gen_bluetooth;
#[cfg(feature = "bluetooth")]
pub mod bluetooth {
    pub use super::gen_bluetooth::*;
}

#[cfg(feature = "bluetooth_low_energy")]
mod gen_bluetooth_low_energy;
#[cfg(feature = "bluetooth_low_energy")]
pub mod bluetooth_low_energy {
    pub use super::gen_bluetooth_low_energy::*;
}

#[cfg(feature = "bluetooth_socket")]
mod gen_bluetooth_socket;
#[cfg(feature = "bluetooth_socket")]
pub mod bluetooth_socket {
    pub use super::gen_bluetooth_socket::*;
}

#[cfg(feature = "bookmarks")]
mod gen_bookmarks;
#[cfg(feature = "bookmarks")]
pub mod bookmarks {
    pub use super::gen_bookmarks::*;
}

#[cfg(feature = "browser")]
mod gen_browser;
#[cfg(feature = "browser")]
pub mod browser {
    pub use super::gen_browser::*;
}

#[cfg(feature = "browser_action")]
mod gen_browser_action;
#[cfg(feature = "browser_action")]
pub mod browser_action {
    pub use super::gen_browser_action::*;
}

#[cfg(feature = "browsing_data")]
mod gen_browsing_data;
#[cfg(feature = "browsing_data")]
pub mod browsing_data {
    pub use super::gen_browsing_data::*;
}

#[cfg(feature = "certificate_provider")]
mod gen_certificate_provider;
#[cfg(feature = "certificate_provider")]
pub mod certificate_provider {
    pub use super::gen_certificate_provider::*;
}

#[cfg(feature = "chrome_url_overrides")]
mod gen_chrome_url_overrides;
#[cfg(feature = "chrome_url_overrides")]
pub mod chrome_url_overrides {
    pub use super::gen_chrome_url_overrides::*;
}

#[cfg(feature = "clipboard")]
mod gen_clipboard;
#[cfg(feature = "clipboard")]
pub mod clipboard {
    pub use super::gen_clipboard::*;
}

#[cfg(feature = "commands")]
mod gen_commands;
#[cfg(feature = "commands")]
pub mod commands {
    pub use super::gen_commands::*;
}

#[cfg(feature = "content_scripts")]
mod gen_content_scripts;
#[cfg(feature = "content_scripts")]
pub mod content_scripts {
    pub use super::gen_content_scripts::*;
}

#[cfg(feature = "content_settings")]
mod gen_content_settings;
#[cfg(feature = "content_settings")]
pub mod content_settings {
    pub use super::gen_content_settings::*;
}

#[cfg(feature = "context_menus")]
mod gen_context_menus;
#[cfg(feature = "context_menus")]
pub mod context_menus {
    pub use super::gen_context_menus::*;
}

#[cfg(feature = "cookies")]
mod gen_cookies;
#[cfg(feature = "cookies")]
pub mod cookies {
    pub use super::gen_cookies::*;
}

#[cfg(feature = "cross_origin_isolation")]
mod gen_cross_origin_isolation;
#[cfg(feature = "cross_origin_isolation")]
pub mod cross_origin_isolation {
    pub use super::gen_cross_origin_isolation::*;
}

#[cfg(feature = "debugger")]
mod gen_debugger;
#[cfg(feature = "debugger")]
pub mod debugger {
    pub use super::gen_debugger::*;
}

#[cfg(feature = "declarative_content")]
mod gen_declarative_content;
#[cfg(feature = "declarative_content")]
pub mod declarative_content {
    pub use super::gen_declarative_content::*;
}

#[cfg(feature = "declarative_net_request")]
mod gen_declarative_net_request;
#[cfg(feature = "declarative_net_request")]
pub mod declarative_net_request {
    pub use super::gen_declarative_net_request::*;
}

#[cfg(feature = "declarative_web_request")]
mod gen_declarative_web_request;
#[cfg(feature = "declarative_web_request")]
pub mod declarative_web_request {
    pub use super::gen_declarative_web_request::*;
}

#[cfg(feature = "desktop_capture")]
mod gen_desktop_capture;
#[cfg(feature = "desktop_capture")]
pub mod desktop_capture {
    pub use super::gen_desktop_capture::*;
}

#[cfg(feature = "devtools_inspected_window")]
mod gen_devtools_inspected_window;
#[cfg(feature = "devtools_inspected_window")]
pub mod devtools_inspected_window {
    pub use super::gen_devtools_inspected_window::*;
}

#[cfg(feature = "devtools_network")]
mod gen_devtools_network;
#[cfg(feature = "devtools_network")]
pub mod devtools_network {
    pub use super::gen_devtools_network::*;
}

#[cfg(feature = "devtools_panels")]
mod gen_devtools_panels;
#[cfg(feature = "devtools_panels")]
pub mod devtools_panels {
    pub use super::gen_devtools_panels::*;
}

#[cfg(feature = "devtools_performance")]
mod gen_devtools_performance;
#[cfg(feature = "devtools_performance")]
pub mod devtools_performance {
    pub use super::gen_devtools_performance::*;
}

#[cfg(feature = "devtools_recorder")]
mod gen_devtools_recorder;
#[cfg(feature = "devtools_recorder")]
pub mod devtools_recorder {
    pub use super::gen_devtools_recorder::*;
}

#[cfg(feature = "diagnostics")]
mod gen_diagnostics;
#[cfg(feature = "diagnostics")]
pub mod diagnostics {
    pub use super::gen_diagnostics::*;
}

#[cfg(feature = "dns")]
mod gen_dns;
#[cfg(feature = "dns")]
pub mod dns {
    pub use super::gen_dns::*;
}

#[cfg(feature = "document_scan")]
mod gen_document_scan;
#[cfg(feature = "document_scan")]
pub mod document_scan {
    pub use super::gen_document_scan::*;
}

#[cfg(feature = "dom")]
mod gen_dom;
#[cfg(feature = "dom")]
pub mod dom {
    pub use super::gen_dom::*;
}

#[cfg(feature = "downloads")]
mod gen_downloads;
#[cfg(feature = "downloads")]
pub mod downloads {
    pub use super::gen_downloads::*;
}

#[cfg(feature = "enterprise_device_attributes")]
mod gen_enterprise_device_attributes;
#[cfg(feature = "enterprise_device_attributes")]
pub mod enterprise_device_attributes {
    pub use super::gen_enterprise_device_attributes::*;
}

#[cfg(feature = "enterprise_hardware_platform")]
mod gen_enterprise_hardware_platform;
#[cfg(feature = "enterprise_hardware_platform")]
pub mod enterprise_hardware_platform {
    pub use super::gen_enterprise_hardware_platform::*;
}

#[cfg(feature = "enterprise_kiosk_input")]
mod gen_enterprise_kiosk_input;
#[cfg(feature = "enterprise_kiosk_input")]
pub mod enterprise_kiosk_input {
    pub use super::gen_enterprise_kiosk_input::*;
}

#[cfg(feature = "enterprise_login")]
mod gen_enterprise_login;
#[cfg(feature = "enterprise_login")]
pub mod enterprise_login {
    pub use super::gen_enterprise_login::*;
}

#[cfg(feature = "enterprise_networking_attributes")]
mod gen_enterprise_networking_attributes;
#[cfg(feature = "enterprise_networking_attributes")]
pub mod enterprise_networking_attributes {
    pub use super::gen_enterprise_networking_attributes::*;
}

#[cfg(feature = "enterprise_platform_keys")]
mod gen_enterprise_platform_keys;
#[cfg(feature = "enterprise_platform_keys")]
pub mod enterprise_platform_keys {
    pub use super::gen_enterprise_platform_keys::*;
}

#[cfg(feature = "enterprise_remote_apps")]
mod gen_enterprise_remote_apps;
#[cfg(feature = "enterprise_remote_apps")]
pub mod enterprise_remote_apps {
    pub use super::gen_enterprise_remote_apps::*;
}

#[cfg(feature = "events")]
mod gen_events;
#[cfg(feature = "events")]
pub mod events {
    pub use super::gen_events::*;
}

#[cfg(feature = "experimental_actor")]
mod gen_experimental_actor;
#[cfg(feature = "experimental_actor")]
pub mod experimental_actor {
    pub use super::gen_experimental_actor::*;
}

#[cfg(feature = "experimental_ai_data")]
mod gen_experimental_ai_data;
#[cfg(feature = "experimental_ai_data")]
pub mod experimental_ai_data {
    pub use super::gen_experimental_ai_data::*;
}

#[cfg(feature = "extension")]
mod gen_extension;
#[cfg(feature = "extension")]
pub mod extension {
    pub use super::gen_extension::*;
}

#[cfg(feature = "extension_types")]
mod gen_extension_types;
#[cfg(feature = "extension_types")]
pub mod extension_types {
    pub use super::gen_extension_types::*;
}

#[cfg(feature = "extensions_manifest_types")]
mod gen_extensions_manifest_types;
#[cfg(feature = "extensions_manifest_types")]
pub mod extensions_manifest_types {
    pub use super::gen_extensions_manifest_types::*;
}

#[cfg(feature = "file_browser_handler")]
mod gen_file_browser_handler;
#[cfg(feature = "file_browser_handler")]
pub mod file_browser_handler {
    pub use super::gen_file_browser_handler::*;
}

#[cfg(feature = "file_handlers")]
mod gen_file_handlers;
#[cfg(feature = "file_handlers")]
pub mod file_handlers {
    pub use super::gen_file_handlers::*;
}

#[cfg(feature = "file_system")]
mod gen_file_system;
#[cfg(feature = "file_system")]
pub mod file_system {
    pub use super::gen_file_system::*;
}

#[cfg(feature = "file_system_provider")]
mod gen_file_system_provider;
#[cfg(feature = "file_system_provider")]
pub mod file_system_provider {
    pub use super::gen_file_system_provider::*;
}

#[cfg(feature = "font_settings")]
mod gen_font_settings;
#[cfg(feature = "font_settings")]
pub mod font_settings {
    pub use super::gen_font_settings::*;
}

#[cfg(feature = "gcm")]
mod gen_gcm;
#[cfg(feature = "gcm")]
pub mod gcm {
    pub use super::gen_gcm::*;
}

#[cfg(feature = "hid")]
mod gen_hid;
#[cfg(feature = "hid")]
pub mod hid {
    pub use super::gen_hid::*;
}

#[cfg(feature = "history")]
mod gen_history;
#[cfg(feature = "history")]
pub mod history {
    pub use super::gen_history::*;
}

#[cfg(feature = "i18n")]
mod gen_i18n;
#[cfg(feature = "i18n")]
pub mod i18n {
    pub use super::gen_i18n::*;
}

#[cfg(feature = "icon_variants")]
mod gen_icon_variants;
#[cfg(feature = "icon_variants")]
pub mod icon_variants {
    pub use super::gen_icon_variants::*;
}

#[cfg(feature = "identity")]
mod gen_identity;
#[cfg(feature = "identity")]
pub mod identity {
    pub use super::gen_identity::*;
}

#[cfg(feature = "idle")]
mod gen_idle;
#[cfg(feature = "idle")]
pub mod idle {
    pub use super::gen_idle::*;
}

#[cfg(feature = "idltest")]
mod gen_idltest;
#[cfg(feature = "idltest")]
pub mod idltest {
    pub use super::gen_idltest::*;
}

#[cfg(feature = "incognito")]
mod gen_incognito;
#[cfg(feature = "incognito")]
pub mod incognito {
    pub use super::gen_incognito::*;
}

#[cfg(feature = "input_ime")]
mod gen_input_ime;
#[cfg(feature = "input_ime")]
pub mod input_ime {
    pub use super::gen_input_ime::*;
}

#[cfg(feature = "instance_id")]
mod gen_instance_id;
#[cfg(feature = "instance_id")]
pub mod instance_id {
    pub use super::gen_instance_id::*;
}

#[cfg(feature = "login")]
mod gen_login;
#[cfg(feature = "login")]
pub mod login {
    pub use super::gen_login::*;
}

#[cfg(feature = "login_screen_storage")]
mod gen_login_screen_storage;
#[cfg(feature = "login_screen_storage")]
pub mod login_screen_storage {
    pub use super::gen_login_screen_storage::*;
}

#[cfg(feature = "login_screen_ui")]
mod gen_login_screen_ui;
#[cfg(feature = "login_screen_ui")]
pub mod login_screen_ui {
    pub use super::gen_login_screen_ui::*;
}

#[cfg(feature = "login_state")]
mod gen_login_state;
#[cfg(feature = "login_state")]
pub mod login_state {
    pub use super::gen_login_state::*;
}

#[cfg(feature = "management")]
mod gen_management;
#[cfg(feature = "management")]
pub mod management {
    pub use super::gen_management::*;
}

#[cfg(feature = "manifest_types")]
mod gen_manifest_types;
#[cfg(feature = "manifest_types")]
pub mod manifest_types {
    pub use super::gen_manifest_types::*;
}

#[cfg(feature = "mdns")]
mod gen_mdns;
#[cfg(feature = "mdns")]
pub mod mdns {
    pub use super::gen_mdns::*;
}

#[cfg(feature = "media_galleries")]
mod gen_media_galleries;
#[cfg(feature = "media_galleries")]
pub mod media_galleries {
    pub use super::gen_media_galleries::*;
}

#[cfg(feature = "networking_onc")]
mod gen_networking_onc;
#[cfg(feature = "networking_onc")]
pub mod networking_onc {
    pub use super::gen_networking_onc::*;
}

#[cfg(feature = "notifications")]
mod gen_notifications;
#[cfg(feature = "notifications")]
pub mod notifications {
    pub use super::gen_notifications::*;
}

#[cfg(feature = "oauth2")]
mod gen_oauth2;
#[cfg(feature = "oauth2")]
pub mod oauth2 {
    pub use super::gen_oauth2::*;
}

#[cfg(feature = "offscreen")]
mod gen_offscreen;
#[cfg(feature = "offscreen")]
pub mod offscreen {
    pub use super::gen_offscreen::*;
}

#[cfg(feature = "omnibox")]
mod gen_omnibox;
#[cfg(feature = "omnibox")]
pub mod omnibox {
    pub use super::gen_omnibox::*;
}

#[cfg(feature = "page_action")]
mod gen_page_action;
#[cfg(feature = "page_action")]
pub mod page_action {
    pub use super::gen_page_action::*;
}

#[cfg(feature = "page_capture")]
mod gen_page_capture;
#[cfg(feature = "page_capture")]
pub mod page_capture {
    pub use super::gen_page_capture::*;
}

#[cfg(feature = "permissions")]
mod gen_permissions;
#[cfg(feature = "permissions")]
pub mod permissions {
    pub use super::gen_permissions::*;
}

#[cfg(feature = "platform_keys")]
mod gen_platform_keys;
#[cfg(feature = "platform_keys")]
pub mod platform_keys {
    pub use super::gen_platform_keys::*;
}

#[cfg(feature = "power")]
mod gen_power;
#[cfg(feature = "power")]
pub mod power {
    pub use super::gen_power::*;
}

#[cfg(feature = "printer_provider")]
mod gen_printer_provider;
#[cfg(feature = "printer_provider")]
pub mod printer_provider {
    pub use super::gen_printer_provider::*;
}

#[cfg(feature = "printing")]
mod gen_printing;
#[cfg(feature = "printing")]
pub mod printing {
    pub use super::gen_printing::*;
}

#[cfg(feature = "printing_metrics")]
mod gen_printing_metrics;
#[cfg(feature = "printing_metrics")]
pub mod printing_metrics {
    pub use super::gen_printing_metrics::*;
}

#[cfg(feature = "privacy")]
mod gen_privacy;
#[cfg(feature = "privacy")]
pub mod privacy {
    pub use super::gen_privacy::*;
}

#[cfg(feature = "processes")]
mod gen_processes;
#[cfg(feature = "processes")]
pub mod processes {
    pub use super::gen_processes::*;
}

#[cfg(feature = "protocol_handlers")]
mod gen_protocol_handlers;
#[cfg(feature = "protocol_handlers")]
pub mod protocol_handlers {
    pub use super::gen_protocol_handlers::*;
}

#[cfg(feature = "proxy")]
mod gen_proxy;
#[cfg(feature = "proxy")]
pub mod proxy {
    pub use super::gen_proxy::*;
}

#[cfg(feature = "reading_list")]
mod gen_reading_list;
#[cfg(feature = "reading_list")]
pub mod reading_list {
    pub use super::gen_reading_list::*;
}

#[cfg(feature = "requirements")]
mod gen_requirements;
#[cfg(feature = "requirements")]
pub mod requirements {
    pub use super::gen_requirements::*;
}

#[cfg(feature = "runtime")]
mod gen_runtime;
#[cfg(feature = "runtime")]
pub mod runtime {
    pub use super::gen_runtime::*;
}

#[cfg(feature = "scripting")]
mod gen_scripting;
#[cfg(feature = "scripting")]
pub mod scripting {
    pub use super::gen_scripting::*;
}

#[cfg(feature = "search")]
mod gen_search;
#[cfg(feature = "search")]
pub mod search {
    pub use super::gen_search::*;
}

#[cfg(feature = "serial")]
mod gen_serial;
#[cfg(feature = "serial")]
pub mod serial {
    pub use super::gen_serial::*;
}

#[cfg(feature = "sessions")]
mod gen_sessions;
#[cfg(feature = "sessions")]
pub mod sessions {
    pub use super::gen_sessions::*;
}

#[cfg(feature = "shared_module")]
mod gen_shared_module;
#[cfg(feature = "shared_module")]
pub mod shared_module {
    pub use super::gen_shared_module::*;
}

#[cfg(feature = "side_panel")]
mod gen_side_panel;
#[cfg(feature = "side_panel")]
pub mod side_panel {
    pub use super::gen_side_panel::*;
}

#[cfg(feature = "socket")]
mod gen_socket;
#[cfg(feature = "socket")]
pub mod socket {
    pub use super::gen_socket::*;
}

#[cfg(feature = "sockets_tcp")]
mod gen_sockets_tcp;
#[cfg(feature = "sockets_tcp")]
pub mod sockets_tcp {
    pub use super::gen_sockets_tcp::*;
}

#[cfg(feature = "sockets_tcp_server")]
mod gen_sockets_tcp_server;
#[cfg(feature = "sockets_tcp_server")]
pub mod sockets_tcp_server {
    pub use super::gen_sockets_tcp_server::*;
}

#[cfg(feature = "sockets_udp")]
mod gen_sockets_udp;
#[cfg(feature = "sockets_udp")]
pub mod sockets_udp {
    pub use super::gen_sockets_udp::*;
}

#[cfg(feature = "storage")]
mod gen_storage;
#[cfg(feature = "storage")]
pub mod storage {
    pub use super::gen_storage::*;
}

#[cfg(feature = "sync_file_system")]
mod gen_sync_file_system;
#[cfg(feature = "sync_file_system")]
pub mod sync_file_system {
    pub use super::gen_sync_file_system::*;
}

#[cfg(feature = "system_cpu")]
mod gen_system_cpu;
#[cfg(feature = "system_cpu")]
pub mod system_cpu {
    pub use super::gen_system_cpu::*;
}

#[cfg(feature = "system_display")]
mod gen_system_display;
#[cfg(feature = "system_display")]
pub mod system_display {
    pub use super::gen_system_display::*;
}

#[cfg(feature = "system_log")]
mod gen_system_log;
#[cfg(feature = "system_log")]
pub mod system_log {
    pub use super::gen_system_log::*;
}

#[cfg(feature = "system_memory")]
mod gen_system_memory;
#[cfg(feature = "system_memory")]
pub mod system_memory {
    pub use super::gen_system_memory::*;
}

#[cfg(feature = "system_network")]
mod gen_system_network;
#[cfg(feature = "system_network")]
pub mod system_network {
    pub use super::gen_system_network::*;
}

#[cfg(feature = "system_storage")]
mod gen_system_storage;
#[cfg(feature = "system_storage")]
pub mod system_storage {
    pub use super::gen_system_storage::*;
}

#[cfg(feature = "tab_capture")]
mod gen_tab_capture;
#[cfg(feature = "tab_capture")]
pub mod tab_capture {
    pub use super::gen_tab_capture::*;
}

#[cfg(feature = "tab_groups")]
mod gen_tab_groups;
#[cfg(feature = "tab_groups")]
pub mod tab_groups {
    pub use super::gen_tab_groups::*;
}

#[cfg(feature = "tabs")]
mod gen_tabs;
#[cfg(feature = "tabs")]
pub mod tabs {
    pub use super::gen_tabs::*;
}

#[cfg(feature = "test")]
mod gen_test;
#[cfg(feature = "test")]
pub mod test {
    pub use super::gen_test::*;
}

#[cfg(feature = "top_sites")]
mod gen_top_sites;
#[cfg(feature = "top_sites")]
pub mod top_sites {
    pub use super::gen_top_sites::*;
}

#[cfg(feature = "tts")]
mod gen_tts;
#[cfg(feature = "tts")]
pub mod tts {
    pub use super::gen_tts::*;
}

#[cfg(feature = "tts_engine")]
mod gen_tts_engine;
#[cfg(feature = "tts_engine")]
pub mod tts_engine {
    pub use super::gen_tts_engine::*;
}

#[cfg(feature = "types")]
mod gen_types;
#[cfg(feature = "types")]
pub mod types {
    pub use super::gen_types::*;
}

#[cfg(feature = "usb")]
mod gen_usb;
#[cfg(feature = "usb")]
pub mod usb {
    pub use super::gen_usb::*;
}

#[cfg(feature = "user_scripts")]
mod gen_user_scripts;
#[cfg(feature = "user_scripts")]
pub mod user_scripts {
    pub use super::gen_user_scripts::*;
}

#[cfg(feature = "virtual_keyboard")]
mod gen_virtual_keyboard;
#[cfg(feature = "virtual_keyboard")]
pub mod virtual_keyboard {
    pub use super::gen_virtual_keyboard::*;
}

#[cfg(feature = "vpn_provider")]
mod gen_vpn_provider;
#[cfg(feature = "vpn_provider")]
pub mod vpn_provider {
    pub use super::gen_vpn_provider::*;
}

#[cfg(feature = "wallpaper")]
mod gen_wallpaper;
#[cfg(feature = "wallpaper")]
pub mod wallpaper {
    pub use super::gen_wallpaper::*;
}

#[cfg(feature = "web_accessible_resources")]
mod gen_web_accessible_resources;
#[cfg(feature = "web_accessible_resources")]
pub mod web_accessible_resources {
    pub use super::gen_web_accessible_resources::*;
}

#[cfg(feature = "web_authentication_proxy")]
mod gen_web_authentication_proxy;
#[cfg(feature = "web_authentication_proxy")]
pub mod web_authentication_proxy {
    pub use super::gen_web_authentication_proxy::*;
}

#[cfg(feature = "web_navigation")]
mod gen_web_navigation;
#[cfg(feature = "web_navigation")]
pub mod web_navigation {
    pub use super::gen_web_navigation::*;
}

#[cfg(feature = "web_request")]
mod gen_web_request;
#[cfg(feature = "web_request")]
pub mod web_request {
    pub use super::gen_web_request::*;
}

#[cfg(feature = "webview_tag")]
mod gen_webview_tag;
#[cfg(feature = "webview_tag")]
pub mod webview_tag {
    pub use super::gen_webview_tag::*;
}

#[cfg(feature = "windows")]
mod gen_windows;
#[cfg(feature = "windows")]
pub mod windows {
    pub use super::gen_windows::*;
}
