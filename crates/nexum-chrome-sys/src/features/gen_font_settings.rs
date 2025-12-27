#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
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
#[wasm_bindgen]
///An ISO 15924 script code. The default, or global, script is represented by script code "Zyyy".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
pub enum LevelOfControl {
    NotControllable = "not_controllable",
    ControlledByOtherExtensions = "controlled_by_other_extensions",
    ControllableByThisExtension = "controllable_by_this_extension",
    ControlledByThisExtension = "controlled_by_this_extension",
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
