#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RemovalOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Options that determine exactly what data will be removed.
    pub type RemovalOptions;
    ///Get the `excludeOrigins` field of this object.
    #[wasm_bindgen(method, getter = "excludeOrigins")]
    pub fn get_exclude_origins(this: &RemovalOptions) -> Option<Array>;
    ///Change the `excludeOrigins` field of this object.
    #[wasm_bindgen(method, setter = "excludeOrigins")]
    pub fn set_exclude_origins(this: &RemovalOptions, val: &Array);
    ///Get the `originTypes` field of this object.
    #[wasm_bindgen(method, getter = "originTypes")]
    pub fn get_origin_types(this: &RemovalOptions) -> Option<Object>;
    ///Change the `originTypes` field of this object.
    #[wasm_bindgen(method, setter = "originTypes")]
    pub fn set_origin_types(this: &RemovalOptions, val: &Object);
    ///Get the `origins` field of this object.
    #[wasm_bindgen(method, getter = "origins")]
    pub fn get_origins(this: &RemovalOptions) -> Option<Array>;
    ///Change the `origins` field of this object.
    #[wasm_bindgen(method, setter = "origins")]
    pub fn set_origins(this: &RemovalOptions, val: &Array);
    ///Get the `since` field of this object.
    #[wasm_bindgen(method, getter = "since")]
    pub fn get_since(this: &RemovalOptions) -> Option<f64>;
    ///Change the `since` field of this object.
    #[wasm_bindgen(method, setter = "since")]
    pub fn set_since(this: &RemovalOptions, val: f64);
}
impl RemovalOptions {
    ///Construct a new `RemovalOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_exclude_origins()` instead."]
    pub fn exclude_origins(&mut self, val: &Array) -> &mut Self {
        self.set_exclude_origins(val);
        self
    }
    #[deprecated = "Use `set_origin_types()` instead."]
    pub fn origin_types(&mut self, val: &Object) -> &mut Self {
        self.set_origin_types(val);
        self
    }
    #[deprecated = "Use `set_origins()` instead."]
    pub fn origins(&mut self, val: &Array) -> &mut Self {
        self.set_origins(val);
        self
    }
    #[deprecated = "Use `set_since()` instead."]
    pub fn since(&mut self, val: f64) -> &mut Self {
        self.set_since(val);
        self
    }
}
impl Default for RemovalOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DataTypeSet")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A set of data types. Missing data types are interpreted as false.
    pub type DataTypeSet;
    ///Get the `appcache` field of this object.
    #[wasm_bindgen(method, getter = "appcache")]
    pub fn get_appcache(this: &DataTypeSet) -> Option<bool>;
    ///Change the `appcache` field of this object.
    #[wasm_bindgen(method, setter = "appcache")]
    pub fn set_appcache(this: &DataTypeSet, val: bool);
    ///Get the `cache` field of this object.
    #[wasm_bindgen(method, getter = "cache")]
    pub fn get_cache(this: &DataTypeSet) -> Option<bool>;
    ///Change the `cache` field of this object.
    #[wasm_bindgen(method, setter = "cache")]
    pub fn set_cache(this: &DataTypeSet, val: bool);
    ///Get the `cacheStorage` field of this object.
    #[wasm_bindgen(method, getter = "cacheStorage")]
    pub fn get_cache_storage(this: &DataTypeSet) -> Option<bool>;
    ///Change the `cacheStorage` field of this object.
    #[wasm_bindgen(method, setter = "cacheStorage")]
    pub fn set_cache_storage(this: &DataTypeSet, val: bool);
    ///Get the `cookies` field of this object.
    #[wasm_bindgen(method, getter = "cookies")]
    pub fn get_cookies(this: &DataTypeSet) -> Option<bool>;
    ///Change the `cookies` field of this object.
    #[wasm_bindgen(method, setter = "cookies")]
    pub fn set_cookies(this: &DataTypeSet, val: bool);
    ///Get the `downloads` field of this object.
    #[wasm_bindgen(method, getter = "downloads")]
    pub fn get_downloads(this: &DataTypeSet) -> Option<bool>;
    ///Change the `downloads` field of this object.
    #[wasm_bindgen(method, setter = "downloads")]
    pub fn set_downloads(this: &DataTypeSet, val: bool);
    ///Get the `fileSystems` field of this object.
    #[wasm_bindgen(method, getter = "fileSystems")]
    pub fn get_file_systems(this: &DataTypeSet) -> Option<bool>;
    ///Change the `fileSystems` field of this object.
    #[wasm_bindgen(method, setter = "fileSystems")]
    pub fn set_file_systems(this: &DataTypeSet, val: bool);
    ///Get the `formData` field of this object.
    #[wasm_bindgen(method, getter = "formData")]
    pub fn get_form_data(this: &DataTypeSet) -> Option<bool>;
    ///Change the `formData` field of this object.
    #[wasm_bindgen(method, setter = "formData")]
    pub fn set_form_data(this: &DataTypeSet, val: bool);
    ///Get the `history` field of this object.
    #[wasm_bindgen(method, getter = "history")]
    pub fn get_history(this: &DataTypeSet) -> Option<bool>;
    ///Change the `history` field of this object.
    #[wasm_bindgen(method, setter = "history")]
    pub fn set_history(this: &DataTypeSet, val: bool);
    ///Get the `indexedDB` field of this object.
    #[wasm_bindgen(method, getter = "indexedDB")]
    pub fn get_indexed_db(this: &DataTypeSet) -> Option<bool>;
    ///Change the `indexedDB` field of this object.
    #[wasm_bindgen(method, setter = "indexedDB")]
    pub fn set_indexed_db(this: &DataTypeSet, val: bool);
    ///Get the `localStorage` field of this object.
    #[wasm_bindgen(method, getter = "localStorage")]
    pub fn get_local_storage(this: &DataTypeSet) -> Option<bool>;
    ///Change the `localStorage` field of this object.
    #[wasm_bindgen(method, setter = "localStorage")]
    pub fn set_local_storage(this: &DataTypeSet, val: bool);
    ///Get the `passwords` field of this object.
    #[wasm_bindgen(method, getter = "passwords")]
    pub fn get_passwords(this: &DataTypeSet) -> Option<bool>;
    ///Change the `passwords` field of this object.
    #[wasm_bindgen(method, setter = "passwords")]
    pub fn set_passwords(this: &DataTypeSet, val: bool);
    ///Get the `pluginData` field of this object.
    #[wasm_bindgen(method, getter = "pluginData")]
    pub fn get_plugin_data(this: &DataTypeSet) -> Option<bool>;
    ///Change the `pluginData` field of this object.
    #[wasm_bindgen(method, setter = "pluginData")]
    pub fn set_plugin_data(this: &DataTypeSet, val: bool);
    ///Get the `serverBoundCertificates` field of this object.
    #[wasm_bindgen(method, getter = "serverBoundCertificates")]
    pub fn get_server_bound_certificates(this: &DataTypeSet) -> Option<bool>;
    ///Change the `serverBoundCertificates` field of this object.
    #[wasm_bindgen(method, setter = "serverBoundCertificates")]
    pub fn set_server_bound_certificates(this: &DataTypeSet, val: bool);
    ///Get the `serviceWorkers` field of this object.
    #[wasm_bindgen(method, getter = "serviceWorkers")]
    pub fn get_service_workers(this: &DataTypeSet) -> Option<bool>;
    ///Change the `serviceWorkers` field of this object.
    #[wasm_bindgen(method, setter = "serviceWorkers")]
    pub fn set_service_workers(this: &DataTypeSet, val: bool);
    ///Get the `webSQL` field of this object.
    #[wasm_bindgen(method, getter = "webSQL")]
    pub fn get_web_sql(this: &DataTypeSet) -> Option<bool>;
    ///Change the `webSQL` field of this object.
    #[wasm_bindgen(method, setter = "webSQL")]
    pub fn set_web_sql(this: &DataTypeSet, val: bool);
}
impl DataTypeSet {
    ///Construct a new `DataTypeSet`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_appcache()` instead."]
    pub fn appcache(&mut self, val: bool) -> &mut Self {
        self.set_appcache(val);
        self
    }
    #[deprecated = "Use `set_cache()` instead."]
    pub fn cache(&mut self, val: bool) -> &mut Self {
        self.set_cache(val);
        self
    }
    #[deprecated = "Use `set_cache_storage()` instead."]
    pub fn cache_storage(&mut self, val: bool) -> &mut Self {
        self.set_cache_storage(val);
        self
    }
    #[deprecated = "Use `set_cookies()` instead."]
    pub fn cookies(&mut self, val: bool) -> &mut Self {
        self.set_cookies(val);
        self
    }
    #[deprecated = "Use `set_downloads()` instead."]
    pub fn downloads(&mut self, val: bool) -> &mut Self {
        self.set_downloads(val);
        self
    }
    #[deprecated = "Use `set_file_systems()` instead."]
    pub fn file_systems(&mut self, val: bool) -> &mut Self {
        self.set_file_systems(val);
        self
    }
    #[deprecated = "Use `set_form_data()` instead."]
    pub fn form_data(&mut self, val: bool) -> &mut Self {
        self.set_form_data(val);
        self
    }
    #[deprecated = "Use `set_history()` instead."]
    pub fn history(&mut self, val: bool) -> &mut Self {
        self.set_history(val);
        self
    }
    #[deprecated = "Use `set_indexed_db()` instead."]
    pub fn indexed_db(&mut self, val: bool) -> &mut Self {
        self.set_indexed_db(val);
        self
    }
    #[deprecated = "Use `set_local_storage()` instead."]
    pub fn local_storage(&mut self, val: bool) -> &mut Self {
        self.set_local_storage(val);
        self
    }
    #[deprecated = "Use `set_passwords()` instead."]
    pub fn passwords(&mut self, val: bool) -> &mut Self {
        self.set_passwords(val);
        self
    }
    #[deprecated = "Use `set_plugin_data()` instead."]
    pub fn plugin_data(&mut self, val: bool) -> &mut Self {
        self.set_plugin_data(val);
        self
    }
    #[deprecated = "Use `set_server_bound_certificates()` instead."]
    pub fn server_bound_certificates(&mut self, val: bool) -> &mut Self {
        self.set_server_bound_certificates(val);
        self
    }
    #[deprecated = "Use `set_service_workers()` instead."]
    pub fn service_workers(&mut self, val: bool) -> &mut Self {
        self.set_service_workers(val);
        self
    }
    #[deprecated = "Use `set_web_sql()` instead."]
    pub fn web_sql(&mut self, val: bool) -> &mut Self {
        self.set_web_sql(val);
        self
    }
}
impl Default for DataTypeSet {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Reports which types of data are currently selected in the 'Clear browsing data' settings UI. Note: some of the data types included in this API are not available in the settings UI, and some UI settings control more than one data type listed here.
    #[wasm_bindgen(js_namespace = ["chrome", "browsingData"], js_name = "settings")]
    pub fn settings() -> Promise;
    ///Clears various types of browsing data stored in a user's profile.
    #[wasm_bindgen(js_namespace = ["chrome", "browsingData"], js_name = "remove")]
    pub fn remove(options: RemovalOptions, data_to_remove: DataTypeSet) -> Promise;
    ///Clears websites' appcache data.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "browsingData"],
        js_name = "removeAppcache"
    )]
    pub fn remove_appcache(options: RemovalOptions) -> Promise;
    ///Clears the browser's cache.
    #[wasm_bindgen(js_namespace = ["chrome", "browsingData"], js_name = "removeCache")]
    pub fn remove_cache(options: RemovalOptions) -> Promise;
    ///Clears websites' cache storage data.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "browsingData"],
        js_name = "removeCacheStorage"
    )]
    pub fn remove_cache_storage(options: RemovalOptions) -> Promise;
    ///Clears the browser's cookies and server-bound certificates modified within a particular timeframe.
    #[wasm_bindgen(js_namespace = ["chrome", "browsingData"], js_name = "removeCookies")]
    pub fn remove_cookies(options: RemovalOptions) -> Promise;
    ///Clears the browser's list of downloaded files (not the downloaded files themselves).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "browsingData"],
        js_name = "removeDownloads"
    )]
    pub fn remove_downloads(options: RemovalOptions) -> Promise;
    ///Clears websites' file system data.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "browsingData"],
        js_name = "removeFileSystems"
    )]
    pub fn remove_file_systems(options: RemovalOptions) -> Promise;
    ///Clears the browser's stored form data (autofill).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "browsingData"],
        js_name = "removeFormData"
    )]
    pub fn remove_form_data(options: RemovalOptions) -> Promise;
    ///Clears the browser's history.
    #[wasm_bindgen(js_namespace = ["chrome", "browsingData"], js_name = "removeHistory")]
    pub fn remove_history(options: RemovalOptions) -> Promise;
    ///Clears websites' IndexedDB data.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "browsingData"],
        js_name = "removeIndexedDB"
    )]
    pub fn remove_indexed_db(options: RemovalOptions) -> Promise;
    ///Clears websites' local storage data.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "browsingData"],
        js_name = "removeLocalStorage"
    )]
    pub fn remove_local_storage(options: RemovalOptions) -> Promise;
    ///Clears plugins' data.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "browsingData"],
        js_name = "removePluginData"
    )]
    pub fn remove_plugin_data(options: RemovalOptions) -> Promise;
    ///Clears the browser's stored passwords.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "browsingData"],
        js_name = "removePasswords"
    )]
    pub fn remove_passwords(options: RemovalOptions) -> Promise;
    ///Clears websites' service workers.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "browsingData"],
        js_name = "removeServiceWorkers"
    )]
    pub fn remove_service_workers(options: RemovalOptions) -> Promise;
    ///Clears websites' WebSQL data.
    #[wasm_bindgen(js_namespace = ["chrome", "browsingData"], js_name = "removeWebSQL")]
    pub fn remove_web_sql(options: RemovalOptions) -> Promise;
}
