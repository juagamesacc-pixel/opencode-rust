//! Port of `packages/schema/src/schema.ts`.
//!
//! Primitives and schema-building combinators shared by every schema module.
//! 1:1 exact translation — same names/signatures/behavior/edge-cases/keys/
//! defaults/ordering. Source is spec.
//!
//! Mapping notes (Effect Schema → serde, per plan §3.2):
//! - `PositiveInt` / `NonNegativeInt` are `Schema.Int` refinements; the Rust
//!   wire type stays `i64` (never narrowed, so negative inputs still reach the
//!   validation boundary instead of failing at deserialization).
//! - `RelativePath` / `AbsolutePath` are branded strings; the wire format
//!   stays a bare string (`#[serde(transparent)]`).
//! - The package `optional(...)` helper means "omit the key when absent":
//!   every `Option<T>` field built with it carries
//!   `#[serde(skip_serializing_if = "Option::is_none")]`.
//! - `DateTimeUtcFromMillis` is `Finite` millis on the wire. `chrono` is NOT
//!   added (new deps forbidden); millis travel as integers and RFC3339
//!   rendering stays at the domain boundary.

#![allow(non_snake_case)]

use std::ops::Deref;

/// `Schema.Int` refined with `Schema.isGreaterThan(0)` (`schema.ts`).
///
/// Wire type is `i64` (never narrowed to `u64`, so negative inputs still reach
/// the validation boundary — see `PositiveInt_is_valid` — instead of failing
/// at deserialization).
pub type PositiveInt = i64;

/// Port of the `Schema.isGreaterThan(0)` refinement on `PositiveInt`.
///
/// Returns `true` when `value` satisfies the source check.
pub fn PositiveInt_is_valid(value: PositiveInt) -> bool {
    value > 0
}

/// `Schema.Int` refined with `Schema.isGreaterThanOrEqualTo(0)` (`schema.ts`).
///
/// Wire type is `i64` (never narrowed to `u64`; see `PositiveInt`).
pub type NonNegativeInt = i64;

/// Port of the `Schema.isGreaterThanOrEqualTo(0)` refinement on
/// `NonNegativeInt`.
///
/// Returns `true` when `value` satisfies the source check.
pub fn NonNegativeInt_is_valid(value: NonNegativeInt) -> bool {
    value >= 0
}

/// `Schema.String` branded `"RelativePath"` (`schema.ts`).
///
/// No refinement check in source; any string is accepted. Wire format is a
/// bare string.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct RelativePath(pub String);

impl RelativePath {
    pub fn new(value: String) -> Self {
        RelativePath(value)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Deref for RelativePath {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for RelativePath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for RelativePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for RelativePath {
    fn from(value: String) -> Self {
        RelativePath(value)
    }
}

impl From<&str> for RelativePath {
    fn from(value: &str) -> Self {
        RelativePath(value.to_string())
    }
}

/// `Schema.String` branded `"AbsolutePath"` (`schema.ts`).
///
/// No refinement check in source; any string is accepted. Wire format is a
/// bare string.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct AbsolutePath(pub String);

impl AbsolutePath {
    pub fn new(value: String) -> Self {
        AbsolutePath(value)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Deref for AbsolutePath {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for AbsolutePath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AbsolutePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for AbsolutePath {
    fn from(value: String) -> Self {
        AbsolutePath(value)
    }
}

impl From<&str> for AbsolutePath {
    fn from(value: &str) -> Self {
        AbsolutePath(value.to_string())
    }
}

/// Port of the package `optional(...)` helper (`schema.ts`).
///
/// Source wraps the field schema (`Schema.optionalKey` + decode/encode so
/// encoded objects omit `undefined` keys). Rust call sites model such fields
/// as `Option<T>` with `#[serde(skip_serializing_if = "Option::is_none")]`;
/// this identity helper preserves the combinator name at value level.
pub fn optional<T>(value: Option<T>) -> Option<T> {
    value
}

/// Port of the `statics(...)` combinator (`schema.ts`).
///
/// Source is `(methods) => (schema) => Object.assign(schema, methods(schema))`,
/// attaching statics to the schema object. Rust cannot attach methods to a
/// value, so the combinator returns the `(schema, methods)` pair instead;
/// each per-type static (`create()`, `global`, `ascending()`, `descending()`,
/// …) lives as an inherent associated function on the branded newtype,
/// exactly where the corresponding source `statics(...)` call defines it.
pub fn statics<S, M>(schema: S, methods: impl FnOnce(&S) -> M) -> (S, M) {
    let attached = methods(&schema);
    (schema, attached)
}

/// `Schema.Finite` decoded to `Schema.DateTimeUtc` via
/// `DateTime.makeUnsafe(value)` and encoded back via
/// `DateTime.toEpochMillis(value)` (`schema.ts`).
///
/// Wire format is a finite JSON number of epoch millis. The Rust alias stays
/// `i64`; fractional inputs truncate toward zero (matching JS `TimeClip`
/// semantics for in-range values). No `chrono` dependency is added.
pub type DateTimeUtcFromMillis = i64;

/// Decode side of `DateTimeUtcFromMillis`: finite millis → epoch millis.
pub fn DateTimeUtcFromMillis_decode(value: f64) -> DateTimeUtcFromMillis {
    value as i64
}

/// Encode side of `DateTimeUtcFromMillis`: epoch millis → finite millis.
pub fn DateTimeUtcFromMillis_encode(value: DateTimeUtcFromMillis) -> f64 {
    value as f64
}

/// Serde helper preserving the `DateTimeUtcFromMillis` wire shape (a bare
/// JSON number of millis) for struct fields.
///
/// Attach with `#[serde(with = "crate::schema_primitives::DateTimeUtcFromMillis_serde")]`.
pub mod DateTimeUtcFromMillis_serde {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(*value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        // `Schema.Finite` accepts any finite number; millis arrive integral.
        let value = f64::deserialize(deserializer)?;
        if !value.is_finite() {
            return Err(serde::de::Error::custom(
                "DateTimeUtcFromMillis must be finite",
            ));
        }
        Ok(value as i64)
    }
}
