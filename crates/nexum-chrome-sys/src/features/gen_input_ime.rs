#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    ///Get the `altKey` field of this object.
    #[wasm_bindgen(method, getter = "altKey")]
    pub fn get_alt_key(this: &KeyboardEvent) -> Option<bool>;
    ///Change the `altKey` field of this object.
    #[wasm_bindgen(method, setter = "altKey")]
    pub fn set_alt_key(this: &KeyboardEvent, val: bool);
    ///Get the `altgrKey` field of this object.
    #[wasm_bindgen(method, getter = "altgrKey")]
    pub fn get_altgr_key(this: &KeyboardEvent) -> Option<bool>;
    ///Change the `altgrKey` field of this object.
    #[wasm_bindgen(method, setter = "altgrKey")]
    pub fn set_altgr_key(this: &KeyboardEvent, val: bool);
    ///Get the `capsLock` field of this object.
    #[wasm_bindgen(method, getter = "capsLock")]
    pub fn get_caps_lock(this: &KeyboardEvent) -> Option<bool>;
    ///Change the `capsLock` field of this object.
    #[wasm_bindgen(method, setter = "capsLock")]
    pub fn set_caps_lock(this: &KeyboardEvent, val: bool);
    ///Get the `code` field of this object.
    #[wasm_bindgen(method, getter = "code")]
    pub fn get_code(this: &KeyboardEvent) -> String;
    ///Change the `code` field of this object.
    #[wasm_bindgen(method, setter = "code")]
    pub fn set_code(this: &KeyboardEvent, val: String);
    ///Get the `ctrlKey` field of this object.
    #[wasm_bindgen(method, getter = "ctrlKey")]
    pub fn get_ctrl_key(this: &KeyboardEvent) -> Option<bool>;
    ///Change the `ctrlKey` field of this object.
    #[wasm_bindgen(method, setter = "ctrlKey")]
    pub fn set_ctrl_key(this: &KeyboardEvent, val: bool);
    ///Get the `extensionId` field of this object.
    #[wasm_bindgen(method, getter = "extensionId")]
    pub fn get_extension_id(this: &KeyboardEvent) -> Option<String>;
    ///Change the `extensionId` field of this object.
    #[wasm_bindgen(method, setter = "extensionId")]
    pub fn set_extension_id(this: &KeyboardEvent, val: String);
    ///Get the `key` field of this object.
    #[wasm_bindgen(method, getter = "key")]
    pub fn get_key(this: &KeyboardEvent) -> String;
    ///Change the `key` field of this object.
    #[wasm_bindgen(method, setter = "key")]
    pub fn set_key(this: &KeyboardEvent, val: String);
    ///Get the `keyCode` field of this object.
    #[wasm_bindgen(method, getter = "keyCode")]
    pub fn get_key_code(this: &KeyboardEvent) -> Option<i32>;
    ///Change the `keyCode` field of this object.
    #[wasm_bindgen(method, setter = "keyCode")]
    pub fn set_key_code(this: &KeyboardEvent, val: i32);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &KeyboardEvent) -> Option<String>;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &KeyboardEvent, val: String);
    ///Get the `shiftKey` field of this object.
    #[wasm_bindgen(method, getter = "shiftKey")]
    pub fn get_shift_key(this: &KeyboardEvent) -> Option<bool>;
    ///Change the `shiftKey` field of this object.
    #[wasm_bindgen(method, setter = "shiftKey")]
    pub fn set_shift_key(this: &KeyboardEvent, val: bool);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &KeyboardEvent) -> KeyboardEventType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &KeyboardEvent, val: KeyboardEventType);
}
impl KeyboardEvent {
    ///Construct a new `KeyboardEvent`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_alt_key()` instead."]
    pub fn alt_key(&mut self, val: bool) -> &mut Self {
        self.set_alt_key(val);
        self
    }
    #[deprecated = "Use `set_altgr_key()` instead."]
    pub fn altgr_key(&mut self, val: bool) -> &mut Self {
        self.set_altgr_key(val);
        self
    }
    #[deprecated = "Use `set_caps_lock()` instead."]
    pub fn caps_lock(&mut self, val: bool) -> &mut Self {
        self.set_caps_lock(val);
        self
    }
    #[deprecated = "Use `set_code()` instead."]
    pub fn code(&mut self, val: String) -> &mut Self {
        self.set_code(val);
        self
    }
    #[deprecated = "Use `set_ctrl_key()` instead."]
    pub fn ctrl_key(&mut self, val: bool) -> &mut Self {
        self.set_ctrl_key(val);
        self
    }
    #[deprecated = "Use `set_extension_id()` instead."]
    pub fn extension_id(&mut self, val: String) -> &mut Self {
        self.set_extension_id(val);
        self
    }
    #[deprecated = "Use `set_key()` instead."]
    pub fn key(&mut self, val: String) -> &mut Self {
        self.set_key(val);
        self
    }
    #[deprecated = "Use `set_key_code()` instead."]
    pub fn key_code(&mut self, val: i32) -> &mut Self {
        self.set_key_code(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: String) -> &mut Self {
        self.set_request_id(val);
        self
    }
    #[deprecated = "Use `set_shift_key()` instead."]
    pub fn shift_key(&mut self, val: bool) -> &mut Self {
        self.set_shift_key(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: KeyboardEventType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for KeyboardEvent {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `KeyboardEvent`. See http://www.w3.org/TR/DOM-Level-3-Events/#events-KeyboardEvent
pub struct KeyboardEventData {
    ///Whether or not the ALT key is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_key: Option<bool>,
    ///Whether or not the ALTGR key is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altgr_key: Option<bool>,
    ///Whether or not the CAPS_LOCK is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caps_lock: Option<bool>,
    ///Value of the physical key being pressed. The value is not affected by current keyboard layout or modifier state.
    pub code: String,
    ///Whether or not the CTRL key is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctrl_key: Option<bool>,
    ///The extension ID of the sender of this keyevent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_id: Option<String>,
    ///Value of the key being pressed
    pub key: String,
    ///The deprecated HTML keyCode, which is system- and implementation-dependent numerical code signifying the unmodified identifier associated with the key pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_code: Option<i32>,
    ///(Deprecated) The ID of the request. Use the requestId param from the onKeyEvent event instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    ///Whether or not the SHIFT key is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_key: Option<bool>,
    ///One of keyup or keydown.
    pub r#type: KeyboardEventType,
}
#[cfg(feature = "serde")]
impl From<&KeyboardEvent> for KeyboardEventData {
    fn from(val: &KeyboardEvent) -> Self {
        Self {
            alt_key: val.get_alt_key(),
            altgr_key: val.get_altgr_key(),
            caps_lock: val.get_caps_lock(),
            code: val.get_code(),
            ctrl_key: val.get_ctrl_key(),
            extension_id: val.get_extension_id(),
            key: val.get_key(),
            key_code: val.get_key_code(),
            request_id: val.get_request_id(),
            shift_key: val.get_shift_key(),
            r#type: val.get_type(),
        }
    }
}
#[wasm_bindgen]
///Type of value this text field edits, (Text, Number, URL, etc)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &InputContext) -> InputContextType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &InputContext, val: InputContextType);
}
impl InputContext {
    ///Construct a new `InputContext`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
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
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: InputContextType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for InputContext {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `InputContext`. Describes an input Context
pub struct InputContextData {
    ///The auto-capitalize type of the text field.
    pub auto_capitalize: AutoCapitalizeType,
    ///Whether the text field wants auto-complete.
    pub auto_complete: bool,
    ///Whether the text field wants auto-correct.
    pub auto_correct: bool,
    ///This is used to specify targets of text field operations. This ID becomes invalid as soon as onBlur is called.
    pub context_id: i32,
    ///Whether text entered into the text field should be used to improve typing suggestions for the user.
    pub should_do_learning: bool,
    ///Whether the text field wants spell-check.
    pub spell_check: bool,
    ///Type of value this text field edits, (Text, Number, URL, etc)
    pub r#type: InputContextType,
}
#[cfg(feature = "serde")]
impl From<&InputContext> for InputContextData {
    fn from(val: &InputContext) -> Self {
        Self {
            auto_capitalize: val.get_auto_capitalize(),
            auto_complete: val.get_auto_complete(),
            auto_correct: val.get_auto_correct(),
            context_id: val.get_context_id(),
            should_do_learning: val.get_should_do_learning(),
            spell_check: val.get_spell_check(),
            r#type: val.get_type(),
        }
    }
}
#[wasm_bindgen]
///The type of menu item. Radio buttons between separators are considered grouped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &MenuItem) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &MenuItem, val: String);
    ///Get the `label` field of this object.
    #[wasm_bindgen(method, getter = "label")]
    pub fn get_label(this: &MenuItem) -> Option<String>;
    ///Change the `label` field of this object.
    #[wasm_bindgen(method, setter = "label")]
    pub fn set_label(this: &MenuItem, val: String);
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
}
impl MenuItem {
    ///Construct a new `MenuItem`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
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
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_label()` instead."]
    pub fn label(&mut self, val: String) -> &mut Self {
        self.set_label(val);
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
}
impl Default for MenuItem {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `MenuItem`. A menu item used by an input method to interact with the user from the language menu.
pub struct MenuItemData {
    ///Indicates this item should be drawn with a check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
    ///Indicates this item is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    ///String that will be passed to callbacks referencing this MenuItem.
    pub id: String,
    ///Text displayed in the menu for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    ///The type of menu item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<MenuItemStyle>,
    ///Indicates this item is visible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&MenuItem> for MenuItemData {
    fn from(val: &MenuItem) -> Self {
        Self {
            checked: val.get_checked(),
            enabled: val.get_enabled(),
            id: val.get_id(),
            label: val.get_label(),
            style: val.get_style(),
            visible: val.get_visible(),
        }
    }
}
#[wasm_bindgen]
///The type of the underline to modify this segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnderlineStyle {
    Underline = "underline",
    DoubleUnderline = "doubleUnderline",
    NoUnderline = "noUnderline",
}
#[wasm_bindgen]
///Where to display the candidate window. If set to 'cursor', the window follows the cursor. If set to 'composition', the window is locked to the beginning of the composition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WindowPosition {
    Cursor = "cursor",
    Composition = "composition",
}
#[wasm_bindgen]
///The screen type under which the IME is activated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScreenType {
    Normal = "normal",
    Login = "login",
    Lock = "lock",
    SecondaryLogin = "secondary-login",
}
#[wasm_bindgen]
///Which mouse buttons was clicked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MouseButton {
    Left = "left",
    Middle = "middle",
    Right = "right",
}
#[wasm_bindgen]
///Type of assistive window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AssistiveWindowType {
    Undo = "undo",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AssistiveWindowProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Properties of the assistive window.
    pub type AssistiveWindowProperties;
    ///Get the `announceString` field of this object.
    #[wasm_bindgen(method, getter = "announceString")]
    pub fn get_announce_string(this: &AssistiveWindowProperties) -> Option<String>;
    ///Change the `announceString` field of this object.
    #[wasm_bindgen(method, setter = "announceString")]
    pub fn set_announce_string(this: &AssistiveWindowProperties, val: String);
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
}
impl AssistiveWindowProperties {
    ///Construct a new `AssistiveWindowProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_announce_string()` instead."]
    pub fn announce_string(&mut self, val: String) -> &mut Self {
        self.set_announce_string(val);
        self
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
}
impl Default for AssistiveWindowProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `AssistiveWindowProperties`. Properties of the assistive window.
pub struct AssistiveWindowPropertiesData {
    ///Strings for ChromeVox to announce.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announce_string: Option<String>,
    ///
    pub r#type: AssistiveWindowType,
    ///Sets true to show AssistiveWindow, sets false to hide.
    pub visible: bool,
}
#[cfg(feature = "serde")]
impl From<&AssistiveWindowProperties> for AssistiveWindowPropertiesData {
    fn from(val: &AssistiveWindowProperties) -> Self {
        Self {
            announce_string: val.get_announce_string(),
            r#type: val.get_type(),
            visible: val.get_visible(),
        }
    }
}
#[wasm_bindgen]
///ID of buttons in assistive window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `MenuParameters`.
pub struct MenuParametersData {
    ///ID of the engine to use.
    pub engine_id: String,
    ///MenuItems to add or update. They will be added in the order they exist in the array.
    pub items: Vec<MenuItemData>,
}
#[cfg(feature = "serde")]
impl From<&MenuParameters> for MenuParametersData {
    fn from(val: &MenuParameters) -> Self {
        Self {
            engine_id: val.get_engine_id(),
            items: serde_wasm_bindgen::from_value(val.get_items().into()).unwrap_or_default(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "OnSurroundingTextChangedSurroundingInfo"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The surrounding information.
    pub type OnSurroundingTextChangedSurroundingInfo;
    ///Get the `anchor` field of this object.
    #[wasm_bindgen(method, getter = "anchor")]
    pub fn get_anchor(this: &OnSurroundingTextChangedSurroundingInfo) -> i32;
    ///Change the `anchor` field of this object.
    #[wasm_bindgen(method, setter = "anchor")]
    pub fn set_anchor(this: &OnSurroundingTextChangedSurroundingInfo, val: i32);
    ///Get the `focus` field of this object.
    #[wasm_bindgen(method, getter = "focus")]
    pub fn get_focus(this: &OnSurroundingTextChangedSurroundingInfo) -> i32;
    ///Change the `focus` field of this object.
    #[wasm_bindgen(method, setter = "focus")]
    pub fn set_focus(this: &OnSurroundingTextChangedSurroundingInfo, val: i32);
    ///Get the `offset` field of this object.
    #[wasm_bindgen(method, getter = "offset")]
    pub fn get_offset(this: &OnSurroundingTextChangedSurroundingInfo) -> i32;
    ///Change the `offset` field of this object.
    #[wasm_bindgen(method, setter = "offset")]
    pub fn set_offset(this: &OnSurroundingTextChangedSurroundingInfo, val: i32);
    ///Get the `text` field of this object.
    #[wasm_bindgen(method, getter = "text")]
    pub fn get_text(this: &OnSurroundingTextChangedSurroundingInfo) -> String;
    ///Change the `text` field of this object.
    #[wasm_bindgen(method, setter = "text")]
    pub fn set_text(this: &OnSurroundingTextChangedSurroundingInfo, val: String);
}
impl OnSurroundingTextChangedSurroundingInfo {
    ///Construct a new `OnSurroundingTextChangedSurroundingInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_anchor()` instead."]
    pub fn anchor(&mut self, val: i32) -> &mut Self {
        self.set_anchor(val);
        self
    }
    #[deprecated = "Use `set_focus()` instead."]
    pub fn focus(&mut self, val: i32) -> &mut Self {
        self.set_focus(val);
        self
    }
    #[deprecated = "Use `set_offset()` instead."]
    pub fn offset(&mut self, val: i32) -> &mut Self {
        self.set_offset(val);
        self
    }
    #[deprecated = "Use `set_text()` instead."]
    pub fn text(&mut self, val: String) -> &mut Self {
        self.set_text(val);
        self
    }
}
impl Default for OnSurroundingTextChangedSurroundingInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "OnAssistiveWindowButtonClickedDetails"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnAssistiveWindowButtonClickedDetails;
    ///Get the `buttonID` field of this object.
    #[wasm_bindgen(method, getter = "buttonID")]
    pub fn get_button_id(this: &OnAssistiveWindowButtonClickedDetails) -> AssistiveWindowButton;
    ///Change the `buttonID` field of this object.
    #[wasm_bindgen(method, setter = "buttonID")]
    pub fn set_button_id(this: &OnAssistiveWindowButtonClickedDetails, val: AssistiveWindowButton);
    ///Get the `windowType` field of this object.
    #[wasm_bindgen(method, getter = "windowType")]
    pub fn get_window_type(this: &OnAssistiveWindowButtonClickedDetails) -> AssistiveWindowType;
    ///Change the `windowType` field of this object.
    #[wasm_bindgen(method, setter = "windowType")]
    pub fn set_window_type(this: &OnAssistiveWindowButtonClickedDetails, val: AssistiveWindowType);
}
impl OnAssistiveWindowButtonClickedDetails {
    ///Construct a new `OnAssistiveWindowButtonClickedDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_button_id()` instead."]
    pub fn button_id(&mut self, val: AssistiveWindowButton) -> &mut Self {
        self.set_button_id(val);
        self
    }
    #[deprecated = "Use `set_window_type()` instead."]
    pub fn window_type(&mut self, val: AssistiveWindowType) -> &mut Self {
        self.set_window_type(val);
        self
    }
}
impl Default for OnAssistiveWindowButtonClickedDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SetCompositionParameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetCompositionParameters;
    ///Get the `contextID` field of this object.
    #[wasm_bindgen(method, getter = "contextID")]
    pub fn get_context_id(this: &SetCompositionParameters) -> i32;
    ///Change the `contextID` field of this object.
    #[wasm_bindgen(method, setter = "contextID")]
    pub fn set_context_id(this: &SetCompositionParameters, val: i32);
    ///Get the `cursor` field of this object.
    #[wasm_bindgen(method, getter = "cursor")]
    pub fn get_cursor(this: &SetCompositionParameters) -> i32;
    ///Change the `cursor` field of this object.
    #[wasm_bindgen(method, setter = "cursor")]
    pub fn set_cursor(this: &SetCompositionParameters, val: i32);
    ///Get the `segments` field of this object.
    #[wasm_bindgen(method, getter = "segments")]
    pub fn get_segments(this: &SetCompositionParameters) -> Option<Array>;
    ///Change the `segments` field of this object.
    #[wasm_bindgen(method, setter = "segments")]
    pub fn set_segments(this: &SetCompositionParameters, val: &Array);
    ///Get the `selectionEnd` field of this object.
    #[wasm_bindgen(method, getter = "selectionEnd")]
    pub fn get_selection_end(this: &SetCompositionParameters) -> Option<i32>;
    ///Change the `selectionEnd` field of this object.
    #[wasm_bindgen(method, setter = "selectionEnd")]
    pub fn set_selection_end(this: &SetCompositionParameters, val: i32);
    ///Get the `selectionStart` field of this object.
    #[wasm_bindgen(method, getter = "selectionStart")]
    pub fn get_selection_start(this: &SetCompositionParameters) -> Option<i32>;
    ///Change the `selectionStart` field of this object.
    #[wasm_bindgen(method, setter = "selectionStart")]
    pub fn set_selection_start(this: &SetCompositionParameters, val: i32);
    ///Get the `text` field of this object.
    #[wasm_bindgen(method, getter = "text")]
    pub fn get_text(this: &SetCompositionParameters) -> String;
    ///Change the `text` field of this object.
    #[wasm_bindgen(method, setter = "text")]
    pub fn set_text(this: &SetCompositionParameters, val: String);
}
impl SetCompositionParameters {
    ///Construct a new `SetCompositionParameters`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_context_id()` instead."]
    pub fn context_id(&mut self, val: i32) -> &mut Self {
        self.set_context_id(val);
        self
    }
    #[deprecated = "Use `set_cursor()` instead."]
    pub fn cursor(&mut self, val: i32) -> &mut Self {
        self.set_cursor(val);
        self
    }
    #[deprecated = "Use `set_segments()` instead."]
    pub fn segments(&mut self, val: &Array) -> &mut Self {
        self.set_segments(val);
        self
    }
    #[deprecated = "Use `set_selection_end()` instead."]
    pub fn selection_end(&mut self, val: i32) -> &mut Self {
        self.set_selection_end(val);
        self
    }
    #[deprecated = "Use `set_selection_start()` instead."]
    pub fn selection_start(&mut self, val: i32) -> &mut Self {
        self.set_selection_start(val);
        self
    }
    #[deprecated = "Use `set_text()` instead."]
    pub fn text(&mut self, val: String) -> &mut Self {
        self.set_text(val);
        self
    }
}
impl Default for SetCompositionParameters {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ClearCompositionParameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ClearCompositionParameters;
    ///Get the `contextID` field of this object.
    #[wasm_bindgen(method, getter = "contextID")]
    pub fn get_context_id(this: &ClearCompositionParameters) -> i32;
    ///Change the `contextID` field of this object.
    #[wasm_bindgen(method, setter = "contextID")]
    pub fn set_context_id(this: &ClearCompositionParameters, val: i32);
}
impl ClearCompositionParameters {
    ///Construct a new `ClearCompositionParameters`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_context_id()` instead."]
    pub fn context_id(&mut self, val: i32) -> &mut Self {
        self.set_context_id(val);
        self
    }
}
impl Default for ClearCompositionParameters {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CommitTextParameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CommitTextParameters;
    ///Get the `contextID` field of this object.
    #[wasm_bindgen(method, getter = "contextID")]
    pub fn get_context_id(this: &CommitTextParameters) -> i32;
    ///Change the `contextID` field of this object.
    #[wasm_bindgen(method, setter = "contextID")]
    pub fn set_context_id(this: &CommitTextParameters, val: i32);
    ///Get the `text` field of this object.
    #[wasm_bindgen(method, getter = "text")]
    pub fn get_text(this: &CommitTextParameters) -> String;
    ///Change the `text` field of this object.
    #[wasm_bindgen(method, setter = "text")]
    pub fn set_text(this: &CommitTextParameters, val: String);
}
impl CommitTextParameters {
    ///Construct a new `CommitTextParameters`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_context_id()` instead."]
    pub fn context_id(&mut self, val: i32) -> &mut Self {
        self.set_context_id(val);
        self
    }
    #[deprecated = "Use `set_text()` instead."]
    pub fn text(&mut self, val: String) -> &mut Self {
        self.set_text(val);
        self
    }
}
impl Default for CommitTextParameters {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SendKeyEventsParameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SendKeyEventsParameters;
    ///Get the `contextID` field of this object.
    #[wasm_bindgen(method, getter = "contextID")]
    pub fn get_context_id(this: &SendKeyEventsParameters) -> i32;
    ///Change the `contextID` field of this object.
    #[wasm_bindgen(method, setter = "contextID")]
    pub fn set_context_id(this: &SendKeyEventsParameters, val: i32);
    ///Get the `keyData` field of this object.
    #[wasm_bindgen(method, getter = "keyData")]
    pub fn get_key_data(this: &SendKeyEventsParameters) -> Array;
    ///Change the `keyData` field of this object.
    #[wasm_bindgen(method, setter = "keyData")]
    pub fn set_key_data(this: &SendKeyEventsParameters, val: &Array);
}
impl SendKeyEventsParameters {
    ///Construct a new `SendKeyEventsParameters`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_context_id()` instead."]
    pub fn context_id(&mut self, val: i32) -> &mut Self {
        self.set_context_id(val);
        self
    }
    #[deprecated = "Use `set_key_data()` instead."]
    pub fn key_data(&mut self, val: &Array) -> &mut Self {
        self.set_key_data(val);
        self
    }
}
impl Default for SendKeyEventsParameters {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "SetCandidateWindowPropertiesParameters"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetCandidateWindowPropertiesParameters;
    ///Get the `engineID` field of this object.
    #[wasm_bindgen(method, getter = "engineID")]
    pub fn get_engine_id(this: &SetCandidateWindowPropertiesParameters) -> String;
    ///Change the `engineID` field of this object.
    #[wasm_bindgen(method, setter = "engineID")]
    pub fn set_engine_id(this: &SetCandidateWindowPropertiesParameters, val: String);
    ///Get the `properties` field of this object.
    #[wasm_bindgen(method, getter = "properties")]
    pub fn get_properties(this: &SetCandidateWindowPropertiesParameters) -> Object;
    ///Change the `properties` field of this object.
    #[wasm_bindgen(method, setter = "properties")]
    pub fn set_properties(this: &SetCandidateWindowPropertiesParameters, val: &Object);
}
impl SetCandidateWindowPropertiesParameters {
    ///Construct a new `SetCandidateWindowPropertiesParameters`.
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
    #[deprecated = "Use `set_properties()` instead."]
    pub fn properties(&mut self, val: &Object) -> &mut Self {
        self.set_properties(val);
        self
    }
}
impl Default for SetCandidateWindowPropertiesParameters {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SetCandidatesParameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetCandidatesParameters;
    ///Get the `candidates` field of this object.
    #[wasm_bindgen(method, getter = "candidates")]
    pub fn get_candidates(this: &SetCandidatesParameters) -> Array;
    ///Change the `candidates` field of this object.
    #[wasm_bindgen(method, setter = "candidates")]
    pub fn set_candidates(this: &SetCandidatesParameters, val: &Array);
    ///Get the `contextID` field of this object.
    #[wasm_bindgen(method, getter = "contextID")]
    pub fn get_context_id(this: &SetCandidatesParameters) -> i32;
    ///Change the `contextID` field of this object.
    #[wasm_bindgen(method, setter = "contextID")]
    pub fn set_context_id(this: &SetCandidatesParameters, val: i32);
}
impl SetCandidatesParameters {
    ///Construct a new `SetCandidatesParameters`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_candidates()` instead."]
    pub fn candidates(&mut self, val: &Array) -> &mut Self {
        self.set_candidates(val);
        self
    }
    #[deprecated = "Use `set_context_id()` instead."]
    pub fn context_id(&mut self, val: i32) -> &mut Self {
        self.set_context_id(val);
        self
    }
}
impl Default for SetCandidatesParameters {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SetCursorPositionParameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetCursorPositionParameters;
    ///Get the `candidateID` field of this object.
    #[wasm_bindgen(method, getter = "candidateID")]
    pub fn get_candidate_id(this: &SetCursorPositionParameters) -> i32;
    ///Change the `candidateID` field of this object.
    #[wasm_bindgen(method, setter = "candidateID")]
    pub fn set_candidate_id(this: &SetCursorPositionParameters, val: i32);
    ///Get the `contextID` field of this object.
    #[wasm_bindgen(method, getter = "contextID")]
    pub fn get_context_id(this: &SetCursorPositionParameters) -> i32;
    ///Change the `contextID` field of this object.
    #[wasm_bindgen(method, setter = "contextID")]
    pub fn set_context_id(this: &SetCursorPositionParameters, val: i32);
}
impl SetCursorPositionParameters {
    ///Construct a new `SetCursorPositionParameters`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_candidate_id()` instead."]
    pub fn candidate_id(&mut self, val: i32) -> &mut Self {
        self.set_candidate_id(val);
        self
    }
    #[deprecated = "Use `set_context_id()` instead."]
    pub fn context_id(&mut self, val: i32) -> &mut Self {
        self.set_context_id(val);
        self
    }
}
impl Default for SetCursorPositionParameters {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "SetAssistiveWindowPropertiesParameters"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetAssistiveWindowPropertiesParameters;
    ///Get the `contextID` field of this object.
    #[wasm_bindgen(method, getter = "contextID")]
    pub fn get_context_id(this: &SetAssistiveWindowPropertiesParameters) -> i32;
    ///Change the `contextID` field of this object.
    #[wasm_bindgen(method, setter = "contextID")]
    pub fn set_context_id(this: &SetAssistiveWindowPropertiesParameters, val: i32);
    ///Get the `properties` field of this object.
    #[wasm_bindgen(method, getter = "properties")]
    pub fn get_properties(
        this: &SetAssistiveWindowPropertiesParameters,
    ) -> AssistiveWindowProperties;
    ///Change the `properties` field of this object.
    #[wasm_bindgen(method, setter = "properties")]
    pub fn set_properties(
        this: &SetAssistiveWindowPropertiesParameters,
        val: &AssistiveWindowProperties,
    );
}
impl SetAssistiveWindowPropertiesParameters {
    ///Construct a new `SetAssistiveWindowPropertiesParameters`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_context_id()` instead."]
    pub fn context_id(&mut self, val: i32) -> &mut Self {
        self.set_context_id(val);
        self
    }
    #[deprecated = "Use `set_properties()` instead."]
    pub fn properties(&mut self, val: &AssistiveWindowProperties) -> &mut Self {
        self.set_properties(val);
        self
    }
}
impl Default for SetAssistiveWindowPropertiesParameters {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "SetAssistiveWindowButtonHighlightedParameters"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetAssistiveWindowButtonHighlightedParameters;
    ///Get the `announceString` field of this object.
    #[wasm_bindgen(method, getter = "announceString")]
    pub fn get_announce_string(
        this: &SetAssistiveWindowButtonHighlightedParameters,
    ) -> Option<String>;
    ///Change the `announceString` field of this object.
    #[wasm_bindgen(method, setter = "announceString")]
    pub fn set_announce_string(this: &SetAssistiveWindowButtonHighlightedParameters, val: String);
    ///Get the `buttonID` field of this object.
    #[wasm_bindgen(method, getter = "buttonID")]
    pub fn get_button_id(
        this: &SetAssistiveWindowButtonHighlightedParameters,
    ) -> AssistiveWindowButton;
    ///Change the `buttonID` field of this object.
    #[wasm_bindgen(method, setter = "buttonID")]
    pub fn set_button_id(
        this: &SetAssistiveWindowButtonHighlightedParameters,
        val: AssistiveWindowButton,
    );
    ///Get the `contextID` field of this object.
    #[wasm_bindgen(method, getter = "contextID")]
    pub fn get_context_id(this: &SetAssistiveWindowButtonHighlightedParameters) -> i32;
    ///Change the `contextID` field of this object.
    #[wasm_bindgen(method, setter = "contextID")]
    pub fn set_context_id(this: &SetAssistiveWindowButtonHighlightedParameters, val: i32);
    ///Get the `highlighted` field of this object.
    #[wasm_bindgen(method, getter = "highlighted")]
    pub fn get_highlighted(this: &SetAssistiveWindowButtonHighlightedParameters) -> bool;
    ///Change the `highlighted` field of this object.
    #[wasm_bindgen(method, setter = "highlighted")]
    pub fn set_highlighted(this: &SetAssistiveWindowButtonHighlightedParameters, val: bool);
    ///Get the `windowType` field of this object.
    #[wasm_bindgen(method, getter = "windowType")]
    pub fn get_window_type(
        this: &SetAssistiveWindowButtonHighlightedParameters,
    ) -> AssistiveWindowType;
    ///Change the `windowType` field of this object.
    #[wasm_bindgen(method, setter = "windowType")]
    pub fn set_window_type(
        this: &SetAssistiveWindowButtonHighlightedParameters,
        val: AssistiveWindowType,
    );
}
impl SetAssistiveWindowButtonHighlightedParameters {
    ///Construct a new `SetAssistiveWindowButtonHighlightedParameters`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_announce_string()` instead."]
    pub fn announce_string(&mut self, val: String) -> &mut Self {
        self.set_announce_string(val);
        self
    }
    #[deprecated = "Use `set_button_id()` instead."]
    pub fn button_id(&mut self, val: AssistiveWindowButton) -> &mut Self {
        self.set_button_id(val);
        self
    }
    #[deprecated = "Use `set_context_id()` instead."]
    pub fn context_id(&mut self, val: i32) -> &mut Self {
        self.set_context_id(val);
        self
    }
    #[deprecated = "Use `set_highlighted()` instead."]
    pub fn highlighted(&mut self, val: bool) -> &mut Self {
        self.set_highlighted(val);
        self
    }
    #[deprecated = "Use `set_window_type()` instead."]
    pub fn window_type(&mut self, val: AssistiveWindowType) -> &mut Self {
        self.set_window_type(val);
        self
    }
}
impl Default for SetAssistiveWindowButtonHighlightedParameters {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "DeleteSurroundingTextParameters"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DeleteSurroundingTextParameters;
    ///Get the `contextID` field of this object.
    #[wasm_bindgen(method, getter = "contextID")]
    pub fn get_context_id(this: &DeleteSurroundingTextParameters) -> i32;
    ///Change the `contextID` field of this object.
    #[wasm_bindgen(method, setter = "contextID")]
    pub fn set_context_id(this: &DeleteSurroundingTextParameters, val: i32);
    ///Get the `engineID` field of this object.
    #[wasm_bindgen(method, getter = "engineID")]
    pub fn get_engine_id(this: &DeleteSurroundingTextParameters) -> String;
    ///Change the `engineID` field of this object.
    #[wasm_bindgen(method, setter = "engineID")]
    pub fn set_engine_id(this: &DeleteSurroundingTextParameters, val: String);
    ///Get the `length` field of this object.
    #[wasm_bindgen(method, getter = "length")]
    pub fn get_length(this: &DeleteSurroundingTextParameters) -> i32;
    ///Change the `length` field of this object.
    #[wasm_bindgen(method, setter = "length")]
    pub fn set_length(this: &DeleteSurroundingTextParameters, val: i32);
    ///Get the `offset` field of this object.
    #[wasm_bindgen(method, getter = "offset")]
    pub fn get_offset(this: &DeleteSurroundingTextParameters) -> i32;
    ///Change the `offset` field of this object.
    #[wasm_bindgen(method, setter = "offset")]
    pub fn set_offset(this: &DeleteSurroundingTextParameters, val: i32);
}
impl DeleteSurroundingTextParameters {
    ///Construct a new `DeleteSurroundingTextParameters`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_context_id()` instead."]
    pub fn context_id(&mut self, val: i32) -> &mut Self {
        self.set_context_id(val);
        self
    }
    #[deprecated = "Use `set_engine_id()` instead."]
    pub fn engine_id(&mut self, val: String) -> &mut Self {
        self.set_engine_id(val);
        self
    }
    #[deprecated = "Use `set_length()` instead."]
    pub fn length(&mut self, val: i32) -> &mut Self {
        self.set_length(val);
        self
    }
    #[deprecated = "Use `set_offset()` instead."]
    pub fn offset(&mut self, val: i32) -> &mut Self {
        self.set_offset(val);
        self
    }
}
impl Default for DeleteSurroundingTextParameters {
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
