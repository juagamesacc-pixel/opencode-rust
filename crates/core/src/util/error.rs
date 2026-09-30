// source: src/util/error.ts — exports: NamedError (abstract class + factories)
//
// The TS `NamedError` is an abstract Error subclass whose concrete factories
// carry: a `name` tag, a `data` field, static `Schema`, static `tag`,
// `isInstance`, `schema()`, and `toObject() = { name, data }`. In Rust the
// error *type* surface is the name tag itself; `NamedError` mirrors the
// runtime observable: `{ name, data }` + `hasName`/`isInstance` checks.
//
// The `schema`/`EffectSchema` statics are Effect-schema descriptors (no
// runtime value beyond the `{ name, data }` shape); they are mirrored as the
// `data` value only.

use serde_json::Value;

/// Port of `NamedError.toObject()` — `{ name, data }` in that field order.
#[derive(Debug, Clone, PartialEq)]
pub struct NamedError {
    pub name: String,
    pub data: Value,
}

impl NamedError {
    /// source: `NamedError.create(name, fields)` — builds an instance with the
    /// given tag and data object (the schema-annotated `{ name, data }`).
    pub fn create(name: impl Into<String>, data: Value) -> Self {
        NamedError {
            name: name.into(),
            data,
        }
    }

    /// source: `NamedError.hasName(error, name)` — typeof object && `name` key
    /// matches. For typed errors this is the `isInstance` check.
    pub fn has_name(error: &dyn std::any::Any, name: &str) -> bool {
        if let Some(e) = error.downcast_ref::<NamedError>() {
            return e.name == name;
        }
        if let Some(e) = error.downcast_ref::<String>() {
            return e == name;
        }
        false
    }

    /// source: `isInstance(input)` — name-tag match.
    pub fn is_instance(&self, name: &str) -> bool {
        self.name == name
    }

    /// source: `toObject()` — `{ name, data }`.
    pub fn to_object(&self) -> (String, Value) {
        (self.name.clone(), self.data.clone())
    }
}

/// source: `NamedError.Unknown = NamedError.create("UnknownError", { message, ref? })`.
pub fn unknown(message: String, reference: Option<String>) -> NamedError {
    let mut data = serde_json::Map::new();
    data.insert("message".to_string(), Value::String(message));
    if let Some(r) = reference {
        data.insert("ref".to_string(), Value::String(r));
    }
    NamedError {
        name: "UnknownError".to_string(),
        data: Value::Object(data),
    }
}
