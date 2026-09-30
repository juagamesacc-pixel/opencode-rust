// source: packages/enterprise — parity tests for Share/Storage/routes logic
use enterprise::core::share::{
    create, data, get, key, legacy, merge, remove, remove_admin, sync, sync_old, Data, Info,
    MessagePayload, PartPayload, SessionPayload,
};
use enterprise::core::storage::{extract_keys, list_prefix, resolve, MemoryStore, Store};

#[test]
fn resolve_joins_with_json_suffix() {
    assert_eq!(
        resolve(&["share_snapshot", "abc"]),
        "share_snapshot/abc.json"
    );
}

#[test]
fn list_prefix_semantics() {
    assert_eq!(list_prefix(&[]), "");
    assert_eq!(list_prefix(&["share_event", "abc"]), "share_event/abc/");
}

#[test]
fn extract_keys_parses_xml() {
    let xml = r#"<ListBucketResult><Contents><Key>a.json</Key></Contents><Contents><Key>b/c.json</Key></Contents></ListBucketResult>"#;
    assert_eq!(extract_keys(xml), vec!["a.json", "b/c.json"]);
}

#[test]
fn key_derivation_verbatim() {
    let session = Data::Session {
        data: SessionPayload {
            id: "s1".into(),
            extra: serde_json::Value::Null,
        },
    };
    assert_eq!(key(&session), "session");
    let message = Data::Message {
        data: MessagePayload {
            id: "m1".into(),
            session_id: "s1".into(),
            extra: serde_json::Value::Null,
        },
    };
    assert_eq!(key(&message), "message/m1");
    let part = Data::Part {
        data: PartPayload {
            id: "p1".into(),
            message_id: "m1".into(),
            extra: serde_json::Value::Null,
        },
    };
    assert_eq!(key(&part), "part/m1/p1");
    assert_eq!(key(&Data::SessionDiff { data: vec![] }), "session_diff");
    assert_eq!(key(&Data::Model { data: vec![] }), "model");
}

#[test]
fn merge_sorts_and_last_wins() {
    let a = Data::Message {
        data: MessagePayload {
            id: "m2".into(),
            session_id: "s1".into(),
            extra: serde_json::Value::Null,
        },
    };
    let b = Data::Message {
        data: MessagePayload {
            id: "m1".into(),
            session_id: "s1".into(),
            extra: serde_json::Value::Null,
        },
    };
    let c = Data::Message {
        data: MessagePayload {
            id: "m2".into(),
            session_id: "s1".into(),
            extra: serde_json::from_str("{\"new\":true}").unwrap(),
        },
    };
    let merged = merge(vec![vec![a, b], vec![c]]);
    assert_eq!(merged.len(), 2);
    assert_eq!(key(&merged[0]), "message/m1");
    // m2 has the later value (c overwrote a)
    let Data::Message { data, .. } = &merged[1] else {
        panic!("expected message")
    };
    assert_eq!(data.extra["new"], serde_json::json!(true));
}

#[test]
fn create_get_remove_lifecycle() {
    let mut store = MemoryStore::new();
    let info = create(&mut store, "session_abcdef12", "secret1", false).unwrap();
    assert_eq!(info.id, "abcdef12");
    assert_eq!(info.session_id, "session_abcdef12");
    assert_eq!(get(&store, &info.id).unwrap().unwrap().secret, "secret1");

    // AlreadyExists
    let dup = create(&mut store, "session_abcdef12", "other", false);
    assert_eq!(
        format!("{}", dup.unwrap_err()),
        "Share already exists: abcdef12"
    );

    // NotFound + InvalidSecret
    assert_eq!(
        format!("{}", remove(&mut store, "nope", "x").unwrap_err()),
        "Share not found: nope"
    );
    assert_eq!(
        format!("{}", remove(&mut store, &info.id, "wrong").unwrap_err()),
        "Share secret invalid: abcdef12"
    );

    remove(&mut store, &info.id, "secret1").unwrap();
    assert_eq!(get(&store, "abcdef12").unwrap(), None);
    assert_eq!(
        format!("{}", remove_admin(&mut store, "abcdef12").unwrap_err()),
        "Share not found: abcdef12"
    );
}

#[test]
fn test_prefix_flag() {
    let mut store = MemoryStore::new();
    // `sessionID.startsWith("test_")` arms the flag: id = "test_" + last 8.
    let info = create(&mut store, "test_abcdefgh", "secret1", false).unwrap();
    assert_eq!(info.id, "test_abcdefgh");
    // Without the flag the id is the bare last-8 slice, even when the
    // session id merely contains "test_".
    let plain = create(&mut store, "session_test_abcd", "secret1", false).unwrap();
    assert_eq!(plain.id, "est_abcd");
}

#[test]
fn sync_and_legacy_flow() {
    let mut store = MemoryStore::new();
    let info = create(&mut store, "session_abcdef12", "secret1", false).unwrap();
    let msg = Data::Message {
        data: MessagePayload {
            id: "m1".into(),
            session_id: "s1".into(),
            extra: serde_json::Value::Null,
        },
    };
    sync(&mut store, &info.id, "secret1", vec![msg.clone()]).unwrap();
    let d = data(&mut store, &info.id).unwrap();
    assert_eq!(d.len(), 1);
    // sync again with a new item — merged
    let msg2 = Data::Message {
        data: MessagePayload {
            id: "m2".into(),
            session_id: "s1".into(),
            extra: serde_json::Value::Null,
        },
    };
    sync(&mut store, &info.id, "secret1", vec![msg2]).unwrap();
    assert_eq!(data(&mut store, &info.id).unwrap().len(), 2);
    // legacy on a fresh share (no snapshot) — empty
    let info2 = create(&mut store, "other_aabbccdd", "s2", false).unwrap();
    assert_eq!(data(&mut store, &info2.id).unwrap().len(), 0);
    let _ = legacy(&mut store, &info2.id).unwrap();
}

#[test]
fn sync_old_writes_bucketed_keys() {
    let mut store = MemoryStore::new();
    let info = create(&mut store, "session_abcdef12", "secret1", false).unwrap();
    let msg = Data::Message {
        data: MessagePayload {
            id: "m1".into(),
            session_id: "s1".into(),
            extra: serde_json::Value::Null,
        },
    };
    sync_old(&mut store, &info.id, "secret1", vec![msg]).unwrap();
    let keys = store
        .list(&list_prefix(&["share_data", &info.id]), None, None, None)
        .unwrap();
    assert_eq!(
        keys,
        vec![format!("share_data/{}/message/m1.json", info.id)]
    );
}

#[test]
fn store_list_bounds() {
    let mut store = MemoryStore::new();
    store.write("share_event/abc/01.json", "[]".into()).unwrap();
    store.write("share_event/abc/02.json", "[]".into()).unwrap();
    store.write("share_event/abc/03.json", "[]".into()).unwrap();
    let all = store.list("share_event/abc/", None, None, None).unwrap();
    assert_eq!(all.len(), 3);
    let before = store
        .list("share_event/abc/", None, None, Some("03"))
        .unwrap();
    assert_eq!(
        before,
        vec!["share_event/abc/01.json", "share_event/abc/02.json"]
    );
    let limited = store.list("share_event/abc/", Some(2), None, None).unwrap();
    assert_eq!(limited.len(), 2);
    let after = store
        .list("share_event/abc/", None, Some("02"), None)
        .unwrap();
    assert_eq!(after, vec!["share_event/abc/03.json"]);
    let _: Option<Info> = None;
}
