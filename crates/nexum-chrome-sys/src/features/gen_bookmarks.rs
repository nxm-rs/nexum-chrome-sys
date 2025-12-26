#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///Indicates the type of folder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FolderType {
    ///The folder whose contents is displayed at the top of the browser window.
    BookmarksBar = "bookmarks-bar",
    ///Bookmarks which are displayed in the full list of bookmarks on all platforms.
    Other = "other",
    ///Bookmarks generally available on the user's mobile devices, but modifiable by extension or in the bookmarks manager.
    Mobile = "mobile",
    ///A top-level folder that may be present if the system administrator or the custodian of a supervised user has configured bookmarks.
    Managed = "managed",
}
#[wasm_bindgen]
///Indicates the reason why this node is unmodifiable. The managed value indicates that this node was configured by the system administrator. Omitted if the node can be modified by the user and the extension (default).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BookmarkTreeNodeUnmodifiable {
    Managed = "managed",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "BookmarkTreeNode")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A node (either a bookmark or a folder) in the bookmark tree. Child nodes are ordered within their parent folder.
    pub type BookmarkTreeNode;
    ///Get the `parentId` field of this object.
    #[wasm_bindgen(method, getter = "parentId")]
    pub fn get_parent_id(this: &BookmarkTreeNode) -> Option<String>;
    ///Change the `parentId` field of this object.
    #[wasm_bindgen(method, setter = "parentId")]
    pub fn set_parent_id(this: &BookmarkTreeNode, val: String);
    ///Get the `dateLastUsed` field of this object.
    #[wasm_bindgen(method, getter = "dateLastUsed")]
    pub fn get_date_last_used(this: &BookmarkTreeNode) -> Option<f64>;
    ///Change the `dateLastUsed` field of this object.
    #[wasm_bindgen(method, setter = "dateLastUsed")]
    pub fn set_date_last_used(this: &BookmarkTreeNode, val: f64);
    ///Get the `folderType` field of this object.
    #[wasm_bindgen(method, getter = "folderType")]
    pub fn get_folder_type(this: &BookmarkTreeNode) -> Option<FolderType>;
    ///Change the `folderType` field of this object.
    #[wasm_bindgen(method, setter = "folderType")]
    pub fn set_folder_type(this: &BookmarkTreeNode, val: FolderType);
    ///Get the `syncing` field of this object.
    #[wasm_bindgen(method, getter = "syncing")]
    pub fn get_syncing(this: &BookmarkTreeNode) -> bool;
    ///Change the `syncing` field of this object.
    #[wasm_bindgen(method, setter = "syncing")]
    pub fn set_syncing(this: &BookmarkTreeNode, val: bool);
    ///Get the `children` field of this object.
    #[wasm_bindgen(method, getter = "children")]
    pub fn get_children(this: &BookmarkTreeNode) -> Option<Array>;
    ///Change the `children` field of this object.
    #[wasm_bindgen(method, setter = "children")]
    pub fn set_children(this: &BookmarkTreeNode, val: &Array);
    ///Get the `unmodifiable` field of this object.
    #[wasm_bindgen(method, getter = "unmodifiable")]
    pub fn get_unmodifiable(this: &BookmarkTreeNode) -> Option<BookmarkTreeNodeUnmodifiable>;
    ///Change the `unmodifiable` field of this object.
    #[wasm_bindgen(method, setter = "unmodifiable")]
    pub fn set_unmodifiable(this: &BookmarkTreeNode, val: BookmarkTreeNodeUnmodifiable);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &BookmarkTreeNode) -> Option<String>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &BookmarkTreeNode, val: String);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &BookmarkTreeNode) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &BookmarkTreeNode, val: String);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &BookmarkTreeNode) -> String;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &BookmarkTreeNode, val: String);
    ///Get the `dateAdded` field of this object.
    #[wasm_bindgen(method, getter = "dateAdded")]
    pub fn get_date_added(this: &BookmarkTreeNode) -> Option<f64>;
    ///Change the `dateAdded` field of this object.
    #[wasm_bindgen(method, setter = "dateAdded")]
    pub fn set_date_added(this: &BookmarkTreeNode, val: f64);
    ///Get the `dateGroupModified` field of this object.
    #[wasm_bindgen(method, getter = "dateGroupModified")]
    pub fn get_date_group_modified(this: &BookmarkTreeNode) -> Option<f64>;
    ///Change the `dateGroupModified` field of this object.
    #[wasm_bindgen(method, setter = "dateGroupModified")]
    pub fn set_date_group_modified(this: &BookmarkTreeNode, val: f64);
    ///Get the `index` field of this object.
    #[wasm_bindgen(method, getter = "index")]
    pub fn get_index(this: &BookmarkTreeNode) -> Option<i32>;
    ///Change the `index` field of this object.
    #[wasm_bindgen(method, setter = "index")]
    pub fn set_index(this: &BookmarkTreeNode, val: i32);
}
impl BookmarkTreeNode {
    ///Construct a new `BookmarkTreeNode`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_parent_id()` instead."]
    pub fn parent_id(&mut self, val: String) -> &mut Self {
        self.set_parent_id(val);
        self
    }
    #[deprecated = "Use `set_date_last_used()` instead."]
    pub fn date_last_used(&mut self, val: f64) -> &mut Self {
        self.set_date_last_used(val);
        self
    }
    #[deprecated = "Use `set_folder_type()` instead."]
    pub fn folder_type(&mut self, val: FolderType) -> &mut Self {
        self.set_folder_type(val);
        self
    }
    #[deprecated = "Use `set_syncing()` instead."]
    pub fn syncing(&mut self, val: bool) -> &mut Self {
        self.set_syncing(val);
        self
    }
    #[deprecated = "Use `set_children()` instead."]
    pub fn children(&mut self, val: &Array) -> &mut Self {
        self.set_children(val);
        self
    }
    #[deprecated = "Use `set_unmodifiable()` instead."]
    pub fn unmodifiable(&mut self, val: BookmarkTreeNodeUnmodifiable) -> &mut Self {
        self.set_unmodifiable(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[deprecated = "Use `set_date_added()` instead."]
    pub fn date_added(&mut self, val: f64) -> &mut Self {
        self.set_date_added(val);
        self
    }
    #[deprecated = "Use `set_date_group_modified()` instead."]
    pub fn date_group_modified(&mut self, val: f64) -> &mut Self {
        self.set_date_group_modified(val);
        self
    }
    #[deprecated = "Use `set_index()` instead."]
    pub fn index(&mut self, val: i32) -> &mut Self {
        self.set_index(val);
        self
    }
}
impl Default for BookmarkTreeNode {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CreateDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Object passed to the create() function.
    pub type CreateDetails;
    ///Get the `index` field of this object.
    #[wasm_bindgen(method, getter = "index")]
    pub fn get_index(this: &CreateDetails) -> Option<i32>;
    ///Change the `index` field of this object.
    #[wasm_bindgen(method, setter = "index")]
    pub fn set_index(this: &CreateDetails, val: i32);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &CreateDetails) -> Option<String>;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &CreateDetails, val: String);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &CreateDetails) -> Option<String>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &CreateDetails, val: String);
    ///Get the `parentId` field of this object.
    #[wasm_bindgen(method, getter = "parentId")]
    pub fn get_parent_id(this: &CreateDetails) -> Option<String>;
    ///Change the `parentId` field of this object.
    #[wasm_bindgen(method, setter = "parentId")]
    pub fn set_parent_id(this: &CreateDetails, val: String);
}
impl CreateDetails {
    ///Construct a new `CreateDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_index()` instead."]
    pub fn index(&mut self, val: i32) -> &mut Self {
        self.set_index(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
    #[deprecated = "Use `set_parent_id()` instead."]
    pub fn parent_id(&mut self, val: String) -> &mut Self {
        self.set_parent_id(val);
        self
    }
}
impl Default for CreateDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Retrieves the specified BookmarkTreeNode(s).
    #[wasm_bindgen(js_namespace = ["chrome", "bookmarks"], js_name = "get")]
    pub fn get(id_or_id_list: JsValue) -> Promise;
    ///Retrieves the children of the specified BookmarkTreeNode id.
    #[wasm_bindgen(js_namespace = ["chrome", "bookmarks"], js_name = "getChildren")]
    pub fn get_children(id: String) -> Promise;
    ///Retrieves the recently added bookmarks.
    #[wasm_bindgen(js_namespace = ["chrome", "bookmarks"], js_name = "getRecent")]
    pub fn get_recent(number_of_items: i32) -> Promise;
    ///Retrieves the entire Bookmarks hierarchy.
    #[wasm_bindgen(js_namespace = ["chrome", "bookmarks"], js_name = "getTree")]
    pub fn get_tree() -> Promise;
    ///Retrieves part of the Bookmarks hierarchy, starting at the specified node.
    #[wasm_bindgen(js_namespace = ["chrome", "bookmarks"], js_name = "getSubTree")]
    pub fn get_sub_tree(id: String) -> Promise;
    ///Searches for BookmarkTreeNodes matching the given query. Queries specified with an object produce BookmarkTreeNodes matching all specified properties.
    #[wasm_bindgen(js_namespace = ["chrome", "bookmarks"], js_name = "search")]
    pub fn search(query: JsValue) -> Promise;
    ///Creates a bookmark or folder under the specified parentId. If url is NULL or missing, it will be a folder.
    #[wasm_bindgen(js_namespace = ["chrome", "bookmarks"], js_name = "create")]
    pub fn create(bookmark: CreateDetails) -> Promise;
    ///Moves the specified BookmarkTreeNode to the provided location.
    #[wasm_bindgen(js_namespace = ["chrome", "bookmarks"], js_name = "move")]
    pub fn r#move(id: String, destination: Object) -> Promise;
    ///Updates the properties of a bookmark or folder. Specify only the properties that you want to change; unspecified properties will be left unchanged. Note: Currently, only 'title' and 'url' are supported.
    #[wasm_bindgen(js_namespace = ["chrome", "bookmarks"], js_name = "update")]
    pub fn update(id: String, changes: Object) -> Promise;
    ///Removes a bookmark or an empty bookmark folder.
    #[wasm_bindgen(js_namespace = ["chrome", "bookmarks"], js_name = "remove")]
    pub fn remove(id: String) -> Promise;
    ///Recursively removes a bookmark folder.
    #[wasm_bindgen(js_namespace = ["chrome", "bookmarks"], js_name = "removeTree")]
    pub fn remove_tree(id: String) -> Promise;
    ///Fired when a bookmark or folder is created.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bookmarks",
        "onCreated"],
        js_name = "addListener"
    )]
    pub fn on_created_add_listener(callback: &Function);
    ///Fired when a bookmark or folder is removed. When a folder is removed recursively, a single notification is fired for the folder, and none for its contents.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bookmarks",
        "onRemoved"],
        js_name = "addListener"
    )]
    pub fn on_removed_add_listener(callback: &Function);
    ///Fired when a bookmark or folder changes. Note: Currently, only title and url changes trigger this.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bookmarks",
        "onChanged"],
        js_name = "addListener"
    )]
    pub fn on_changed_add_listener(callback: &Function);
    ///Fired when a bookmark or folder is moved to a different parent folder.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bookmarks",
        "onMoved"],
        js_name = "addListener"
    )]
    pub fn on_moved_add_listener(callback: &Function);
    ///Fired when the children of a folder have changed their order due to the order being sorted in the UI. This is not called as a result of a move().
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bookmarks",
        "onChildrenReordered"],
        js_name = "addListener"
    )]
    pub fn on_children_reordered_add_listener(callback: &Function);
    ///Fired when a bookmark import session is begun. Expensive observers should ignore onCreated updates until onImportEnded is fired. Observers should still handle other notifications immediately.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bookmarks",
        "onImportBegan"],
        js_name = "addListener"
    )]
    pub fn on_import_began_add_listener(callback: &Function);
    ///Fired when a bookmark import session is ended.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bookmarks",
        "onImportEnded"],
        js_name = "addListener"
    )]
    pub fn on_import_ended_add_listener(callback: &Function);
}
