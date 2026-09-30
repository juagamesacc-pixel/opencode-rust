// source: src/agent/subagent-permissions.ts — exports: deriveSubagentSessionPermission
// (doc comment preserved in source; rule-combination logic verbatim).

use crate::permission::Rule;

/// source: deriveSubagentSessionPermission — parent external_directory + deny
/// rules, plus todowrite/task denies unless subagent permits. Verbatim.
pub fn derive_subagent_session_permission(parent: &[Rule], subagent: &[Rule]) -> Vec<Rule> {
    let mut out: Vec<Rule> = parent
        .iter()
        .filter(|r| r.permission == "external_directory" || r.action == "deny")
        .cloned()
        .collect();
    let can_todo = subagent.iter().any(|r| r.permission == "todowrite");
    let can_task = subagent.iter().any(|r| r.permission == "task");
    if !can_todo {
        out.push(Rule {
            permission: "todowrite".to_string(),
            pattern: "*".to_string(),
            action: "deny".to_string(),
        });
    }
    if !can_task {
        out.push(Rule {
            permission: "task".to_string(),
            pattern: "*".to_string(),
            action: "deny".to_string(),
        });
    }
    out
}
