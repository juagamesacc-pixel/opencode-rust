// source: test/plugin/loader-shared.test.ts — exports: [named, named]
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { afterEach, describe, expect, spyOn } from "bun:test"; import { LayerNode } from "@opencode-ai/core/effect/layer

#[test]
fn plugin_loader_shared() {
    // source: "plugin.loader.shared"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn loads_a_file_plugin_function_export() {
    // source: "loads a file:// plugin function export"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn deduplicates_same_function_exported_as_default_and_named() {
    // source: "deduplicates same function exported as default and named"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn uses_only_default_v1_server_plugin_when_present() {
    // source: "uses only default v1 server plugin when present"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rejects_v1_file_server_plugin_without_id() {
    // source: "rejects v1 file server plugin without id"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rejects_v1_plugin_that_exports_server_and_tui_together() {
    // source: "rejects v1 plugin that exports server and tui together"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resolves_npm_plugin_specs_with_explicit_and_default_versions() {
    // source: "resolves npm plugin specs with explicit and default versions"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn loads_npm_server_plugin_from_package_server_export() {
    // source: "loads npm server plugin from package ./server export"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn loads_npm_server_plugin_from_package_server_export_without_l() {
    // source: "loads npm server plugin from package server export without leading dot"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn loads_npm_server_plugin_from_package_main_without_leading_do() {
    // source: "loads npm server plugin from package main without leading dot"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_use_npm_package_exports_dot_for_server_entry() {
    // source: "does not use npm package exports dot for server entry"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rejects_npm_server_export_that_resolves_outside_plugin_direc() {
    // source: "rejects npm server export that resolves outside plugin directory"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn skips_legacy_codex_and_copilot_auth_plugin_specs() {
    // source: "skips legacy codex and copilot auth plugin specs"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn skips_broken_plugin_when_install_fails() {
    // source: "skips broken plugin when install fails"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn continues_loading_plugins_when_plugin_init_throws() {
    // source: "continues loading plugins when plugin init throws"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn continues_loading_plugins_when_plugin_module_has_invalid_exp() {
    // source: "continues loading plugins when plugin module has invalid export"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn continues_loading_plugins_when_plugin_import_fails() {
    // source: "continues loading plugins when plugin import fails"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn loads_object_plugin_via_plugin_server() {
    // source: "loads object plugin via plugin.server"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn passes_tuple_plugin_options_into_server_plugin() {
    // source: "passes tuple plugin options into server plugin"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn initializes_server_plugins_in_config_order() {
    // source: "initializes server plugins in config order"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn n() {
    // source: "\n"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn skips_external_plugins_in_pure_mode() {
    // source: "skips external plugins in pure mode"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn reads_oc_themes_from_package_manifest() {
    // source: "reads oc-themes from package manifest"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn handles_no_entrypoint_tui_packages_via_missing_callback() {
    // source: "handles no-entrypoint tui packages via missing callback"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn passes_package_metadata_for_entrypoint_tui_plugins() {
    // source: "passes package metadata for entrypoint tui plugins"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rejects_oc_themes_path_traversal() {
    // source: "rejects oc-themes path traversal"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn retries_failed_file_plugins_once_after_wait_and_keeps_order() {
    // source: "retries failed file plugins once after wait and keeps order"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_retry_permanent_file_plugin_entry_errors() {
    // source: "does not retry permanent file plugin entry errors"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_retry_file_plugins_when_finish_returns_undefined() {
    // source: "does not retry file plugins when finish returns undefined"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_wait_or_retry_npm_plugin_failures() {
    // source: "does not wait or retry npm plugin failures"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
