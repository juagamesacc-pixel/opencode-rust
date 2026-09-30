//! Port of `packages/schema/src/durable-event-manifest.ts`.
//!
//! Source exports: `SessionDurable` (`{ definitions, schema }`) and `Durable`.
//! `SessionDurable.definitions` covers `SessionEvent.DurableDefinitions` only;
//! top-level `Durable` covers SessionV1 durable defs + SessionEvent durable
//! defs. `schema` is `SessionEvent.Durable` (Lane B union, plan §4).
//!
//! Cross-lane assumptions: `crate::session_event::Event::DurableDefinitions`
//! (see `event_manifest.rs`) and `crate::session_event::Durable` (Lane B).

#![allow(non_snake_case, non_upper_case_globals)]

use crate::event::Durable;
use crate::event_manifest::{Definition, DurableEntry};
use std::collections::BTreeMap;
use std::sync::LazyLock;

fn durable_row(entry: &DurableEntry) -> Definition {
    Definition {
        r#type: entry.r#type,
        durable: Some(Durable {
            aggregate: entry.aggregate,
            version: entry.version,
        }),
        data: "",
    }
}

fn session_v1_durable_rows() -> Vec<Definition> {
    use crate::session_v1::Event as V1;
    [
        (
            V1::Created::TYPE,
            V1::Created::DURABLE_AGGREGATE.unwrap(),
            V1::Created::DURABLE_VERSION.unwrap(),
        ),
        (
            V1::Updated::TYPE,
            V1::Updated::DURABLE_AGGREGATE.unwrap(),
            V1::Updated::DURABLE_VERSION.unwrap(),
        ),
        (
            V1::Deleted::TYPE,
            V1::Deleted::DURABLE_AGGREGATE.unwrap(),
            V1::Deleted::DURABLE_VERSION.unwrap(),
        ),
        (
            V1::MessageUpdated::TYPE,
            V1::MessageUpdated::DURABLE_AGGREGATE.unwrap(),
            V1::MessageUpdated::DURABLE_VERSION.unwrap(),
        ),
        (
            V1::MessageRemoved::TYPE,
            V1::MessageRemoved::DURABLE_AGGREGATE.unwrap(),
            V1::MessageRemoved::DURABLE_VERSION.unwrap(),
        ),
        (
            V1::PartUpdated::TYPE,
            V1::PartUpdated::DURABLE_AGGREGATE.unwrap(),
            V1::PartUpdated::DURABLE_VERSION.unwrap(),
        ),
        (
            V1::PartRemoved::TYPE,
            V1::PartRemoved::DURABLE_AGGREGATE.unwrap(),
            V1::PartRemoved::DURABLE_VERSION.unwrap(),
        ),
    ]
    .into_iter()
    .map(|(r#type, aggregate, version)| Definition {
        r#type,
        durable: Some(Durable { aggregate, version }),
        data: "",
    })
    .collect()
}

fn session_event_durable_rows() -> Vec<Definition> {
    crate::session_event::Event::DurableDefinitions
        .iter()
        .map(durable_row)
        .collect()
}

fn versioned_type(r#type: &str, version: i64) -> String {
    format!("{}.{}", r#type, version)
}

fn durable_map(rows: &[Definition]) -> BTreeMap<String, Definition> {
    let mut result = BTreeMap::new();
    for definition in rows {
        let version = definition
            .durable
            .as_ref()
            .expect("durable definition needs a version")
            .version;
        let key = versioned_type(definition.r#type, version);
        if result.contains_key(&key) {
            panic!("Duplicate durable event definition for {key}");
        }
        result.insert(key, definition.clone());
    }
    result
}

/// `DurableEventManifest.SessionDurable` (`{ definitions, schema }`).
pub mod SessionDurable {
    use std::collections::BTreeMap;
    use std::sync::LazyLock;

    /// `Event.durable(SessionEvent.DurableDefinitions)`.
    pub static definitions: LazyLock<BTreeMap<String, crate::event_manifest::Definition>> =
        LazyLock::new(|| super::durable_map(&super::session_event_durable_rows()));

    /// `SessionEvent.Durable` union schema (Lane B).
    pub use crate::session_event::Durable as schema;
}

/// `DurableEventManifest.Durable` (source expects size 32).
pub static Durable: LazyLock<BTreeMap<String, Definition>> = LazyLock::new(|| {
    let mut rows = session_v1_durable_rows();
    rows.extend(session_event_durable_rows());
    durable_map(&rows)
});
