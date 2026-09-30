// source: src/plugin/install.ts — exports: Target, InstallDeps,
// PatchDeps, PatchInput, InstallResult, ManifestResult, PatchItem,
// PatchResult, installPlugin, readPluginManifest, patchPluginConfig
// (+ Mode/Kind literals; PROVISIONAL: npm/jsonc-patch as descriptors).
// Code strings + file names verbatim.

/// source: Mode literals — verbatim.
pub const MODE_NOOP: &str = "noop";
pub const MODE_ADD: &str = "add";
pub const MODE_REPLACE: &str = "replace";

/// source: result codes — verbatim.
pub const CODE_INSTALL_FAILED: &str = "install_failed";
pub const CODE_MANIFEST_READ_FAILED: &str = "manifest_read_failed";
pub const CODE_MANIFEST_NO_TARGETS: &str = "manifest_no_targets";
pub const CODE_INVALID_JSON: &str = "invalid_json";
pub const CODE_PATCH_FAILED: &str = "patch_failed";

/// source: config file names — verbatim ("opencode" | "tui").
pub const CONFIG_NAMES: &[&str] = &["opencode", "tui"];
