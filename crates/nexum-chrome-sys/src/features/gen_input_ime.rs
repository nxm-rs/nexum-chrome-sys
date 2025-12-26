#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardEventType {
    Keyup = "keyup",
    Keydown = "keydown",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "KeyboardEvent")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///See http://www.w3.org/TR/DOM-Level-3-Events/#events-KeyboardEvent
    pub type KeyboardEvent;
    ///Get the `key` field of this object.
    #[wasm_bindgen(method, getter = "key")]
    pub fn get_key(this: &KeyboardEvent) -> String;
    ///Change the `key` field of this object.
    #[wasm_bindgen(method, setter = "key")]
    pub fn set_key(this: &KeyboardEvent, val: String);
    ///Get the `code` field of this object.
    #[wasm_bindgen(method, getter = "code")]
    pub fn get_code(this: &KeyboardEvent) -> String;
    ///Change the `code` field of this object.
    #[wasm_bindgen(method, setter = "code")]
    pub fn set_code(this: &KeyboardEvent, val: String);
    ///Get the `shiftKey` field of this object.
    #[wasm_bindgen(method, getter = "shiftKey")]
    pub fn get_shift_key(this: &KeyboardEvent) -> Option<bool>;
    ///Change the `shiftKey` field of this object.
    #[wasm_bindgen(method, setter = "shiftKey")]
    pub fn set_shift_key(this: &KeyboardEvent, val: bool);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &KeyboardEvent) -> Option<String>;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &KeyboardEvent, val: String);
    ///Get the `ctrlKey` field of this object.
    #[wasm_bindgen(method, getter = "ctrlKey")]
    pub fn get_ctrl_key(this: &KeyboardEvent) -> Option<bool>;
    ///Change the `ctrlKey` field of this object.
    #[wasm_bindgen(method, setter = "ctrlKey")]
    pub fn set_ctrl_key(this: &KeyboardEvent, val: bool);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &KeyboardEvent) -> KeyboardEventType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &KeyboardEvent, val: KeyboardEventType);
    ///Get the `capsLock` field of this object.
    #[wasm_bindgen(method, getter = "capsLock")]
    pub fn get_caps_lock(this: &KeyboardEvent) -> Option<bool>;
    ///Change the `capsLock` field of this object.
    #[wasm_bindgen(method, setter = "capsLock")]
    pub fn set_caps_lock(this: &KeyboardEvent, val: bool);
    ///Get the `keyCode` field of this object.
    #[wasm_bindgen(method, getter = "keyCode")]
    pub fn get_key_code(this: &KeyboardEvent) -> Option<i32>;
    ///Change the `keyCode` field of this object.
    #[wasm_bindgen(method, setter = "keyCode")]
    pub fn set_key_code(this: &KeyboardEvent, val: i32);
    ///Get the `altKey` field of this object.
    #[wasm_bindgen(method, getter = "altKey")]
    pub fn get_alt_key(this: &KeyboardEvent) -> Option<bool>;
    ///Change the `altKey` field of this object.
    #[wasm_bindgen(method, setter = "altKey")]
    pub fn set_alt_key(this: &KeyboardEvent, val: bool);
    ///Get the `extensionId` field of this object.
    #[wasm_bindgen(method, getter = "extensionId")]
    pub fn get_extension_id(this: &KeyboardEvent) -> Option<String>;
    ///Change the `extensionId` field of this object.
    #[wasm_bindgen(method, setter = "extensionId")]
    pub fn set_extension_id(this: &KeyboardEvent, val: String);
    ///Get the `altgrKey` field of this object.
    #[wasm_bindgen(method, getter = "altgrKey")]
    pub fn get_altgr_key(this: &KeyboardEvent) -> Option<bool>;
    ///Change the `altgrKey` field of this object.
    #[wasm_bindgen(method, setter = "altgrKey")]
    pub fn set_altgr_key(this: &KeyboardEvent, val: bool);
}
impl KeyboardEvent {
    ///Construct a new `KeyboardEvent`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_key()` instead."]
    pub fn key(&mut self, val: String) -> &mut Self {
        self.set_key(val);
        self
    }
    #[deprecated = "Use `set_code()` instead."]
    pub fn code(&mut self, val: String) -> &mut Self {
        self.set_code(val);
        self
    }
    #[deprecated = "Use `set_shift_key()` instead."]
    pub fn shift_key(&mut self, val: bool) -> &mut Self {
        self.set_shift_key(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: String) -> &mut Self {
        self.set_request_id(val);
        self
    }
    #[deprecated = "Use `set_ctrl_key()` instead."]
    pub fn ctrl_key(&mut self, val: bool) -> &mut Self {
        self.set_ctrl_key(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: KeyboardEventType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_caps_lock()` instead."]
    pub fn caps_lock(&mut self, val: bool) -> &mut Self {
        self.set_caps_lock(val);
        self
    }
    #[deprecated = "Use `set_key_code()` instead."]
    pub fn key_code(&mut self, val: i32) -> &mut Self {
        self.set_key_code(val);
        self
    }
    #[deprecated = "Use `set_alt_key()` instead."]
    pub fn alt_key(&mut self, val: bool) -> &mut Self {
        self.set_alt_key(val);
        self
    }
    #[deprecated = "Use `set_extension_id()` instead."]
    pub fn extension_id(&mut self, val: String) -> &mut Self {
        self.set_extension_id(val);
        self
    }
    #[deprecated = "Use `set_altgr_key()` instead."]
    pub fn altgr_key(&mut self, val: bool) -> &mut Self {
        self.set_altgr_key(val);
        self
    }
}
impl Default for KeyboardEvent {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///Type of value this text field edits, (Text, Number, URL, etc)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputContextType {
    Text = "text",
    Search = "search",
    Tel = "tel",
    Url = "url",
    Email = "email",
    Number = "number",
    Password = "password",
    Null = "null",
}
#[wasm_bindgen]
///The auto-capitalize type of the text field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoCapitalizeType {
    Characters = "characters",
    Words = "words",
    Sentences = "sentences",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "InputContext")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Describes an input Context
    pub type InputContext;
    ///Get the `shouldDoLearning` field of this object.
    #[wasm_bindgen(method, getter = "shouldDoLearning")]
    pub fn get_should_do_learning(this: &InputContext) -> bool;
    ///Change the `shouldDoLearning` field of this object.
    #[wasm_bindgen(method, setter = "shouldDoLearning")]
    pub fn set_should_do_learning(this: &InputContext, val: bool);
    ///Get the `spellCheck` field of this object.
    #[wasm_bindgen(method, getter = "spellCheck")]
    pub fn get_spell_check(this: &InputContext) -> bool;
    ///Change the `spellCheck` field of this object.
    #[wasm_bindgen(method, setter = "spellCheck")]
    pub fn set_spell_check(this: &InputContext, val: bool);
    ///Get the `autoCorrect` field of this object.
    #[wasm_bindgen(method, getter = "autoCorrect")]
    pub fn get_auto_correct(this: &InputContext) -> bool;
    ///Change the `autoCorrect` field of this object.
    #[wasm_bindgen(method, setter = "autoCorrect")]
    pub fn set_auto_correct(this: &InputContext, val: bool);
    ///Get the `contextID` field of this object.
    #[wasm_bindgen(method, getter = "contextID")]
    pub fn get_context_id(this: &InputContext) -> i32;
    ///Change the `contextID` field of this object.
    #[wasm_bindgen(method, setter = "contextID")]
    pub fn set_context_id(this: &InputContext, val: i32);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &InputContext) -> InputContextType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &InputContext, val: InputContextType);
    ///Get the `autoCapitalize` field of this object.
    #[wasm_bindgen(method, getter = "autoCapitalize")]
    pub fn get_auto_capitalize(this: &InputContext) -> AutoCapitalizeType;
    ///Change the `autoCapitalize` field of this object.
    #[wasm_bindgen(method, setter = "autoCapitalize")]
    pub fn set_auto_capitalize(this: &InputContext, val: AutoCapitalizeType);
    ///Get the `autoComplete` field of this object.
    #[wasm_bindgen(method, getter = "autoComplete")]
    pub fn get_auto_complete(this: &InputContext) -> bool;
    ///Change the `autoComplete` field of this object.
    #[wasm_bindgen(method, setter = "autoComplete")]
    pub fn set_auto_complete(this: &InputContext, val: bool);
}
impl InputContext {
    ///Construct a new `InputContext`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_should_do_learning()` instead."]
    pub fn should_do_learning(&mut self, val: bool) -> &mut Self {
        self.set_should_do_learning(val);
        self
    }
    #[deprecated = "Use `set_spell_check()` instead."]
    pub fn spell_check(&mut self, val: bool) -> &mut Self {
        self.set_spell_check(val);
        self
    }
    #[deprecated = "Use `set_auto_correct()` instead."]
    pub fn auto_correct(&mut self, val: bool) -> &mut Self {
        self.set_auto_correct(val);
        self
    }
    #[deprecated = "Use `set_context_id()` instead."]
    pub fn context_id(&mut self, val: i32) -> &mut Self {
        self.set_context_id(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: InputContextType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_auto_capitalize()` instead."]
    pub fn auto_capitalize(&mut self, val: AutoCapitalizeType) -> &mut Self {
        self.set_auto_capitalize(val);
        self
    }
    #[deprecated = "Use `set_auto_complete()` instead."]
    pub fn auto_complete(&mut self, val: bool) -> &mut Self {
        self.set_auto_complete(val);
        self
    }
}
impl Default for InputContext {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///The type of menu item. Radio buttons between separators are considered grouped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItemStyle {
    Check = "check",
    Radio = "radio",
    Separator = "separator",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MenuItem")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A menu item used by an input method to interact with the user from the language menu.
    pub type MenuItem;
    ///Get the `label` field of this object.
    #[wasm_bindgen(method, getter = "label")]
    pub fn get_label(this: &MenuItem) -> Option<String>;
    ///Change the `label` field of this object.
    #[wasm_bindgen(method, setter = "label")]
    pub fn set_label(this: &MenuItem, val: String);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &MenuItem) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &MenuItem, val: String);
    ///Get the `style` field of this object.
    #[wasm_bindgen(method, getter = "style")]
    pub fn get_style(this: &MenuItem) -> Option<MenuItemStyle>;
    ///Change the `style` field of this object.
    #[wasm_bindgen(method, setter = "style")]
    pub fn set_style(this: &MenuItem, val: MenuItemStyle);
    ///Get the `visible` field of this object.
    #[wasm_bindgen(method, getter = "visible")]
    pub fn get_visible(this: &MenuItem) -> Option<bool>;
    ///Change the `visible` field of this object.
    #[wasm_bindgen(method, setter = "visible")]
    pub fn set_visible(this: &MenuItem, val: bool);
    ///Get the `checked` field of this object.
    #[wasm_bindgen(method, getter = "checked")]
    pub fn get_checked(this: &MenuItem) -> Option<bool>;
    ///Change the `checked` field of this object.
    #[wasm_bindgen(method, setter = "checked")]
    pub fn set_checked(this: &MenuItem, val: bool);
    ///Get the `enabled` field of this object.
    #[wasm_bindgen(method, getter = "enabled")]
    pub fn get_enabled(this: &MenuItem) -> Option<bool>;
    ///Change the `enabled` field of this object.
    #[wasm_bindgen(method, setter = "enabled")]
    pub fn set_enabled(this: &MenuItem, val: bool);
}
impl MenuItem {
    ///Construct a new `MenuItem`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_label()` instead."]
    pub fn label(&mut self, val: String) -> &mut Self {
        self.set_label(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_style()` instead."]
    pub fn style(&mut self, val: MenuItemStyle) -> &mut Self {
        self.set_style(val);
        self
    }
    #[deprecated = "Use `set_visible()` instead."]
    pub fn visible(&mut self, val: bool) -> &mut Self {
        self.set_visible(val);
        self
    }
    #[deprecated = "Use `set_checked()` instead."]
    pub fn checked(&mut self, val: bool) -> &mut Self {
        self.set_checked(val);
        self
    }
    #[deprecated = "Use `set_enabled()` instead."]
    pub fn enabled(&mut self, val: bool) -> &mut Self {
        self.set_enabled(val);
        self
    }
}
impl Default for MenuItem {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///The type of the underline to modify this segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnderlineStyle {
    Underline = "underline",
    DoubleUnderline = "doubleUnderline",
    NoUnderline = "noUnderline",
}
#[wasm_bindgen]
///Where to display the candidate window. If set to 'cursor', the window follows the cursor. If set to 'composition', the window is locked to the beginning of the composition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowPosition {
    Cursor = "cursor",
    Composition = "composition",
}
#[wasm_bindgen]
///The screen type under which the IME is activated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenType {
    Normal = "normal",
    Login = "login",
    Lock = "lock",
    SecondaryLogin = "secondary-login",
}
#[wasm_bindgen]
///Which mouse buttons was clicked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left = "left",
    Middle = "middle",
    Right = "right",
}
#[wasm_bindgen]
///Type of assistive window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssistiveWindowType {
    Undo = "undo",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AssistiveWindowProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Properties of the assistive window.
    pub type AssistiveWindowProperties;
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &AssistiveWindowProperties) -> AssistiveWindowType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &AssistiveWindowProperties, val: AssistiveWindowType);
    ///Get the `visible` field of this object.
    #[wasm_bindgen(method, getter = "visible")]
    pub fn get_visible(this: &AssistiveWindowProperties) -> bool;
    ///Change the `visible` field of this object.
    #[wasm_bindgen(method, setter = "visible")]
    pub fn set_visible(this: &AssistiveWindowProperties, val: bool);
    ///Get the `announceString` field of this object.
    #[wasm_bindgen(method, getter = "announceString")]
    pub fn get_announce_string(this: &AssistiveWindowProperties) -> Option<String>;
    ///Change the `announceString` field of this object.
    #[wasm_bindgen(method, setter = "announceString")]
    pub fn set_announce_string(this: &AssistiveWindowProperties, val: String);
}
impl AssistiveWindowProperties {
    ///Construct a new `AssistiveWindowProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: AssistiveWindowType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_visible()` instead."]
    pub fn visible(&mut self, val: bool) -> &mut Self {
        self.set_visible(val);
        self
    }
    #[deprecated = "Use `set_announce_string()` instead."]
    pub fn announce_string(&mut self, val: String) -> &mut Self {
        self.set_announce_string(val);
        self
    }
}
impl Default for AssistiveWindowProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///ID of buttons in assistive window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssistiveWindowButton {
    Undo = "undo",
    AddToDictionary = "addToDictionary",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MenuParameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MenuParameters;
    ///Get the `engineID` field of this object.
    #[wasm_bindgen(method, getter = "engineID")]
    pub fn get_engine_id(this: &MenuParameters) -> String;
    ///Change the `engineID` field of this object.
    #[wasm_bindgen(method, setter = "engineID")]
    pub fn set_engine_id(this: &MenuParameters, val: String);
    ///Get the `items` field of this object.
    #[wasm_bindgen(method, getter = "items")]
    pub fn get_items(this: &MenuParameters) -> Array;
    ///Change the `items` field of this object.
    #[wasm_bindgen(method, setter = "items")]
    pub fn set_items(this: &MenuParameters, val: &Array);
}
impl MenuParameters {
    ///Construct a new `MenuParameters`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_engine_id()` instead."]
    pub fn engine_id(&mut self, val: String) -> &mut Self {
        self.set_engine_id(val);
        self
    }
    #[deprecated = "Use `set_items()` instead."]
    pub fn items(&mut self, val: &Array) -> &mut Self {
        self.set_items(val);
        self
    }
}
impl Default for MenuParameters {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Set the current composition. If this extension does not own the active IME, this fails.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime"],
        js_name = "setComposition"
    )]
    pub fn set_composition(parameters: Object) -> Promise;
    ///Clear the current composition. If this extension does not own the active IME, this fails.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime"],
        js_name = "clearComposition"
    )]
    pub fn clear_composition(parameters: Object) -> Promise;
    ///Commits the provided text to the current input.
    #[wasm_bindgen(js_namespace = ["chrome", "input", "ime"], js_name = "commitText")]
    pub fn commit_text(parameters: Object) -> Promise;
    ///Sends the key events. This function is expected to be used by virtual keyboards. When key(s) on a virtual keyboard is pressed by a user, this function is used to propagate that event to the system.
    #[wasm_bindgen(js_namespace = ["chrome", "input", "ime"], js_name = "sendKeyEvents")]
    pub fn send_key_events(parameters: Object) -> Promise;
    ///Hides the input view window, which is popped up automatically by system. If the input view window is already hidden, this function will do nothing.
    #[wasm_bindgen(js_namespace = ["chrome", "input", "ime"], js_name = "hideInputView")]
    pub fn hide_input_view();
    ///Sets the properties of the candidate window. This fails if the extension doesn't own the active IME
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime"],
        js_name = "setCandidateWindowProperties"
    )]
    pub fn set_candidate_window_properties(parameters: Object) -> Promise;
    ///Sets the current candidate list. This fails if this extension doesn't own the active IME
    #[wasm_bindgen(js_namespace = ["chrome", "input", "ime"], js_name = "setCandidates")]
    pub fn set_candidates(parameters: Object) -> Promise;
    ///Set the position of the cursor in the candidate window. This is a no-op if this extension does not own the active IME.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime"],
        js_name = "setCursorPosition"
    )]
    pub fn set_cursor_position(parameters: Object) -> Promise;
    ///Shows/Hides an assistive window with the given properties.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime"],
        js_name = "setAssistiveWindowProperties"
    )]
    pub fn set_assistive_window_properties(parameters: Object) -> Promise;
    ///Highlights/Unhighlights a button in an assistive window.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime"],
        js_name = "setAssistiveWindowButtonHighlighted"
    )]
    pub fn set_assistive_window_button_highlighted(parameters: Object) -> Promise;
    ///Adds the provided menu items to the language menu when this IME is active.
    #[wasm_bindgen(js_namespace = ["chrome", "input", "ime"], js_name = "setMenuItems")]
    pub fn set_menu_items(parameters: MenuParameters) -> Promise;
    ///Updates the state of the MenuItems specified
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime"],
        js_name = "updateMenuItems"
    )]
    pub fn update_menu_items(parameters: MenuParameters) -> Promise;
    ///Deletes the text around the caret.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime"],
        js_name = "deleteSurroundingText"
    )]
    pub fn delete_surrounding_text(parameters: Object) -> Promise;
    ///Indicates that the key event received by onKeyEvent is handled. This should only be called if the onKeyEvent listener is asynchronous.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime"],
        js_name = "keyEventHandled"
    )]
    pub fn key_event_handled(request_id: String, response: bool);
    ///This event is sent when an IME is activated. It signals that the IME will be receiving onKeyPress events.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime",
        "onActivate"],
        js_name = "addListener"
    )]
    pub fn on_activate_add_listener(callback: &Function);
    ///This event is sent when an IME is deactivated. It signals that the IME will no longer be receiving onKeyPress events.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime",
        "onDeactivated"],
        js_name = "addListener"
    )]
    pub fn on_deactivated_add_listener(callback: &Function);
    ///This event is sent when focus enters a text box. It is sent to all extensions that are listening to this event, and enabled by the user.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime",
        "onFocus"],
        js_name = "addListener"
    )]
    pub fn on_focus_add_listener(callback: &Function);
    ///This event is sent when focus leaves a text box. It is sent to all extensions that are listening to this event, and enabled by the user.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime",
        "onBlur"],
        js_name = "addListener"
    )]
    pub fn on_blur_add_listener(callback: &Function);
    ///This event is sent when the properties of the current InputContext change, such as the the type. It is sent to all extensions that are listening to this event, and enabled by the user.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime",
        "onInputContextUpdate"],
        js_name = "addListener"
    )]
    pub fn on_input_context_update_add_listener(callback: &Function);
    ///Fired when a key event is sent from the operating system. The event will be sent to the extension if this extension owns the active IME. The listener function should return true if the event was handled false if it was not. If the event will be evaluated asynchronously, this function must return undefined and the IME must later call keyEventHandled() with the result.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime",
        "onKeyEvent"],
        js_name = "addListener"
    )]
    pub fn on_key_event_add_listener(callback: &Function);
    ///This event is sent if this extension owns the active IME.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime",
        "onCandidateClicked"],
        js_name = "addListener"
    )]
    pub fn on_candidate_clicked_add_listener(callback: &Function);
    ///Called when the user selects a menu item
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime",
        "onMenuItemActivated"],
        js_name = "addListener"
    )]
    pub fn on_menu_item_activated_add_listener(callback: &Function);
    ///Called when the editable string around caret is changed or when the caret position is moved. The text length is limited to 100 characters for each back and forth direction.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime",
        "onSurroundingTextChanged"],
        js_name = "addListener"
    )]
    pub fn on_surrounding_text_changed_add_listener(callback: &Function);
    ///This event is sent when chrome terminates ongoing text input session.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime",
        "onReset"],
        js_name = "addListener"
    )]
    pub fn on_reset_add_listener(callback: &Function);
    ///This event is sent when a button in an assistive window is clicked.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "input",
        "ime",
        "onAssistiveWindowButtonClicked"],
        js_name = "addListener"
    )]
    pub fn on_assistive_window_button_clicked_add_listener(callback: &Function);
}
