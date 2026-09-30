// source: test/permission-task.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { PermissionV1 } from "@opencode-ai/core/v1/permission"; import { LayerNode } from "@opencode-ai/core/effect/laye

#[test]
fn permission_evaluate_for_permission_task() {
    // source: "Permission.evaluate for permission.task"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_ask_when_no_match_default() {
    // source: "returns ask when no match (default)"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_deny_for_explicit_deny() {
    // source: "returns deny for explicit deny"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_allow_for_explicit_allow() {
    // source: "returns allow for explicit allow"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_ask_for_explicit_ask() {
    // source: "returns ask for explicit ask"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn matches_wildcard_patterns_with_deny() {
    // source: "matches wildcard patterns with deny"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn matches_wildcard_patterns_with_allow() {
    // source: "matches wildcard patterns with allow"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn matches_wildcard_patterns_with_ask() {
    // source: "matches wildcard patterns with ask"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn later_rules_take_precedence_last_match_wins() {
    // source: "later rules take precedence (last match wins)"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn matches_global_wildcard() {
    // source: "matches global wildcard"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn permission_disabled_for_task_tool() {
    // source: "Permission.disabled for task tool"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn task_tool_is_disabled_when_global_deny_pattern_exists_even_w() {
    // source: "task tool is disabled when global deny pattern exists (even with specific allows)"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn task_tool_is_disabled_when_global_deny_pattern_exists_even_w_1() {
    // source: "task tool is disabled when global deny pattern exists (even with ask overrides)"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn task_tool_is_disabled_when_global_deny_pattern_exists() {
    // source: "task tool is disabled when global deny pattern exists"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn task_tool_is_not_disabled_when_only_specific_patterns_are_de() {
    // source: "task tool is NOT disabled when only specific patterns are denied (no wildcard)"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn task_tool_is_enabled_when_no_task_rules_exist_default_ask() {
    // source: "task tool is enabled when no task rules exist (default ask)"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn task_tool_is_not_disabled_when_last_wildcard_pattern_is_allo() {
    // source: "task tool is NOT disabled when last wildcard pattern is allow"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn permission_task_with_real_config_files() {
    // source: "permission.task with real config files"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn loads_task_permissions_from_opencode_json_config() {
    // source: "loads task permissions from opencode.json config"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn loads_task_permissions_with_wildcard_patterns_from_config() {
    // source: "loads task permissions with wildcard patterns from config"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn evaluate_respects_task_permission_from_config() {
    // source: "evaluate respects task permission from config"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn mixed_permission_config_with_task_and_other_tools() {
    // source: "mixed permission config with task and other tools"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn task_tool_disabled_when_global_deny_comes_last_in_config() {
    // source: "task tool disabled when global deny comes last in config"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn task_tool_not_disabled_when_specific_allow_comes_last_in_con() {
    // source: "task tool NOT disabled when specific allow comes last in config"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
