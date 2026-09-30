//! Port of `packages/schema/test/event.test.ts` (public event schemas).
//!
//! Case names/order mirror the source. Mapping notes (`toBe` → `assert_eq!`,
//! `size` → `len`, `has` → `contains_key`):
//! - `Event.inventory` / `Event.define` / `Event.latest` / `Event.durable` are
//!   `schema::event` (Lane A, plan §4). Expected Lane A signatures:
//!   `define(type, durable) -> Definition`, `inventory(Vec<Definition>) ->
//!   Vec<Definition>`, `latest(&[Definition]) -> BTreeMap<String, Definition>`
//!   (highest durable version wins, verbatim duplicate panic), `durable(&…)`
//!   keyed by `versioned_type(type, version)`, `versioned_type(&str, i64) ->
//!   String`. The assembly lane reconciles any signature drift; assertions
//!   below are verbatim.

use schema::event::{define, durable, inventory, latest, DefineInput, DurableMeta};
use std::collections::BTreeMap;

#[test]
fn definition_is_pure() {
    let definitions: Vec<schema::event::Definition> = inventory(vec![]);
    define(DefineInput {
        r#type: "test.pure",
        durable: None,
        data: "",
    });
    assert_eq!(definitions, vec![]);
}

#[test]
fn latest_selection_is_independent_of_declaration_order() {
    let historical = define(DefineInput {
        r#type: "test.versioned",
        durable: Some(DurableMeta {
            aggregate: "id",
            version: 1,
        }),
        data: "",
    });
    let current = define(DefineInput {
        r#type: "test.versioned",
        durable: Some(DurableMeta {
            aggregate: "id",
            version: 2,
        }),
        data: "",
    });

    let forward: BTreeMap<String, schema::event::Definition> =
        latest(&[historical.clone(), current.clone()]);
    assert_eq!(forward.get(current.r#type), Some(&current));
    let reversed: BTreeMap<String, schema::event::Definition> =
        latest(&[current.clone(), historical.clone()]);
    assert_eq!(reversed.get(current.r#type), Some(&current));
}

#[test]
fn durable_definitions_are_indexed_by_type_and_version() {
    let definition = define(DefineInput {
        r#type: "test.durable",
        durable: Some(DurableMeta {
            aggregate: "id",
            version: 1,
        }),
        data: "",
    });

    assert_eq!(
        durable(std::slice::from_ref(&definition)).get("test.durable.1"),
        Some(&definition)
    );
}
