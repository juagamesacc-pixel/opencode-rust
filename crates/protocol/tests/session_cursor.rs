//! Rust port of `packages/protocol/test/session-cursor.test.ts`
//! (opencode v1.18.30).
//!
//! 1:1 port — same cases in the same order with the same assertions:
//! 1. `SessionsCursor` round trips (cursor make/parse are inverse).
//! 2. `SessionHistoryQuery` decodes numeric paging inputs (`after`/`limit`
//!    arrive as `NumberFromString` strings and decode to numbers).
//!
//! Notes on the translation:
//! - `Effect.runPromise(SessionsCursor.parse(cursor))` becomes
//!   `SessionsCursor::parse(...).expect(...)`; the `Effect` failure channel
//!   (`"Invalid cursor"`) is the `Err` string.
//! - `Schema.decodeUnknownEffect(SessionHistoryQuery)(…)` becomes
//!   `serde_json::from_value::<SessionHistoryQuery>(…)`.
//! - The `anchor` leaf is the schema-owned `Session.ListAnchor`, built here
//!   from its wire JSON (never redefined in `protocol`); the `ses_test` id
//!   satisfies the `SessionID` `ses` prefix. `workspace: undefined` in
//!   source is `None` (the key is omitted from the cursor JSON, matching
//!   `optional()` encoding).

use protocol::groups::session::{
    SessionHistoryQuery, SessionOrder, SessionsCursor, SessionsCursorAllInput, SessionsCursorInput,
};

#[test]
fn sessions_cursor_round_trips_without_node_globals() {
    let anchor: schema::session::ListAnchor = serde_json::from_value(serde_json::json!({
      "id": "ses_test",
      "time": 1,
      "direction": "next",
    }))
    .expect("anchor wire JSON decodes to Session.ListAnchor");
    let input = SessionsCursorInput::All(SessionsCursorAllInput {
        workspace: None,
        search: Some("protocol".to_string()),
        order: Some(SessionOrder::Desc),
        anchor,
    });
    let cursor = SessionsCursor::make(&input);

    assert_eq!(
        SessionsCursor::parse(cursor.as_str()).expect("cursor parses"),
        input
    );
}

#[test]
fn session_history_query_decodes_numeric_paging_inputs() {
    let query: SessionHistoryQuery =
        serde_json::from_value(serde_json::json!({ "after": "3", "limit": "10" }))
            .expect("paging inputs decode");

    assert_eq!(
        query,
        SessionHistoryQuery {
            after: Some(3),
            limit: Some(10),
        }
    );
}
