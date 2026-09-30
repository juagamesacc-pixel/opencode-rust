//! Port of `packages/schema/src/event.ts`.
//!
//! Event core: branded event IDs, the `define` / `inventory` / `latest` /
//! `durable` / `versionedType` combinators, and the `Definition` / `Payload`
//! shapes. 1:1 exact translation — same names/signatures/behavior/edge-cases/
//! error-strings/keys/defaults/ordering. Source is spec.
//!
//! Notes:
//! - This module itself is the `Event` namespace: source line 1
//!   (`export * as Event from "./event"`) is the self-namespace pattern, so
//!   `crate::event` plays that role in Rust; no extra item is emitted for it.
//! - `Data<D extends Definition>` (`= Schema.Schema.Type<D["data"]>`) has no
//!   standalone Rust item: each domain module's payload `Data` struct (e.g.
//!   `crate::workspace_event::Ready::Data`) IS the decoded data form.
//! - `define(...)` returns the attached descriptor (`{ type, durable?, data }`
//!   statics in source). The envelope shape (`id`, `metadata?`, `type`,
//!   `durable?`, `location?`, `data`, annotated with `input.type`) is
//!   `Payload<Data>` below; per-event `TYPE` consts + payload structs +
//!   `definition()` fns live in the domain modules (e.g. `workspace_event`).

#![allow(non_snake_case)]

use std::collections::BTreeMap;
use std::ops::Deref;

/// `Schema.String` checked with `Schema.isStartsWith("evt_")`, branded
/// `"Event.ID"`, with `statics((schema) => ({ create: ... }))` (`event.ts`).
///
/// Wire format is a bare string.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ID(pub String);

impl ID {
    /// Port of `create` (`() => schema.make("evt_" + ascending())`).
    pub fn create() -> Self {
        ID(format!("evt_{}", crate::identifier::ascending()))
    }

    /// Port of the `Schema.isStartsWith("evt_")` check.
    pub fn is_valid(value: &str) -> bool {
        value.starts_with("evt_")
    }

    pub fn new(value: String) -> Self {
        ID(value)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Deref for ID {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ID {
    fn from(value: String) -> Self {
        ID(value)
    }
}

impl From<&str> for ID {
    fn from(value: &str) -> Self {
        ID(value.to_string())
    }
}

/// Port of the inline `durable` descriptor on `Definition`
/// (`{ readonly version: number; readonly aggregate: string }` in `event.ts`).
///
/// `version` is `i64` so integer versions keep their exact JSON shape.
/// Const-constructible so domain modules can publish `Definitions` as a const
/// (port of the `inventory(...)` freeze).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Durable {
    pub version: i64,
    pub aggregate: &'static str,
}
pub type DurableMeta = Durable;

/// Port of `Definition<Type, DataSchema>` (`event.ts`):
/// `{ readonly type: Type; readonly durable?; readonly data: DataSchema }`.
///
/// The TS `data` member is a Schema codec object; the Rust descriptor carries
/// the payload type's static name while runtime validation lives in the
/// payload structs' serde impls. All members are const-constructible so domain
/// modules can publish their inventory as a const.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Definition {
    pub r#type: &'static str,
    pub durable: Option<Durable>,
    pub data: &'static str,
}

/// Input to `define(...)` (`event.ts`):
/// `{ readonly type: Type; readonly durable?; readonly schema: Fields }`.
///
/// `schema` (the `Fields` record of codecs) maps to `data`, the static name
/// of the payload struct produced from those fields; field-level JSON shape
/// (keys, literals, optionals) is preserved by that struct's serde impl.
pub struct DefineInput {
    pub r#type: &'static str,
    pub durable: Option<Durable>,
    pub data: &'static str,
}

/// Port of `define(input)` (`event.ts`).
///
/// Source builds `data = Schema.Struct(input.schema)`, wraps it in the
/// `{ id, metadata?, type, durable?, location?, data }` envelope annotated
/// with `input.type`, and attaches the `{ type, durable?, data }` statics.
/// The Rust return is the attached descriptor; the envelope shape is
/// `Payload<Data>`.
pub fn define(input: DefineInput) -> Definition {
    Definition {
        r#type: input.r#type,
        durable: input.durable,
        data: input.data,
    }
}

/// Port of the `durable` member inside `define`'s envelope
/// (`Schema.Struct({ aggregateID: Schema.String, seq: Schema.Int, version: Schema.Int })`
/// wrapped in `optional(...)`). Key order and key names are verbatim;
/// `Schema.Int` members are `i64`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PayloadDurable {
    pub aggregateID: String,
    pub seq: i64,
    pub version: i64,
}

/// Local JSON-shape port of `Location.Ref`
/// (`Schema.Struct({ directory: AbsolutePath, workspaceID: optional(WorkspaceID) })`,
/// identifier `"Location.Ref"`, key order preserved, `workspaceID` omitted
/// when absent via the package `optional(...)` helper).
///
/// Canonical type: `crate::location::Ref` (owned by the assembly lane; this
/// struct preserves the exact wire shape here only and must be unified there).
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LocationRef {
    pub directory: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaceID: Option<String>,
}

/// Port of `Payload<D extends Definition>` (`event.ts`).
///
/// Field order matches source (`id`, `type`, `data`, `durable?`, `location?`,
/// `metadata?`). The three trailing members use the package `optional(...)`
/// helper (`Schema.Record(Schema.String, Schema.Unknown)` for `metadata`),
/// so absent keys are omitted on encode rather than encoded as `null`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Payload<Data> {
    pub id: ID,
    #[serde(rename = "type")]
    pub r#type: String,
    pub data: Data,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable: Option<PayloadDurable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, serde_json::Value>>,
}

/// Port of `inventory(...definitions)` (`event.ts`).
///
/// Source collects the rest params and returns `Object.freeze(definitions)`.
/// Rust returns the `Vec` as given; callers publish it as their `Definitions`
/// inventory and must not mutate it afterwards (freeze semantics).
pub fn inventory(definitions: Vec<Definition>) -> Vec<Definition> {
    definitions
}

/// Port of `latest(definitions)` (`event.ts`).
///
/// Groups by `type`; when several durable definitions share a type the
/// highest `durable.version` wins, otherwise a second definition for the same
/// type throws `` `Duplicate latest event definition for ${definition.type}` ``
/// (Rust: panics with the byte-identical message).
///
/// Reference-identity note: source compares with `!==` (same object is a
/// no-op); Rust compares by value, so two distinct-but-equal descriptors are
/// also a no-op here. Module singletons (the only in-repo call shape) are
/// unaffected.
pub fn latest(definitions: &[Definition]) -> BTreeMap<String, Definition> {
    let mut result: BTreeMap<String, Definition> = BTreeMap::new();
    for definition in definitions {
        let key = definition.r#type.to_string();
        if let std::collections::btree_map::Entry::Vacant(e) = result.entry(key.clone()) {
            e.insert(definition.clone());
            continue;
        }
        let existing = result.get(&key).cloned().expect("latest: key just checked");
        if let (Some(next), Some(prev)) = (definition.durable.as_ref(), existing.durable.as_ref()) {
            if next.version != prev.version {
                if next.version > prev.version {
                    result.insert(key, definition.clone());
                }
                continue;
            }
        }
        if definition.clone() != existing {
            panic!(
                "Duplicate latest event definition for {}",
                definition.r#type
            );
        }
    }
    result
}

/// Port of `versionedType(type, version)` (`event.ts`): `` `${type}.${version}` ``.
pub fn versionedType(r#type: &str, version: i64) -> String {
    format!("{}.{}", r#type, version)
}

/// Port of `durable(definitions)` (`event.ts`).
///
/// Indexes durable definitions by `versionedType(type, version)`; a duplicate
/// key throws `` `Duplicate durable event definition for ${key}` `` (Rust:
/// panics with the byte-identical message). Non-durable definitions are
/// skipped.
pub fn durable(definitions: &[Definition]) -> BTreeMap<String, Definition> {
    let mut result: BTreeMap<String, Definition> = BTreeMap::new();
    for definition in definitions {
        if definition.durable.is_none() {
            continue;
        }
        let version = definition
            .durable
            .as_ref()
            .expect("durable: durable just checked")
            .version;
        let key = versionedType(definition.r#type, version);
        if result.contains_key(&key) {
            panic!("Duplicate durable event definition for {key}");
        }
        result.insert(key, definition.clone());
    }
    result
}
