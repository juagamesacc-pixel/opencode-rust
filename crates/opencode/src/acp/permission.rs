// source: src/acp/permission.ts — exports: Handler, ACPPermission
// PROVISIONAL pending ACP sdk + sdk/v2 + diff npm + @/util/filesystem +
// ./session + ./tool: permissionOptions table, per-session queue rule,
// requestPermission flow (reject without connection, catch→reject,
// non-once/always→reject), edit write-back, title/location/content rules verbatim.

use serde::{Deserialize, Serialize};

/// source: permissionOptions — verbatim optionId/kind/name triples in order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionOption {
    pub option_id: String,
    pub kind: String,
    pub name: String,
}

/// source: permissionOptions table — verbatim.
pub fn permission_options() -> [PermissionOption; 3] {
    [
        PermissionOption {
            option_id: "once".into(),
            kind: "allow_once".into(),
            name: "Allow once".into(),
        },
        PermissionOption {
            option_id: "always".into(),
            kind: "allow_always".into(),
            name: "Always allow".into(),
        },
        PermissionOption {
            option_id: "reject".into(),
            kind: "reject_once".into(),
            name: "Reject".into(),
        },
    ]
}

/// source: Reply = "once" | "always" | "reject" — verbatim.
pub const REPLY_ONCE: &str = "once";
pub const REPLY_ALWAYS: &str = "always";
pub const REPLY_REJECT: &str = "reject";

/// source: selectedReply() — non-selected → reject; once/always passthrough; else reject. Verbatim.
pub fn selected_reply(outcome: &str, option_id: Option<&str>) -> &'static str {
    if outcome != "selected" {
        return REPLY_REJECT;
    }
    match option_id {
        Some("once") => REPLY_ONCE,
        Some("always") => REPLY_ALWAYS,
        _ => REPLY_REJECT,
    }
}

/// source: permissionTitle() tool branches — verbatim key sets.
pub fn title_keys(tool: &str) -> &'static [&'static str] {
    match tool {
        "external_directory" => &["description", "command", "parentDir"],
        "webfetch" => &["url"],
        "websearch" => &["query"],
        "grep" | "glob" => &["pattern"],
        "read" | "edit" | "write" => &["filePath", "filepath", "path"],
        _ => &[],
    }
}

/// source: editTitle() — 1 file → relativePath ?? filePath; N → "{N} files". Verbatim.
pub fn edit_title(count: usize, single: Option<&str>) -> Option<String> {
    if count == 1 {
        return single.map(|s| s.to_string());
    }
    if count > 1 {
        return Some(format!("{} files", count));
    }
    None
}

/// source: diff content type "diff" — verbatim.
pub const DIFF_TYPE: &str = "diff";
