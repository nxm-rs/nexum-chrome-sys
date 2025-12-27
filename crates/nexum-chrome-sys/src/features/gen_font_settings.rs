#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "FontName")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Represents a font name.
    pub type FontName;
    ///Get the `displayName` field of this object.
    #[wasm_bindgen(method, getter = "displayName")]
    pub fn get_display_name(this: &FontName) -> String;
    ///Change the `displayName` field of this object.
    #[wasm_bindgen(method, setter = "displayName")]
    pub fn set_display_name(this: &FontName, val: String);
    ///Get the `fontId` field of this object.
    #[wasm_bindgen(method, getter = "fontId")]
    pub fn get_font_id(this: &FontName) -> String;
    ///Change the `fontId` field of this object.
    #[wasm_bindgen(method, setter = "fontId")]
    pub fn set_font_id(this: &FontName, val: String);
}
impl FontName {
    ///Construct a new `FontName`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_display_name()` instead."]
    pub fn display_name(&mut self, val: String) -> &mut Self {
        self.set_display_name(val);
        self
    }
    #[deprecated = "Use `set_font_id()` instead."]
    pub fn font_id(&mut self, val: String) -> &mut Self {
        self.set_font_id(val);
        self
    }
}
impl Default for FontName {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `FontName`. Represents a font name.
pub struct FontNameData {
    ///The display name of the font.
    pub display_name: String,
    ///The font ID.
    pub font_id: String,
}
#[cfg(feature = "serde")]
impl From<&FontName> for FontNameData {
    fn from(val: &FontName) -> Self {
        Self {
            display_name: val.get_display_name(),
            font_id: val.get_font_id(),
        }
    }
}
#[wasm_bindgen]
///An ISO 15924 script code. The default, or global, script is represented by script code "Zyyy".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScriptCode {
    Afak = "Afak",
    Arab = "Arab",
    Armi = "Armi",
    Armn = "Armn",
    Avst = "Avst",
    Bali = "Bali",
    Bamu = "Bamu",
    Bass = "Bass",
    Batk = "Batk",
    Beng = "Beng",
    Blis = "Blis",
    Bopo = "Bopo",
    Brah = "Brah",
    Brai = "Brai",
    Bugi = "Bugi",
    Buhd = "Buhd",
    Cakm = "Cakm",
    Cans = "Cans",
    Cari = "Cari",
    Cham = "Cham",
    Cher = "Cher",
    Cirt = "Cirt",
    Copt = "Copt",
    Cprt = "Cprt",
    Cyrl = "Cyrl",
    Cyrs = "Cyrs",
    Deva = "Deva",
    Dsrt = "Dsrt",
    Dupl = "Dupl",
    Egyd = "Egyd",
    Egyh = "Egyh",
    Egyp = "Egyp",
    Elba = "Elba",
    Ethi = "Ethi",
    Geor = "Geor",
    Geok = "Geok",
    Glag = "Glag",
    Goth = "Goth",
    Gran = "Gran",
    Grek = "Grek",
    Gujr = "Gujr",
    Guru = "Guru",
    Hang = "Hang",
    Hani = "Hani",
    Hano = "Hano",
    Hans = "Hans",
    Hant = "Hant",
    Hebr = "Hebr",
    Hluw = "Hluw",
    Hmng = "Hmng",
    Hung = "Hung",
    Inds = "Inds",
    Ital = "Ital",
    Java = "Java",
    Jpan = "Jpan",
    Jurc = "Jurc",
    Kali = "Kali",
    Khar = "Khar",
    Khmr = "Khmr",
    Khoj = "Khoj",
    Knda = "Knda",
    Kpel = "Kpel",
    Kthi = "Kthi",
    Lana = "Lana",
    Laoo = "Laoo",
    Latf = "Latf",
    Latg = "Latg",
    Latn = "Latn",
    Lepc = "Lepc",
    Limb = "Limb",
    Lina = "Lina",
    Linb = "Linb",
    Lisu = "Lisu",
    Loma = "Loma",
    Lyci = "Lyci",
    Lydi = "Lydi",
    Mand = "Mand",
    Mani = "Mani",
    Maya = "Maya",
    Mend = "Mend",
    Merc = "Merc",
    Mero = "Mero",
    Mlym = "Mlym",
    Moon = "Moon",
    Mong = "Mong",
    Mroo = "Mroo",
    Mtei = "Mtei",
    Mymr = "Mymr",
    Narb = "Narb",
    Nbat = "Nbat",
    Nkgb = "Nkgb",
    Nkoo = "Nkoo",
    Nshu = "Nshu",
    Ogam = "Ogam",
    Olck = "Olck",
    Orkh = "Orkh",
    Orya = "Orya",
    Osma = "Osma",
    Palm = "Palm",
    Perm = "Perm",
    Phag = "Phag",
    Phli = "Phli",
    Phlp = "Phlp",
    Phlv = "Phlv",
    Phnx = "Phnx",
    Plrd = "Plrd",
    Prti = "Prti",
    Rjng = "Rjng",
    Roro = "Roro",
    Runr = "Runr",
    Samr = "Samr",
    Sara = "Sara",
    Sarb = "Sarb",
    Saur = "Saur",
    Sgnw = "Sgnw",
    Shaw = "Shaw",
    Shrd = "Shrd",
    Sind = "Sind",
    Sinh = "Sinh",
    Sora = "Sora",
    Sund = "Sund",
    Sylo = "Sylo",
    Syrc = "Syrc",
    Syre = "Syre",
    Syrj = "Syrj",
    Syrn = "Syrn",
    Tagb = "Tagb",
    Takr = "Takr",
    Tale = "Tale",
    Talu = "Talu",
    Taml = "Taml",
    Tang = "Tang",
    Tavt = "Tavt",
    Telu = "Telu",
    Teng = "Teng",
    Tfng = "Tfng",
    Tglg = "Tglg",
    Thaa = "Thaa",
    Thai = "Thai",
    Tibt = "Tibt",
    Tirh = "Tirh",
    Ugar = "Ugar",
    Vaii = "Vaii",
    Visp = "Visp",
    Wara = "Wara",
    Wole = "Wole",
    Xpeo = "Xpeo",
    Xsux = "Xsux",
    Yiii = "Yiii",
    Zmth = "Zmth",
    Zsym = "Zsym",
    Zyyy = "Zyyy",
}
#[wasm_bindgen]
///A CSS generic font family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GenericFamily {
    Standard = "standard",
    Sansserif = "sansserif",
    Serif = "serif",
    Fixed = "fixed",
    Cursive = "cursive",
    Fantasy = "fantasy",
    Math = "math",
}
#[wasm_bindgen]
///One ofnot_controllable: cannot be controlled by any extensioncontrolled_by_other_extensions: controlled by extensions with higher precedencecontrollable_by_this_extension: can be controlled by this extensioncontrolled_by_this_extension: controlled by this extension
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LevelOfControl {
    NotControllable = "not_controllable",
    ControlledByOtherExtensions = "controlled_by_other_extensions",
    ControllableByThisExtension = "controllable_by_this_extension",
    ControlledByThisExtension = "controlled_by_this_extension",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnFontChangedDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnFontChangedDetails;
    ///Get the `fontId` field of this object.
    #[wasm_bindgen(method, getter = "fontId")]
    pub fn get_font_id(this: &OnFontChangedDetails) -> String;
    ///Change the `fontId` field of this object.
    #[wasm_bindgen(method, setter = "fontId")]
    pub fn set_font_id(this: &OnFontChangedDetails, val: String);
    ///Get the `genericFamily` field of this object.
    #[wasm_bindgen(method, getter = "genericFamily")]
    pub fn get_generic_family(this: &OnFontChangedDetails) -> GenericFamily;
    ///Change the `genericFamily` field of this object.
    #[wasm_bindgen(method, setter = "genericFamily")]
    pub fn set_generic_family(this: &OnFontChangedDetails, val: GenericFamily);
    ///Get the `levelOfControl` field of this object.
    #[wasm_bindgen(method, getter = "levelOfControl")]
    pub fn get_level_of_control(this: &OnFontChangedDetails) -> LevelOfControl;
    ///Change the `levelOfControl` field of this object.
    #[wasm_bindgen(method, setter = "levelOfControl")]
    pub fn set_level_of_control(this: &OnFontChangedDetails, val: LevelOfControl);
    ///Get the `script` field of this object.
    #[wasm_bindgen(method, getter = "script")]
    pub fn get_script(this: &OnFontChangedDetails) -> Option<ScriptCode>;
    ///Change the `script` field of this object.
    #[wasm_bindgen(method, setter = "script")]
    pub fn set_script(this: &OnFontChangedDetails, val: ScriptCode);
}
impl OnFontChangedDetails {
    ///Construct a new `OnFontChangedDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_font_id()` instead."]
    pub fn font_id(&mut self, val: String) -> &mut Self {
        self.set_font_id(val);
        self
    }
    #[deprecated = "Use `set_generic_family()` instead."]
    pub fn generic_family(&mut self, val: GenericFamily) -> &mut Self {
        self.set_generic_family(val);
        self
    }
    #[deprecated = "Use `set_level_of_control()` instead."]
    pub fn level_of_control(&mut self, val: LevelOfControl) -> &mut Self {
        self.set_level_of_control(val);
        self
    }
    #[deprecated = "Use `set_script()` instead."]
    pub fn script(&mut self, val: ScriptCode) -> &mut Self {
        self.set_script(val);
        self
    }
}
impl Default for OnFontChangedDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnFontChangedDetails`.
pub struct OnFontChangedDetailsData {
    ///The font ID. See the description in getFont.
    pub font_id: String,
    ///The generic font family for which the font setting has changed.
    pub generic_family: GenericFamily,
    ///The level of control this extension has over the setting.
    pub level_of_control: LevelOfControl,
    ///The script code for which the font setting has changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<ScriptCode>,
}
#[cfg(feature = "serde")]
impl From<&OnFontChangedDetails> for OnFontChangedDetailsData {
    fn from(val: &OnFontChangedDetails) -> Self {
        Self {
            font_id: val.get_font_id(),
            generic_family: val.get_generic_family(),
            level_of_control: val.get_level_of_control(),
            script: val.get_script(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "OnDefaultFontSizeChangedDetails"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnDefaultFontSizeChangedDetails;
    ///Get the `levelOfControl` field of this object.
    #[wasm_bindgen(method, getter = "levelOfControl")]
    pub fn get_level_of_control(this: &OnDefaultFontSizeChangedDetails) -> LevelOfControl;
    ///Change the `levelOfControl` field of this object.
    #[wasm_bindgen(method, setter = "levelOfControl")]
    pub fn set_level_of_control(this: &OnDefaultFontSizeChangedDetails, val: LevelOfControl);
    ///Get the `pixelSize` field of this object.
    #[wasm_bindgen(method, getter = "pixelSize")]
    pub fn get_pixel_size(this: &OnDefaultFontSizeChangedDetails) -> i32;
    ///Change the `pixelSize` field of this object.
    #[wasm_bindgen(method, setter = "pixelSize")]
    pub fn set_pixel_size(this: &OnDefaultFontSizeChangedDetails, val: i32);
}
impl OnDefaultFontSizeChangedDetails {
    ///Construct a new `OnDefaultFontSizeChangedDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_level_of_control()` instead."]
    pub fn level_of_control(&mut self, val: LevelOfControl) -> &mut Self {
        self.set_level_of_control(val);
        self
    }
    #[deprecated = "Use `set_pixel_size()` instead."]
    pub fn pixel_size(&mut self, val: i32) -> &mut Self {
        self.set_pixel_size(val);
        self
    }
}
impl Default for OnDefaultFontSizeChangedDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnDefaultFontSizeChangedDetails`.
pub struct OnDefaultFontSizeChangedDetailsData {
    ///The level of control this extension has over the setting.
    pub level_of_control: LevelOfControl,
    ///The font size in pixels.
    pub pixel_size: i32,
}
#[cfg(feature = "serde")]
impl From<&OnDefaultFontSizeChangedDetails> for OnDefaultFontSizeChangedDetailsData {
    fn from(val: &OnDefaultFontSizeChangedDetails) -> Self {
        Self {
            level_of_control: val.get_level_of_control(),
            pixel_size: val.get_pixel_size(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "OnDefaultFixedFontSizeChangedDetails"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnDefaultFixedFontSizeChangedDetails;
    ///Get the `levelOfControl` field of this object.
    #[wasm_bindgen(method, getter = "levelOfControl")]
    pub fn get_level_of_control(this: &OnDefaultFixedFontSizeChangedDetails) -> LevelOfControl;
    ///Change the `levelOfControl` field of this object.
    #[wasm_bindgen(method, setter = "levelOfControl")]
    pub fn set_level_of_control(this: &OnDefaultFixedFontSizeChangedDetails, val: LevelOfControl);
    ///Get the `pixelSize` field of this object.
    #[wasm_bindgen(method, getter = "pixelSize")]
    pub fn get_pixel_size(this: &OnDefaultFixedFontSizeChangedDetails) -> i32;
    ///Change the `pixelSize` field of this object.
    #[wasm_bindgen(method, setter = "pixelSize")]
    pub fn set_pixel_size(this: &OnDefaultFixedFontSizeChangedDetails, val: i32);
}
impl OnDefaultFixedFontSizeChangedDetails {
    ///Construct a new `OnDefaultFixedFontSizeChangedDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_level_of_control()` instead."]
    pub fn level_of_control(&mut self, val: LevelOfControl) -> &mut Self {
        self.set_level_of_control(val);
        self
    }
    #[deprecated = "Use `set_pixel_size()` instead."]
    pub fn pixel_size(&mut self, val: i32) -> &mut Self {
        self.set_pixel_size(val);
        self
    }
}
impl Default for OnDefaultFixedFontSizeChangedDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnDefaultFixedFontSizeChangedDetails`.
pub struct OnDefaultFixedFontSizeChangedDetailsData {
    ///The level of control this extension has over the setting.
    pub level_of_control: LevelOfControl,
    ///The font size in pixels.
    pub pixel_size: i32,
}
#[cfg(feature = "serde")]
impl From<&OnDefaultFixedFontSizeChangedDetails> for OnDefaultFixedFontSizeChangedDetailsData {
    fn from(val: &OnDefaultFixedFontSizeChangedDetails) -> Self {
        Self {
            level_of_control: val.get_level_of_control(),
            pixel_size: val.get_pixel_size(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "OnMinimumFontSizeChangedDetails"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnMinimumFontSizeChangedDetails;
    ///Get the `levelOfControl` field of this object.
    #[wasm_bindgen(method, getter = "levelOfControl")]
    pub fn get_level_of_control(this: &OnMinimumFontSizeChangedDetails) -> LevelOfControl;
    ///Change the `levelOfControl` field of this object.
    #[wasm_bindgen(method, setter = "levelOfControl")]
    pub fn set_level_of_control(this: &OnMinimumFontSizeChangedDetails, val: LevelOfControl);
    ///Get the `pixelSize` field of this object.
    #[wasm_bindgen(method, getter = "pixelSize")]
    pub fn get_pixel_size(this: &OnMinimumFontSizeChangedDetails) -> i32;
    ///Change the `pixelSize` field of this object.
    #[wasm_bindgen(method, setter = "pixelSize")]
    pub fn set_pixel_size(this: &OnMinimumFontSizeChangedDetails, val: i32);
}
impl OnMinimumFontSizeChangedDetails {
    ///Construct a new `OnMinimumFontSizeChangedDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_level_of_control()` instead."]
    pub fn level_of_control(&mut self, val: LevelOfControl) -> &mut Self {
        self.set_level_of_control(val);
        self
    }
    #[deprecated = "Use `set_pixel_size()` instead."]
    pub fn pixel_size(&mut self, val: i32) -> &mut Self {
        self.set_pixel_size(val);
        self
    }
}
impl Default for OnMinimumFontSizeChangedDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnMinimumFontSizeChangedDetails`.
pub struct OnMinimumFontSizeChangedDetailsData {
    ///The level of control this extension has over the setting.
    pub level_of_control: LevelOfControl,
    ///The font size in pixels.
    pub pixel_size: i32,
}
#[cfg(feature = "serde")]
impl From<&OnMinimumFontSizeChangedDetails> for OnMinimumFontSizeChangedDetailsData {
    fn from(val: &OnMinimumFontSizeChangedDetails) -> Self {
        Self {
            level_of_control: val.get_level_of_control(),
            pixel_size: val.get_pixel_size(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ClearFontDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ClearFontDetails;
    ///Get the `genericFamily` field of this object.
    #[wasm_bindgen(method, getter = "genericFamily")]
    pub fn get_generic_family(this: &ClearFontDetails) -> GenericFamily;
    ///Change the `genericFamily` field of this object.
    #[wasm_bindgen(method, setter = "genericFamily")]
    pub fn set_generic_family(this: &ClearFontDetails, val: GenericFamily);
    ///Get the `script` field of this object.
    #[wasm_bindgen(method, getter = "script")]
    pub fn get_script(this: &ClearFontDetails) -> Option<ScriptCode>;
    ///Change the `script` field of this object.
    #[wasm_bindgen(method, setter = "script")]
    pub fn set_script(this: &ClearFontDetails, val: ScriptCode);
}
impl ClearFontDetails {
    ///Construct a new `ClearFontDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_generic_family()` instead."]
    pub fn generic_family(&mut self, val: GenericFamily) -> &mut Self {
        self.set_generic_family(val);
        self
    }
    #[deprecated = "Use `set_script()` instead."]
    pub fn script(&mut self, val: ScriptCode) -> &mut Self {
        self.set_script(val);
        self
    }
}
impl Default for ClearFontDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ClearFontDetails`.
pub struct ClearFontDetailsData {
    ///The generic font family for which the font should be cleared.
    pub generic_family: GenericFamily,
    ///The script for which the font should be cleared. If omitted, the global script font setting is cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<ScriptCode>,
}
#[cfg(feature = "serde")]
impl From<&ClearFontDetails> for ClearFontDetailsData {
    fn from(val: &ClearFontDetails) -> Self {
        Self {
            generic_family: val.get_generic_family(),
            script: val.get_script(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetFontDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GetFontDetails;
    ///Get the `genericFamily` field of this object.
    #[wasm_bindgen(method, getter = "genericFamily")]
    pub fn get_generic_family(this: &GetFontDetails) -> GenericFamily;
    ///Change the `genericFamily` field of this object.
    #[wasm_bindgen(method, setter = "genericFamily")]
    pub fn set_generic_family(this: &GetFontDetails, val: GenericFamily);
    ///Get the `script` field of this object.
    #[wasm_bindgen(method, getter = "script")]
    pub fn get_script(this: &GetFontDetails) -> Option<ScriptCode>;
    ///Change the `script` field of this object.
    #[wasm_bindgen(method, setter = "script")]
    pub fn set_script(this: &GetFontDetails, val: ScriptCode);
}
impl GetFontDetails {
    ///Construct a new `GetFontDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_generic_family()` instead."]
    pub fn generic_family(&mut self, val: GenericFamily) -> &mut Self {
        self.set_generic_family(val);
        self
    }
    #[deprecated = "Use `set_script()` instead."]
    pub fn script(&mut self, val: ScriptCode) -> &mut Self {
        self.set_script(val);
        self
    }
}
impl Default for GetFontDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `GetFontDetails`.
pub struct GetFontDetailsData {
    ///The generic font family for which the font should be retrieved.
    pub generic_family: GenericFamily,
    ///The script for which the font should be retrieved. If omitted, the font setting for the global script (script code "Zyyy") is retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<ScriptCode>,
}
#[cfg(feature = "serde")]
impl From<&GetFontDetails> for GetFontDetailsData {
    fn from(val: &GetFontDetails) -> Self {
        Self {
            generic_family: val.get_generic_family(),
            script: val.get_script(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SetFontDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetFontDetails;
    ///Get the `fontId` field of this object.
    #[wasm_bindgen(method, getter = "fontId")]
    pub fn get_font_id(this: &SetFontDetails) -> String;
    ///Change the `fontId` field of this object.
    #[wasm_bindgen(method, setter = "fontId")]
    pub fn set_font_id(this: &SetFontDetails, val: String);
    ///Get the `genericFamily` field of this object.
    #[wasm_bindgen(method, getter = "genericFamily")]
    pub fn get_generic_family(this: &SetFontDetails) -> GenericFamily;
    ///Change the `genericFamily` field of this object.
    #[wasm_bindgen(method, setter = "genericFamily")]
    pub fn set_generic_family(this: &SetFontDetails, val: GenericFamily);
    ///Get the `script` field of this object.
    #[wasm_bindgen(method, getter = "script")]
    pub fn get_script(this: &SetFontDetails) -> Option<ScriptCode>;
    ///Change the `script` field of this object.
    #[wasm_bindgen(method, setter = "script")]
    pub fn set_script(this: &SetFontDetails, val: ScriptCode);
}
impl SetFontDetails {
    ///Construct a new `SetFontDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_font_id()` instead."]
    pub fn font_id(&mut self, val: String) -> &mut Self {
        self.set_font_id(val);
        self
    }
    #[deprecated = "Use `set_generic_family()` instead."]
    pub fn generic_family(&mut self, val: GenericFamily) -> &mut Self {
        self.set_generic_family(val);
        self
    }
    #[deprecated = "Use `set_script()` instead."]
    pub fn script(&mut self, val: ScriptCode) -> &mut Self {
        self.set_script(val);
        self
    }
}
impl Default for SetFontDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SetFontDetails`.
pub struct SetFontDetailsData {
    ///The font ID. The empty string means to fallback to the global script font setting.
    pub font_id: String,
    ///The generic font family for which the font should be set.
    pub generic_family: GenericFamily,
    ///The script code which the font should be set. If omitted, the font setting for the global script (script code "Zyyy") is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<ScriptCode>,
}
#[cfg(feature = "serde")]
impl From<&SetFontDetails> for SetFontDetailsData {
    fn from(val: &SetFontDetails) -> Self {
        Self {
            font_id: val.get_font_id(),
            generic_family: val.get_generic_family(),
            script: val.get_script(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SetDefaultFontSizeDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetDefaultFontSizeDetails;
    ///Get the `pixelSize` field of this object.
    #[wasm_bindgen(method, getter = "pixelSize")]
    pub fn get_pixel_size(this: &SetDefaultFontSizeDetails) -> i32;
    ///Change the `pixelSize` field of this object.
    #[wasm_bindgen(method, setter = "pixelSize")]
    pub fn set_pixel_size(this: &SetDefaultFontSizeDetails, val: i32);
}
impl SetDefaultFontSizeDetails {
    ///Construct a new `SetDefaultFontSizeDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_pixel_size()` instead."]
    pub fn pixel_size(&mut self, val: i32) -> &mut Self {
        self.set_pixel_size(val);
        self
    }
}
impl Default for SetDefaultFontSizeDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SetDefaultFontSizeDetails`.
pub struct SetDefaultFontSizeDetailsData {
    ///The font size in pixels.
    pub pixel_size: i32,
}
#[cfg(feature = "serde")]
impl From<&SetDefaultFontSizeDetails> for SetDefaultFontSizeDetailsData {
    fn from(val: &SetDefaultFontSizeDetails) -> Self {
        Self {
            pixel_size: val.get_pixel_size(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "SetDefaultFixedFontSizeDetails"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetDefaultFixedFontSizeDetails;
    ///Get the `pixelSize` field of this object.
    #[wasm_bindgen(method, getter = "pixelSize")]
    pub fn get_pixel_size(this: &SetDefaultFixedFontSizeDetails) -> i32;
    ///Change the `pixelSize` field of this object.
    #[wasm_bindgen(method, setter = "pixelSize")]
    pub fn set_pixel_size(this: &SetDefaultFixedFontSizeDetails, val: i32);
}
impl SetDefaultFixedFontSizeDetails {
    ///Construct a new `SetDefaultFixedFontSizeDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_pixel_size()` instead."]
    pub fn pixel_size(&mut self, val: i32) -> &mut Self {
        self.set_pixel_size(val);
        self
    }
}
impl Default for SetDefaultFixedFontSizeDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SetDefaultFixedFontSizeDetails`.
pub struct SetDefaultFixedFontSizeDetailsData {
    ///The font size in pixels.
    pub pixel_size: i32,
}
#[cfg(feature = "serde")]
impl From<&SetDefaultFixedFontSizeDetails> for SetDefaultFixedFontSizeDetailsData {
    fn from(val: &SetDefaultFixedFontSizeDetails) -> Self {
        Self {
            pixel_size: val.get_pixel_size(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SetMinimumFontSizeDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetMinimumFontSizeDetails;
    ///Get the `pixelSize` field of this object.
    #[wasm_bindgen(method, getter = "pixelSize")]
    pub fn get_pixel_size(this: &SetMinimumFontSizeDetails) -> i32;
    ///Change the `pixelSize` field of this object.
    #[wasm_bindgen(method, setter = "pixelSize")]
    pub fn set_pixel_size(this: &SetMinimumFontSizeDetails, val: i32);
}
impl SetMinimumFontSizeDetails {
    ///Construct a new `SetMinimumFontSizeDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_pixel_size()` instead."]
    pub fn pixel_size(&mut self, val: i32) -> &mut Self {
        self.set_pixel_size(val);
        self
    }
}
impl Default for SetMinimumFontSizeDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SetMinimumFontSizeDetails`.
pub struct SetMinimumFontSizeDetailsData {
    ///The font size in pixels.
    pub pixel_size: i32,
}
#[cfg(feature = "serde")]
impl From<&SetMinimumFontSizeDetails> for SetMinimumFontSizeDetailsData {
    fn from(val: &SetMinimumFontSizeDetails) -> Self {
        Self {
            pixel_size: val.get_pixel_size(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Clears the font set by this extension, if any.
    #[wasm_bindgen(js_namespace = ["chrome", "fontSettings"], js_name = "clearFont")]
    pub fn clear_font(details: Object) -> Promise;
    ///Gets the font for a given script and generic font family.
    #[wasm_bindgen(js_namespace = ["chrome", "fontSettings"], js_name = "getFont")]
    pub fn get_font(details: Object) -> Promise;
    ///Sets the font for a given script and generic font family.
    #[wasm_bindgen(js_namespace = ["chrome", "fontSettings"], js_name = "setFont")]
    pub fn set_font(details: Object) -> Promise;
    ///Gets a list of fonts on the system.
    #[wasm_bindgen(js_namespace = ["chrome", "fontSettings"], js_name = "getFontList")]
    pub fn get_font_list() -> Promise;
    ///Clears the default font size set by this extension, if any.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fontSettings"],
        js_name = "clearDefaultFontSize"
    )]
    pub fn clear_default_font_size(details: Option<Object>) -> Promise;
    ///Gets the default font size.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fontSettings"],
        js_name = "getDefaultFontSize"
    )]
    pub fn get_default_font_size(details: Option<Object>) -> Promise;
    ///Sets the default font size.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fontSettings"],
        js_name = "setDefaultFontSize"
    )]
    pub fn set_default_font_size(details: Object) -> Promise;
    ///Clears the default fixed font size set by this extension, if any.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fontSettings"],
        js_name = "clearDefaultFixedFontSize"
    )]
    pub fn clear_default_fixed_font_size(details: Option<Object>) -> Promise;
    ///Gets the default size for fixed width fonts.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fontSettings"],
        js_name = "getDefaultFixedFontSize"
    )]
    pub fn get_default_fixed_font_size(details: Option<Object>) -> Promise;
    ///Sets the default size for fixed width fonts.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fontSettings"],
        js_name = "setDefaultFixedFontSize"
    )]
    pub fn set_default_fixed_font_size(details: Object) -> Promise;
    ///Clears the minimum font size set by this extension, if any.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fontSettings"],
        js_name = "clearMinimumFontSize"
    )]
    pub fn clear_minimum_font_size(details: Option<Object>) -> Promise;
    ///Gets the minimum font size.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fontSettings"],
        js_name = "getMinimumFontSize"
    )]
    pub fn get_minimum_font_size(details: Option<Object>) -> Promise;
    ///Sets the minimum font size.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fontSettings"],
        js_name = "setMinimumFontSize"
    )]
    pub fn set_minimum_font_size(details: Object) -> Promise;
    ///Fired when a font setting changes.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fontSettings",
        "onFontChanged"],
        js_name = "addListener"
    )]
    pub fn on_font_changed_add_listener(callback: &Function);
    ///Fired when the default font size setting changes.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fontSettings",
        "onDefaultFontSizeChanged"],
        js_name = "addListener"
    )]
    pub fn on_default_font_size_changed_add_listener(callback: &Function);
    ///Fired when the default fixed font size setting changes.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fontSettings",
        "onDefaultFixedFontSizeChanged"],
        js_name = "addListener"
    )]
    pub fn on_default_fixed_font_size_changed_add_listener(callback: &Function);
    ///Fired when the minimum font size setting changes.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fontSettings",
        "onMinimumFontSizeChanged"],
        js_name = "addListener"
    )]
    pub fn on_minimum_font_size_changed_add_listener(callback: &Function);
}
