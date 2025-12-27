//! Rust types matching chrome-types JSON schema.
//!
//! These types are used to deserialize the chrome-api.json file produced by
//! the chrome-types `prepare.js` script.

use serde::Deserialize;
use std::collections::HashMap;

/// Top-level structure of the chrome-api.json file.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessedApiData {
    pub head_revision: String,
    pub definitions_revision: String,
    pub version: Option<VersionInfo>,
    pub when: String,
    pub feature: HashMap<String, FeatureSpecOrArray>,
    pub api: HashMap<String, NamespaceSpec>,
}

#[derive(Debug, Deserialize)]
pub struct VersionInfo {
    pub revision: String,
    pub version: String,
}

/// Feature specification can be a single spec or array of specs.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FeatureSpecOrArray {
    Single(Box<FeatureSpec>),
    Multiple(Vec<FeatureSpec>),
}

/// Chrome release channel.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Channel {
    Stable,
    Beta,
    Dev,
    Canary,
    Trunk,
}

/// Platform identifiers.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Chromeos,
    DesktopAndroid,
    Linux,
    Mac,
    Win,
    #[serde(other)]
    Unknown,
}

/// Extension context types.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Context {
    ContentScript,
    OffscreenExtension,
    PrivilegedExtension,
    PrivilegedWebPage,
    UnprivilegedExtension,
    UserScript,
    WebPage,
    Webui,
    WebuiUntrusted,
    #[serde(other)]
    Unknown,
}

/// Extension types.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionType {
    Extension,
    HostedApp,
    LegacyPackagedApp,
    PlatformApp,
    SharedModule,
    Theme,
    LoginScreenExtension,
    ChromeosSystemExtension,
    #[serde(other)]
    Unknown,
}

/// Session types.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum SessionType {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "kiosk")]
    Kiosk,
    #[serde(rename = "kiosk.autolaunched")]
    KioskAutolaunched,
}

/// Location restrictions.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Location {
    Component,
    ExternalComponent,
    Policy,
    Unpacked,
}

/// Contexts can be a list or the literal "all".
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ContextsOrAll {
    All(AllLiteral),
    Contexts(Vec<Context>),
}

/// Extension types can be a list or the literal "all".
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ExtensionTypesOrAll {
    All(AllLiteral),
    Types(Vec<ExtensionType>),
}

/// The literal string "all".
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum AllLiteral {
    #[serde(rename = "all")]
    All,
}

/// Feature specification describing API availability.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct FeatureSpec {
    pub allowlist: Option<Vec<String>>,
    pub blocklist: Option<Vec<String>>,
    pub channel: Option<Channel>,
    pub command_line_switch: Option<String>,
    pub component_extensions_auto_granted: Option<bool>,
    pub contexts: Option<ContextsOrAll>,
    pub default_parent: Option<bool>,
    pub dependencies: Option<Vec<String>>,
    pub disallow_for_service_workers: Option<bool>,
    pub extension_types: Option<ExtensionTypesOrAll>,
    pub feature_flag: Option<String>,
    pub location: Option<Location>,
    pub internal: Option<bool>,
    pub matches: Option<Vec<String>>,
    pub max_manifest_version: Option<i32>,
    pub min_manifest_version: Option<i32>,
    pub noparent: Option<bool>,
    pub platforms: Option<Vec<Platform>>,
    pub session_types: Option<Vec<SessionType>>,
    // Additional fields that appear in the JSON
    pub source: Option<String>,
    pub alias: Option<String>,
    pub requires_delegated_availability_check: Option<bool>,
    pub developer_mode_only: Option<bool>,
    pub nocompile: Option<bool>,
    pub required_buildflags: Option<Vec<String>>,
}

/// Primitive types understood by Chrome's extensions.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PrimitiveType {
    Void,
    Undefined,
    Never,
    Array,
    Any,
    Int64,
    Binary,
    Boolean,
    Integer,
    Double,
    Number,
    String,
    Object,
    Function,
}

/// Enum value can be a string, number, or object with name/description.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum EnumValue {
    String(String),
    Number(i64),
    Named(NamedEnumValue),
}

#[derive(Debug, Clone, Deserialize)]
pub struct NamedEnumValue {
    pub name: String,
    pub description: Option<String>,
}

/// Value field can have various forms.
///
/// This represents constant values or property references in the Chrome API schema.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ValueSpec {
    /// Integer constant (e.g., `0`, `-1`, `100`)
    Integer(i64),
    /// String constant (e.g., `"_dynamic"`, `"_session"`)
    String(String),
    /// Property reference: `["propertyName"]`
    PropertyRef(PropertyRefValue),
    /// Inline TypeSpec (a property definition, often for a property literally named "value")
    InlineType(Box<InlineValueTypeSpec>),
}

/// Property reference value - an array containing a property name and optionally a type hint.
/// Examples:
/// - `["sync"]` - simple property name reference
/// - `["spokenFeedback", {"type": "boolean"}]` - property name with inline type
#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "Vec<serde_json::Value>")]
pub struct PropertyRefValue {
    /// The property name being referenced
    pub property_name: String,
    /// Optional inline type specification for the property
    pub type_hint: Option<PropertyTypeHint>,
}

impl TryFrom<Vec<serde_json::Value>> for PropertyRefValue {
    type Error = &'static str;

    fn try_from(arr: Vec<serde_json::Value>) -> Result<Self, Self::Error> {
        if arr.is_empty() {
            return Err("Empty array in PropertyRefValue");
        }

        let property_name = arr[0]
            .as_str()
            .ok_or("First element must be a string")?
            .to_string();

        let type_hint = if arr.len() > 1 {
            serde_json::from_value(arr[1].clone()).ok()
        } else {
            None
        };

        Ok(PropertyRefValue {
            property_name,
            type_hint,
        })
    }
}

/// Type hint for a property reference - a simplified type specification.
/// Only contains: `$ref`, `enum`, or `type`.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct PropertyTypeHint {
    #[serde(rename = "type")]
    pub type_: Option<PrimitiveType>,
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    #[serde(rename = "enum")]
    pub enum_: Option<Vec<EnumValue>>,
}

/// Inline type specification used within ValueSpec for object values.
/// This is a subset of TypeSpec fields that appear when the value is an object
/// (typically a property definition for a field literally named "value").
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct InlineValueTypeSpec {
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<PrimitiveType>,
    pub optional: Option<bool>,
    pub choices: Option<Vec<ChoiceTypeSpec>>,
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    #[serde(rename = "isInstanceOf")]
    pub is_instance_of: Option<String>,
    pub nocompile: Option<bool>,
    pub items: Option<Box<ChoiceTypeSpec>>,
}

/// Type specification used in choices arrays and items.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct ChoiceTypeSpec {
    #[serde(rename = "type")]
    pub type_: Option<PrimitiveType>,
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    pub items: Option<Box<ChoiceTypeSpec>>,
}

/// Compiler options for a namespace.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct CompilerOptions {
    /// Path to C++ implementation header file
    pub implemented_in: Option<String>,
    /// Whether to generate error messages
    pub generate_error_messages: Option<bool>,
    /// Whether to generate type functions
    pub generate_type_functions: Option<bool>,
}

/// Documentation options for a namespace (rarely used, structure unknown).
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct DocumentationOptions {
    // Currently no known fields - placeholder for future use
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Numeric value that can be integer or floating point.
/// Preserves the original type from JSON.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(untagged)]
pub enum NumericValue {
    Integer(i64),
    Float(f64),
}

impl NumericValue {
    pub fn as_f64(&self) -> f64 {
        match self {
            NumericValue::Integer(i) => *i as f64,
            NumericValue::Float(f) => *f,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            NumericValue::Integer(i) => Some(*i),
            NumericValue::Float(f) => {
                if f.fract() == 0.0 {
                    Some(*f as i64)
                } else {
                    None
                }
            }
        }
    }
}

/// Type specification - the core type definition structure.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct TypeSpec {
    // Identity
    pub id: Option<String>,
    pub name: Option<String>,
    pub optional: Option<bool>,

    // Type info
    #[serde(rename = "type")]
    pub type_: Option<PrimitiveType>,

    // References
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    #[serde(rename = "isInstanceOf")]
    pub is_instance_of: Option<String>,

    // Value (for constants)
    pub value: Option<ValueSpec>,

    // Numeric constraints
    pub minimum: Option<NumericValue>,
    pub maximum: Option<NumericValue>,

    // Object type
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<Box<TypeSpec>>,

    // Function serialization
    #[serde(rename = "serializableFunction")]
    pub serializable_function: Option<bool>,
    pub serialized_type: Option<PrimitiveType>,

    // Function signature
    pub parameters: Option<Vec<TypeSpec>>,
    pub returns: Option<Box<TypeSpec>>,
    pub returns_async: Option<Box<TypeSpec>>,

    // Enum/union types
    #[serde(rename = "enum")]
    pub enum_: Option<Vec<EnumValue>>,
    pub choices: Option<Vec<TypeSpec>>,

    // Array type
    pub items: Option<Box<TypeSpec>>,
    #[serde(rename = "minItems")]
    pub min_items: Option<i32>,
    #[serde(rename = "maxItems")]
    pub max_items: Option<i32>,

    // Documentation
    pub description: Option<String>,
    pub deprecated: Option<DeprecatedValue>,
    pub nodoc: Option<BoolOrString>,
    pub jsexterns: Option<String>,
    pub platforms: Option<Vec<String>>,
    pub noinline_doc: Option<BoolOrString>,

    // Nested content
    pub properties: Option<HashMap<String, TypeSpec>>,
    pub functions: Option<Vec<TypeSpec>>,
    pub events: Option<Vec<EventSpec>>,

    // Promise support
    pub does_not_support_promises: Option<String>,

    // Internal marker
    #[serde(rename = "_event")]
    pub _event: Option<bool>,

    // Additional fields found in actual JSON
    #[serde(rename = "customBindings")]
    pub custom_bindings: Option<String>,
    pub js_module: Option<String>,
    #[serde(rename = "maximumManifestVersion")]
    pub maximum_manifest_version: Option<i32>,
    pub min_version: Option<String>,
    pub nocompile: Option<bool>,
    /// List of required property names (for object types).
    pub required: Option<Vec<String>>,
}

impl TypeSpec {
    /// Check if this type/function is marked as not for documentation.
    pub fn is_nodoc(&self) -> bool {
        self.nodoc.as_ref().is_some_and(|n| n.is_true())
    }
}

/// Some boolean fields can be "true" or "True" strings.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum BoolOrString {
    Bool(bool),
    String(String),
}

impl BoolOrString {
    pub fn is_true(&self) -> bool {
        match self {
            BoolOrString::Bool(b) => *b,
            BoolOrString::String(s) => s.eq_ignore_ascii_case("true"),
        }
    }
}

/// Deprecated field can be a boolean (true = deprecated) or a string (deprecation message).
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum DeprecatedValue {
    Bool(bool),
    Message(String),
}

impl DeprecatedValue {
    pub fn is_deprecated(&self) -> bool {
        match self {
            DeprecatedValue::Bool(b) => *b,
            DeprecatedValue::Message(_) => true,
        }
    }

    pub fn message(&self) -> Option<&str> {
        match self {
            DeprecatedValue::Bool(_) => None,
            DeprecatedValue::Message(s) => Some(s),
        }
    }
}

/// Event specification - extends TypeSpec with event-specific fields.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct EventSpec {
    // Required for events
    pub name: String,

    // Usually "function" for events
    #[serde(rename = "type")]
    pub type_: Option<PrimitiveType>,

    // Event-specific
    #[serde(rename = "extraParameters")]
    pub extra_parameters: Option<Vec<TypeSpec>>,
    pub filters: Option<Vec<TypeSpec>>,
    pub options: Option<EventOptions>,

    // Inherited from TypeSpec
    pub description: Option<String>,
    pub deprecated: Option<DeprecatedValue>,
    pub parameters: Option<Vec<TypeSpec>>,
    pub returns: Option<Box<TypeSpec>>,
    pub returns_async: Option<Box<TypeSpec>>,
    pub nodoc: Option<BoolOrString>,
    pub platforms: Option<Vec<String>>,

    // Additional fields found in actual JSON
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    pub nocompile: Option<bool>,
}

impl EventSpec {
    /// Check if this event is marked as not for documentation.
    pub fn is_nodoc(&self) -> bool {
        self.nodoc.as_ref().is_some_and(|n| n.is_true())
    }
}

/// Event options.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct EventOptions {
    pub unmanaged: Option<bool>,
    #[serde(rename = "maxListeners")]
    pub max_listeners: Option<i32>,
    #[serde(rename = "supportsFilters")]
    pub supports_filters: Option<bool>,
    #[serde(rename = "supportsListeners")]
    pub supports_listeners: Option<bool>,
    #[serde(rename = "supportsRules")]
    pub supports_rules: Option<bool>,
    pub conditions: Option<Vec<String>>,
    pub actions: Option<Vec<String>>,
    #[serde(rename = "supportsDom")]
    pub supports_dom: Option<bool>,
    #[serde(rename = "supportsLazyListeners")]
    pub supports_lazy_listeners: Option<bool>,
}

/// Namespace specification - represents a chrome.* namespace.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct NamespaceSpec {
    pub namespace: String,
    pub description: Option<String>,
    pub deprecated: Option<DeprecatedValue>,
    pub nodoc: Option<BoolOrString>,
    pub jsexterns: Option<String>,
    pub platforms: Option<Vec<String>>,

    // Content
    pub types: Option<Vec<TypeSpec>>,
    pub properties: Option<HashMap<String, TypeSpec>>,
    pub functions: Option<Vec<TypeSpec>>,
    pub events: Option<Vec<EventSpec>>,

    // Rarely used
    pub manifest_keys: Option<HashMap<String, TypeSpec>>,
    pub documentation_options: Option<DocumentationOptions>,
    pub compiler_options: Option<CompilerOptions>,
    pub internal: Option<bool>,
    pub dependencies: Option<Vec<String>>,
    pub nocompile: Option<bool>,
    pub unprivileged: Option<bool>,
}

impl NamespaceSpec {
    /// Check if this namespace has any exportable content (types, functions, or events).
    pub fn has_exportable_content(&self) -> bool {
        let has_types = self.types.as_ref().is_some_and(|t| {
            t.iter()
                .any(|ts| ts.id.as_ref().is_some_and(|id| !id.starts_with('_')))
        });
        let has_functions = self.functions.as_ref().is_some_and(|f| !f.is_empty());
        let has_events = self.events.as_ref().is_some_and(|e| !e.is_empty());

        has_types || has_functions || has_events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sample() {
        let json = r#"{
            "headRevision": "abc123",
            "definitionsRevision": "abc123",
            "version": null,
            "when": "2024-01-01",
            "feature": {},
            "api": {}
        }"#;

        let data: ProcessedApiData = serde_json::from_str(json).unwrap();
        assert_eq!(data.head_revision, "abc123");
    }
}
