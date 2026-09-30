//! Port of packages/app/src/utils/session-message.test.ts — 1:1 assertions.
//! Original: packages/app/src/utils/session-message.test.ts
#![allow(clippy::all)]
use serde_json::json;

#[test]
fn compare_messages_and_key() {
    // Mirrors compareMessages/messageKey from packages/app/src/utils/session-message.ts
    let a_key = app::utils::session_message::message_key(10, "a");
    let b_key = app::utils::session_message::message_key(20, "b");
    assert_eq!(a_key, "10a");
    assert_eq!(b_key, "20b");
    assert_eq!(
        app::utils::session_message::compare_messages(10, "a", 20, "b"),
        -1
    );
    assert_eq!(
        app::utils::session_message::compare_messages(20, "b", 10, "a"),
        1
    );
    assert_eq!(
        app::utils::session_message::compare_messages(10, "a", 10, "a"),
        0
    );
}

#[test]
fn normalize_tool_input_file_path() {
    // Mirrors normalizeToolInput edit/write path→filePath normalization
    let input = json!({"path": "/tmp/foo.ts"});
    let out = app::utils::session_message::normalize_tool_input("edit", input);
    assert_eq!(
        out.get("filePath").and_then(|v| v.as_str()),
        Some("/tmp/foo.ts")
    );
    let already = json!({"filePath": "/tmp/bar.ts", "path": "/tmp/foo.ts"});
    let out2 = app::utils::session_message::normalize_tool_input("edit", already.clone());
    assert_eq!(out2, already);
}

#[test]
fn normalize_tool_metadata_filediff() {
    // Mirrors normalizeToolMetadata files→filediff
    let meta = json!({"files": [{"file": "a.ts", "patch": "@@ -1 +1 @@", "additions": 1, "deletions": 0}]});
    let out = app::utils::session_message::normalize_tool_metadata("edit", meta);
    assert_eq!(
        out.get("filediff")
            .and_then(|v| v.get("file"))
            .and_then(|v| v.as_str()),
        Some("a.ts")
    );
    // non-edit stays verbatim
    let meta2 = json!({"files": [{"file": "a.ts"}]});
    let out2 = app::utils::session_message::normalize_tool_metadata("bash", meta2.clone());
    assert_eq!(out2, meta2);
}

#[test]
fn session_message_part_id_format() {
    // Mirrors sessionMessagePartID
    assert_eq!(
        app::utils::session_message::session_message_part_id("m1", "text", 0),
        "m1:text:0"
    );
}
