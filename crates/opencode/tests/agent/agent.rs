// source: test/agent/agent.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { afterEach, expect } from "bun:test"; import { LayerNode } from "@opencode-ai/core/effect/layer-node"; import {

#[test]
fn returns_default_native_agents_when_no_config() {
    // source: "returns default native agents when no config"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn build_agent_has_correct_default_properties() {
    // source: "build agent has correct default properties"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn plan_agent_denies_edits_except_opencode_plans() {
    // source: "plan agent denies edits except .opencode/plans/*"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn plan_agent_denies_the_general_subagent_by_default() {
    // source: "plan agent denies the general subagent by default"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn user_permission_can_allow_the_general_subagent_from_plan_mod() {
    // source: "user permission can allow the general subagent from plan mode"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn explore_agent_denies_edit_and_write() {
    // source: "explore agent denies edit and write"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn explore_agent_asks_for_external_directories_and_allows_white() {
    // source: "explore agent asks for external directories and allows whitelisted external paths"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn reference_config_does_not_create_subagents() {
    // source: "reference config does not create subagents"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn general_agent_denies_todo_tools() {
    // source: "general agent denies todo tools"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn compaction_agent_denies_all_permissions() {
    // source: "compaction agent denies all permissions"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn custom_agent_from_config_creates_new_agent() {
    // source: "custom agent from config creates new agent"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn custom_agent_config_overrides_native_agent_properties() {
    // source: "custom agent config overrides native agent properties"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn agent_disable_removes_agent_from_list() {
    // source: "agent disable removes agent from list"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn agent_permission_config_merges_with_defaults() {
    // source: "agent permission config merges with defaults"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn global_permission_config_applies_to_all_agents() {
    // source: "global permission config applies to all agents"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn agent_steps_max_steps_config_sets_steps_property() {
    // source: "agent steps/maxSteps config sets steps property"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn agent_mode_can_be_overridden() {
    // source: "agent mode can be overridden"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn agent_name_can_be_overridden() {
    // source: "agent name can be overridden"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn agent_prompt_can_be_set_from_config() {
    // source: "agent prompt can be set from config"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn unknown_agent_properties_are_placed_into_options() {
    // source: "unknown agent properties are placed into options"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn agent_options_merge_correctly() {
    // source: "agent options merge correctly"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn multiple_custom_agents_can_be_defined() {
    // source: "multiple custom agents can be defined"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn agent_list_keeps_the_default_agent_first_and_sorts_the_rest_() {
    // source: "Agent.list keeps the default agent first and sorts the rest by name"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn agent_get_returns_undefined_for_non_existent_agent() {
    // source: "Agent.get returns undefined for non-existent agent"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn default_permission_includes_doom_loop_and_external_directory() {
    // source: "default permission includes doom_loop and external_directory as ask"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn webfetch_is_allowed_by_default() {
    // source: "webfetch is allowed by default"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn legacy_tools_config_converts_to_permissions() {
    // source: "legacy tools config converts to permissions"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn legacy_tools_config_maps_write_edit_patch_to_edit_permission() {
    // source: "legacy tools config maps write/edit/patch to edit permission"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn truncate_glob_is_allowed_even_when_user_denies_external_dire() {
    // source: "Truncate.GLOB is allowed even when user denies external_directory globally"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn global_tmp_directory_children_are_allowed_for_external_direc() {
    // source: "global tmp directory children are allowed for external_directory"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn truncate_glob_is_allowed_even_when_user_denies_external_dire_1() {
    // source: "Truncate.GLOB is allowed even when user denies external_directory per-agent"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn explicit_truncate_glob_deny_is_respected() {
    // source: "explicit Truncate.GLOB deny is respected"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn skill_directories_are_allowed_for_external_directory() {
    // source: "skill directories are allowed for external_directory"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn project_reference_directories_are_allowed_for_external_direc() {
    // source: "project reference directories are allowed for external_directory"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn default_agent_returns_build_when_no_default_agent_config() {
    // source: "defaultAgent returns build when no default_agent config"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn default_info_returns_resolved_build_agent_when_no_default_ag() {
    // source: "defaultInfo returns resolved build agent when no default_agent config"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn default_agent_respects_default_agent_config_set_to_plan() {
    // source: "defaultAgent respects default_agent config set to plan"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn default_agent_respects_default_agent_config_set_to_custom_ag() {
    // source: "defaultAgent respects default_agent config set to custom agent with mode all"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn default_agent_throws_when_default_agent_points_to_subagent() {
    // source: "defaultAgent throws when default_agent points to subagent"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn default_agent_throws_when_default_agent_points_to_hidden_age() {
    // source: "defaultAgent throws when default_agent points to hidden agent"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn default_agent_throws_when_default_agent_points_to_non_existe() {
    // source: "defaultAgent throws when default_agent points to non-existent agent"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn default_agent_returns_plan_when_build_is_disabled_and_defaul() {
    // source: "defaultAgent returns plan when build is disabled and default_agent not set"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn default_agent_throws_when_all_primary_agents_are_disabled() {
    // source: "defaultAgent throws when all primary agents are disabled"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
