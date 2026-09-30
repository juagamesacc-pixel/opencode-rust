//! Port of packages/app/src/utils/session.test.ts — 1:1 assertions.
//! Original: packages/app/src/utils/session.test.ts
#![allow(clippy::all)]
use serde_json::json;

#[test]
fn normalize_session_info_adapts_current_shape() {
    // Mirrors "adapts a current session to the app session shape" from packages/app/src/utils/session.test.ts
    let input = json!({
        "id": "session-1",
        "projectID": "project-1",
        "agent": "build",
        "model": { "id": "gpt-5", "providerID": "openai", "variant": "high" },
        "cost": 0,
        "tokens": { "input": 0, "output": 0, "reasoning": 0, "cache": { "read": 0, "write": 0 } },
        "time": { "created": 1, "updated": 1 },
        "title": "New session",
        "location": { "directory": "/repo/worktree", "workspaceID": "workspace-1" },
        "subpath": "worktree",
        "revert": { "messageID": "message-1", "partID": "part-1", "snapshot": "snapshot", "files": [] }
    });
    let result = app::utils::session::normalize_session_info(&input);
    assert_eq!(result.get("id").and_then(|v| v.as_str()), Some("session-1"));
    assert_eq!(
        result.get("slug").and_then(|v| v.as_str()),
        Some("session-1")
    );
    assert_eq!(
        result.get("directory").and_then(|v| v.as_str()),
        Some("/repo/worktree")
    );
    assert_eq!(
        result.get("workspaceID").and_then(|v| v.as_str()),
        Some("workspace-1")
    );
    assert_eq!(
        result.get("path").and_then(|v| v.as_str()),
        Some("worktree")
    );
    assert_eq!(result.get("version").and_then(|v| v.as_str()), Some(""));
    assert_eq!(
        result.get("title").and_then(|v| v.as_str()),
        Some("New session")
    );
    assert_eq!(
        result
            .get("revert")
            .and_then(|v| v.get("messageID"))
            .and_then(|v| v.as_str()),
        Some("message-1")
    );
}

#[test]
fn normalize_session_info_timestamped_titles() {
    // Mirrors "supplies timestamped titles for untitled current sessions"
    let root = json!({
        "id": "session-1",
        "projectID": "project-1",
        "cost": 0,
        "tokens": { "input": 0, "output": 0, "reasoning": 0, "cache": { "read": 0, "write": 0 } },
        "time": { "created": 0, "updated": 0 },
        "location": { "directory": "/repo" }
    });
    let child = json!({
        "id": "session-2",
        "parentID": "session-1",
        "projectID": "project-1",
        "cost": 0,
        "tokens": { "input": 0, "output": 0, "reasoning": 0, "cache": { "read": 0, "write": 0 } },
        "time": { "created": 0, "updated": 0 },
        "location": { "directory": "/repo" }
    });
    let r1 = app::utils::session::normalize_session_info(&root);
    let r2 = app::utils::session::normalize_session_info(&child);
    assert_eq!(
        r1.get("title").and_then(|v| v.as_str()),
        Some("New session - 1970-01-01T00:00:00.000Z")
    );
    assert_eq!(
        r2.get("title").and_then(|v| v.as_str()),
        Some("Child session - 1970-01-01T00:00:00.000Z")
    );
}

#[test]
fn session_list_accumulator_transition_done() {
    // Mirrors listAllSessions pagination termination logic
    let acc = app::utils::session::SessionListAccumulator::new();
    assert!(acc.transition_done(0, true));
    assert!(acc.transition_done(5, false));
    assert!(!acc.transition_done(5, true));
}
