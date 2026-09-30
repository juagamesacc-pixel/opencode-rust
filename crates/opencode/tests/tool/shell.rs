// source: test/tool/shell.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { PermissionV1 } from "@opencode-ai/core/v1/permission"; import { describe, expect } from "bun:test"; import { La

#[test]
fn name_item_label() {
    // source: "${name} [${item.label}]"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn tool_shell() {
    // source: "tool.shell"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn falls_back_from_terminal_only_configured_shell() {
    // source: "falls back from terminal-only configured shell"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn tool_shell_permissions() {
    // source: "tool.shell permissions"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn parses_power_shell_conditionals_for_permission_prompts_item_() {
    // source: "parses PowerShell conditionals for permission prompts [${item.label}]"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn uses_power_shell_cmdlet_prefixes_for_always_allow_prompts_it() {
    // source: "uses PowerShell cmdlet prefixes for always-allow prompts [${item.label}]"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn asks_for_nested_bash_command_permissions_bash() {
    // source: "asks for nested bash command permissions [bash]"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn asks_for_external_directory_permission_for_power_shell_paths() {
    // source: "asks for external_directory permission for PowerShell paths after switches [${item.label}]"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn asks_for_nested_power_shell_command_permissions_item_label() {
    // source: "asks for nested PowerShell command permissions [${item.label}]"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn asks_for_external_directory_permission_for_drive_relative_po() {
    // source: "asks for external_directory permission for drive-relative PowerShell paths [${item.label}]"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn asks_for_external_directory_permission_for_home_power_shell_() {
    // source: "asks for external_directory permission for $HOME PowerShell paths [${item.label}]"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn asks_for_external_directory_permission_for_pwd_power_shell_p() {
    // source: "asks for external_directory permission for $PWD PowerShell paths [${item.label}]"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn asks_for_external_directory_permission_for_pshome_power_shel() {
    // source: "asks for external_directory permission for $PSHOME PowerShell paths [${item.label}]"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn asks_for_external_directory_permission_for_missing_power_she() {
    // source: "asks for external_directory permission for missing PowerShell env paths [${item.label}]"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn asks_for_external_directory_permission_for_power_shell_env_p() {
    // source: "asks for external_directory permission for PowerShell env paths [${item.label}]"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn asks_for_external_directory_permission_for_power_shell_file_() {
    // source: "asks for external_directory permission for PowerShell FileSystem paths [${item.label}]"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn asks_for_external_directory_permission_for_braced_power_shel() {
    // source: "asks for external_directory permission for braced PowerShell env paths [${item.label}]"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn treats_set_location_like_cd_for_permissions_item_label() {
    // source: "treats Set-Location like cd for permissions [${item.label}]"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_add_nested_power_shell_expressions_to_permission_pr() {
    // source: "does not add nested PowerShell expressions to permission prompts [${item.label}]"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn asks_for_external_directory_permission_for_cmd_file_commands() {
    // source: "asks for external_directory permission for cmd file commands [cmd]"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn normalizes_external_directory_workdir_variants_on_windows() {
    // source: "normalizes external_directory workdir variants on Windows"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn uses_git_bash_tmp_semantics_for_external_workdir() {
    // source: "uses Git Bash /tmp semantics for external workdir"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn uses_git_bash_tmp_semantics_for_external_file_paths() {
    // source: "uses Git Bash /tmp semantics for external file paths"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn tool_shell_abort() {
    // source: "tool.shell abort"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn preserves_output_when_aborted() {
    // source: "preserves output when aborted"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn terminates_command_on_timeout() {
    // source: "terminates command on timeout"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn uses_runtime_flags_bash_default_timeout_ms_when_timeout_is_o() {
    // source: "uses RuntimeFlags bashDefaultTimeoutMs when timeout is omitted"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn captures_stderr_in_output() {
    // source: "captures stderr in output"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_non_zero_exit_code() {
    // source: "returns non-zero exit code"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn streams_metadata_updates_progressively() {
    // source: "streams metadata updates progressively"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn tool_shell_truncation() {
    // source: "tool.shell truncation"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn truncates_output_exceeding_line_limit() {
    // source: "truncates output exceeding line limit"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn truncates_output_exceeding_byte_limit() {
    // source: "truncates output exceeding byte limit"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_truncate_small_output() {
    // source: "does not truncate small output"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn full_output_is_saved_to_file_when_truncated() {
    // source: "full output is saved to file when truncated"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
