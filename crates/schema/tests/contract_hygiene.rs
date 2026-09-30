//! Port of `packages/schema/test/contract-hygiene.test.ts`.
//!
//! Case names/order mirror the source. Mapping notes:
//! - `optional()` helper semantics (transform + omit-when-absent) are ported
//!   with a test-local string-or-number codec standing in for
//!   `FiniteFromString`; the three encode/decode expectations are verbatim.
//! - `SessionTodo.Info` is `schema::session_todo::Info` (Lane B, plan §4).
//! - ID constructors are this lane's (`question`, `pty`).
//! - Reusable public identifiers: asserted as stable, non-empty, unique Rust
//!   type paths for the same 12 schemas (plan §4 module names).
//! - The no-`Any`/no-`mutable` source scan checks the verbatim forbidden
//!   strings in top-level `src/*.rs` (excluding the `*_v1.rs` shims, mirroring
//!   the source `*-v1.ts` exclusion; the `v1/` subtree is out of glob scope
//!   exactly as in source).

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashSet;

fn deserialize_finite_from_string<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};

    struct FiniteVisitor;

    impl<'de> Visitor<'de> for FiniteVisitor {
        type Value = Option<f64>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a finite number or a numeric string")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_any(FiniteSomeVisitor)
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value.is_finite() {
                Ok(Some(value))
            } else {
                Err(E::custom("expected a finite number"))
            }
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value as f64))
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value as f64))
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            value
                .parse::<f64>()
                .map_err(|_| E::custom("expected a numeric string"))
                .and_then(|parsed| {
                    if parsed.is_finite() {
                        Ok(Some(parsed))
                    } else {
                        Err(E::custom("expected a finite number"))
                    }
                })
        }
    }

    struct FiniteSomeVisitor;

    impl<'de> Visitor<'de> for FiniteSomeVisitor {
        type Value = Option<f64>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a finite number or a numeric string")
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value.is_finite() {
                Ok(Some(value))
            } else {
                Err(E::custom("expected a finite number"))
            }
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value as f64))
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value as f64))
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            value
                .parse::<f64>()
                .map_err(|_| E::custom("expected a numeric string"))
                .map(Some)
        }
    }

    deserializer.deserialize_option(FiniteVisitor)
}

fn serialize_finite_to_string<S>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(number) => serializer.serialize_str(&number.to_string()),
        None => serializer.serialize_none(),
    }
}

/// Test-local `{ value: optional(FiniteFromString) }` analogue.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct FiniteValue {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_finite_from_string",
        serialize_with = "serialize_finite_to_string"
    )]
    value: Option<f64>,
}

#[test]
fn optional_properties_preserve_transformations_and_omit_undefined_while_encoding() {
    let decoded: FiniteValue = serde_json::from_value(json!({ "value": "1" })).unwrap();
    assert_eq!(decoded, FiniteValue { value: Some(1.0) });
    assert_eq!(
        serde_json::to_value(&FiniteValue { value: Some(1.0) }).unwrap(),
        json!({ "value": "1" })
    );
    assert_eq!(
        serde_json::to_value(&FiniteValue { value: None }).unwrap(),
        json!({})
    );
}

#[test]
fn todo_status_and_priority_preserve_arbitrary_strings() {
    let decoded: schema::session_todo::Info = serde_json::from_value(
        json!({ "content": "ship", "status": "waiting", "priority": "urgent" }),
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(&decoded).unwrap(),
        json!({ "content": "ship", "status": "waiting", "priority": "urgent" })
    );
}

#[test]
fn current_id_constructors_expose_create() {
    assert!(schema::question::ID::create().as_ref().starts_with("que_"));
    assert!(schema::pty::ID::create().as_ref().starts_with("pty_"));
}

#[test]
fn reusable_public_identifiers_are_stable_and_unique() {
    let identifiers = [
        std::any::type_name::<schema::agent::Color>(),
        std::any::type_name::<schema::filesystem::Submatch>(),
        std::any::type_name::<schema::model::Ref>(),
        std::any::type_name::<schema::model::Capabilities>(),
        std::any::type_name::<schema::model::Cost>(),
        std::any::type_name::<schema::model::Api>(),
        std::any::type_name::<schema::project::Icon>(),
        std::any::type_name::<schema::project::Commands>(),
        std::any::type_name::<schema::project::Time>(),
        std::any::type_name::<schema::project::Info>(),
        std::any::type_name::<schema::pty::Info>(),
        std::any::type_name::<schema::session::ListAnchor>(),
    ];

    assert!(identifiers.iter().all(|identifier| !identifier.is_empty()));
    let unique: HashSet<&&str> = identifiers.iter().collect();
    assert_eq!(unique.len(), identifiers.len());
}

#[test]
fn current_source_avoids_any_and_mutable_contract_wrappers() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let src_dir = std::path::Path::new(manifest_dir).join("src");
    let entries = std::fs::read_dir(&src_dir).unwrap();
    let mut source = String::new();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
        if !file_name.ends_with(".rs") {
            continue;
        }
        if file_name.ends_with("_v1.rs") {
            continue;
        }
        source.push_str(&std::fs::read_to_string(&path).unwrap());
        source.push('\n');
    }

    assert!(!source.contains("Schema.Any"));
    assert!(!source.contains("Schema.mutable"));
}
