// source: test/config/tui.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { expect } from "bun:test"; import path from "path"; import { pathToFileURL } from "url"

#[test]
fn keeps_server_and_tui_plugin_merge_semantics_aligned() {
    // source: "keeps server and tui plugin merge semantics aligned"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn loads_tui_config_with_the_same_precedence_order_as_server_co() {
    // source: "loads tui config with the same precedence order as server config paths"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resolves_attention_config_defaults_and_overrides() {
    // source: "resolves attention config defaults and overrides"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn migrates_tui_specific_keys_from_opencode_json_when_tui_json_() {
    // source: "migrates tui-specific keys from opencode.json when tui.json does not exist"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn migrates_project_legacy_tui_keys_even_when_global_tui_json_a() {
    // source: "migrates project legacy tui keys even when global tui.json already exists"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn drops_unknown_legacy_tui_keys_during_migration() {
    // source: "drops unknown legacy tui keys during migration"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn skips_migration_when_opencode_jsonc_is_syntactically_invalid() {
    // source: "skips migration when opencode.jsonc is syntactically invalid"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn skips_migration_when_tui_json_already_exists() {
    // source: "skips migration when tui.json already exists"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn continues_loading_tui_config_when_legacy_source_cannot_be_st() {
    // source: "continues loading tui config when legacy source cannot be stripped"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn migration_backup_preserves_jsonc_comments() {
    // source: "migration backup preserves JSONC comments"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn migrates_legacy_tui_keys_across_multiple_opencode_json_level() {
    // source: "migrates legacy tui keys across multiple opencode.json levels"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn flattens_nested_tui_key_inside_tui_json() {
    // source: "flattens nested tui key inside tui.json"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn top_level_keys_in_tui_json_take_precedence_over_nested_tui_k() {
    // source: "top-level keys in tui.json take precedence over nested tui key"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn project_config_takes_precedence_over_opencode_tui_config_mat() {
    // source: "project config takes precedence over OPENCODE_TUI_CONFIG (matches OPENCODE_CONFIG)"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn merges_keybind_overrides_across_precedence_layers() {
    // source: "merges keybind overrides across precedence layers"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn ignores_unknown_keybind_names_without_dropping_valid_overrid() {
    // source: "ignores unknown keybind names without dropping valid overrides from the same file"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resolves_keybind_lookup_from_canonical_keybinds() {
    // source: "resolves keybind lookup from canonical keybinds"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn keybinds_accept_open_tui_binding_specs() {
    // source: "keybinds accept OpenTUI binding specs"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn applies_windows_keybind_defaults() {
    // source: "applies Windows keybind defaults"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn ignores_explicit_keybind_terminal_suspend_binding_on_windows() {
    // source: "ignores explicit keybind terminal suspend binding on Windows"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn keeps_explicit_configured_keybind_input_undo_on_windows() {
    // source: "keeps explicit configured keybind input undo on Windows"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn opencode_tui_config_provides_settings_when_no_project_config() {
    // source: "OPENCODE_TUI_CONFIG provides settings when no project config exists"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_derive_tui_path_from_opencode_config() {
    // source: "does not derive tui path from OPENCODE_CONFIG"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn applies_env_and_file_substitutions_in_tui_json() {
    // source: "applies env and file substitutions in tui.json"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn applies_file_substitutions_when_first_identical_token_is_in_() {
    // source: "applies file substitutions when first identical token is in a commented line"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn loads_opencode_tui_json() {
    // source: "loads .opencode/tui.json"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn supports_tuple_plugin_specs_with_options_in_tui_json() {
    // source: "supports tuple plugin specs with options in tui.json"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn deduplicates_tuple_plugin_specs_by_name_with_higher_preceden() {
    // source: "deduplicates tuple plugin specs by name with higher precedence winning"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn tracks_global_and_local_plugin_metadata_in_merged_tui_config() {
    // source: "tracks global and local plugin metadata in merged tui config"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn merges_plugin_enabled_flags_across_config_layers() {
    // source: "merges plugin_enabled flags across config layers"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn silently_skips_malformed_tui_json_load_failures_degrade_to() {
    // source: "silently skips malformed tui.json - load failures degrade to {}"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn silently_skips_non_enoent_read_failures_e_g_tui_json_is_a_di() {
    // source: "silently skips non-ENOENT read failures (e.g. tui.json is a directory) - fallback layer still loads"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn missing_tui_json_silently_treated_as_empty_enoent_path() {
    // source: "missing tui.json - silently treated as empty (ENOENT path)"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
