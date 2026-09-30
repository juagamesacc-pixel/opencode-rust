#![allow(clippy::all)]
// source: test/credential.test.ts — exports/cases: ["stores, updates, lists, and removes credentials"]
// Real DB assertions: credential CRUD against a fresh schema (delete-then-insert replace per
// integration, asc(time_created) ordering, label update, remove).

use core::credential::sql::{create, get, list, remove, update};
use core::database::database::open_in_memory;
use core::database::migration::apply;

// describe: ["Credential"]
#[test]
fn stores_updates_lists_and_removes_credentials() {
    // source: "stores, updates, lists, and removes credentials"
    let mut conn = open_in_memory().unwrap();
    apply(&mut conn).unwrap();

    let integration_id = "openai";
    let created_id = "cred_work";
    create(
        &mut conn,
        created_id,
        integration_id,
        "Work",
        "{\"type\":\"key\",\"key\":\"secret\"}",
    )
    .unwrap();
    let created = get(&conn, created_id).unwrap().unwrap();
    assert_eq!(created.label, "Work");
    assert_eq!(created.integration_id.as_deref(), Some("openai"));
    assert_eq!(list(&conn, integration_id).unwrap(), vec![created]);

    update(&conn, created_id, Some("Personal"), None).unwrap();
    assert_eq!(list(&conn, integration_id).unwrap()[0].label, "Personal");

    // a second create replaces the integration credential entirely.
    let replacement_id = "cred_replacement";
    create(
        &mut conn,
        replacement_id,
        integration_id,
        "Replacement",
        "{\"type\":\"key\",\"key\":\"replacement\"}",
    )
    .unwrap();
    let replacement = get(&conn, replacement_id).unwrap().unwrap();
    assert_eq!(list(&conn, integration_id).unwrap(), vec![replacement]);
    assert!(get(&conn, created_id).unwrap().is_none());

    remove(&conn, replacement_id).unwrap();
    assert_eq!(list(&conn, integration_id).unwrap(), Vec::new());
}
