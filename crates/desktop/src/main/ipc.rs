//! Rust port of `src/main/ipc.ts` (opencode v1.18.30).
//!
//! Ported 1:1: the channel → handler table ([`HANDLERS`], including the
//! `handle`/`on` split and which handlers are keyed by the sender id), the
//! `pickerFilters` early return, the three picker option bags with their
//! `nativeT` title defaults, the picker result shaping (cancel → `null`,
//! single-vs-multiple path, `basename` + `stat().size` file entries, the
//! attachment budget check and the authorization token), the `open-path`
//! platform split, the `reveal-path` existence gate, the
//! `read-clipboard-image` shape, the `store-get` string-or-JSON coercion with
//! its `catch → null`, `draft-delete`'s `set(key, null)`, the two
//! `get-window-id` error messages, the `get-window-focused` /
//! `get-window-fullscreen` `?? false` fallback, the `set-zoom-factor`
//! "set then update the titlebar" order, the `updater-subscribe`
//! sender-id registry, the `set-native-translations` sender validation, and
//! the `run-desktop-menu-action` handler wiring.
//!
//! PROVISIONAL(packages/desktop/src/main/ipc.ts): `ipcMain`, `dialog`,
//! `BrowserWindow`, `clipboard`, `shell`, `execFile`, `stat`, `app`'s
//! `will-quit`/`before-quit`/`browser-window-created` events, and the
//! `DesktopNativeBundle` parser (which lives in `@opencode-ai/app/i18n`).
//!
//! Original file: `packages/desktop/src/main/ipc.ts`

use crate::main::native_translations::{native_t, parse_desktop_native_bundle};
use serde_json::Value;

/// `ipcMain.handle` (returns a value to the renderer) vs `ipcMain.on`
/// (fire-and-forget).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlerKind {
    Handle,
    On,
}

/// One row of the handler registration block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandlerBinding {
    pub channel: &'static str,
    pub kind: HandlerKind,
    /// The arguments the handler reads after `_event`.
    pub arity: usize,
    /// The handler reads `event.sender.id` (the updater registry and the
    /// picked-file authorizations are keyed by it).
    pub keyed_by_sender: bool,
}

/// Every channel `registerIpcHandlers` registers, in source order (54).
pub const HANDLERS: [HandlerBinding; 54] = [
    HandlerBinding {
        channel: "kill-sidecar",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "await-initialization",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "consume-initial-deep-links",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "get-default-server-url",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "set-default-server-url",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "is-first-launch-onboarding-pending",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "finish-first-launch-onboarding",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "is-old-layout-eligible",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "get-display-backend",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "set-display-backend",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "check-app-exists",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "resolve-app-path",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "updater-subscribe",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: true,
    },
    HandlerBinding {
        channel: "updater-unsubscribe",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: true,
    },
    HandlerBinding {
        channel: "updater-check",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "updater-install",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "set-background-color",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "export-debug-logs",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "set-force-focus",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "record-fatal-renderer-error",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "set-native-translations",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "store-get",
        kind: HandlerKind::Handle,
        arity: 2,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "store-set",
        kind: HandlerKind::Handle,
        arity: 3,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "store-delete",
        kind: HandlerKind::Handle,
        arity: 2,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "store-clear",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "store-keys",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "store-length",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "draft-get",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "draft-set",
        kind: HandlerKind::Handle,
        arity: 2,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "draft-delete",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "draft-blob-put",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "draft-blob-get",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "open-directory-picker",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "open-file-picker",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: true,
    },
    HandlerBinding {
        channel: "read-picked-file",
        kind: HandlerKind::Handle,
        arity: 2,
        keyed_by_sender: true,
    },
    HandlerBinding {
        channel: "release-picked-files",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: true,
    },
    HandlerBinding {
        channel: "save-file-picker",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "open-external",
        kind: HandlerKind::On,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "open-local-file",
        kind: HandlerKind::On,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "open-path",
        kind: HandlerKind::Handle,
        arity: 2,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "reveal-path",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "read-clipboard-image",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "get-window-id",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "get-window-focused",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "get-window-fullscreen",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "set-window-focus",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "show-window",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "relaunch",
        kind: HandlerKind::On,
        arity: 0,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "get-zoom-factor",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "set-zoom-factor",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "get-pinch-zoom-enabled",
        kind: HandlerKind::Handle,
        arity: 0,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "set-pinch-zoom-enabled",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "set-titlebar",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
    HandlerBinding {
        channel: "run-desktop-menu-action",
        kind: HandlerKind::Handle,
        arity: 1,
        keyed_by_sender: false,
    },
];

/// The preload channels that `ipc.ts` deliberately does *not* register:
///
/// - the five `on*` members listen for main→renderer events, so they have no
///   handler (their channels appear in
///   [`crate::preload::index::EVENT_CHANNELS`]);
/// - `installCli` is served by `background-cli.ts` via its own registration.
pub const NON_IPC_CHANNELS: [&str; 6] = [
    "menu-command",
    "deep-link",
    "window-fullscreen-changed",
    "pinch-zoom-enabled-changed",
    "zoom-factor-changed",
    "install-cli",
];

/// The `throw new Error(...)` messages the source raises. Native dialog and
/// error copy stays in the i18n bundle; these are the two internal validation
/// messages, which the source also hardcodes.
pub const INVALID_NATIVE_TRANSLATION_SENDER: &str = "Invalid native translation sender";
pub const INVALID_NATIVE_TRANSLATION_BUNDLE: &str = "Invalid native translation bundle";
/// `throw new Error("Window not found")`
pub const WINDOW_NOT_FOUND: &str = "Window not found";
/// `throw new Error("Window ID not found")`
pub const WINDOW_ID_NOT_FOUND: &str = "Window ID not found";

/// `const pickerFilters = (ext?: string[]) => …`
pub fn picker_filters(extensions: Option<&[String]>) -> Option<Vec<(String, Vec<String>)>> {
    let extensions = extensions?;
    if extensions.is_empty() {
        return None;
    }
    Some(vec![(
        native_t("desktop.dialog.files", &[]),
        extensions.to_vec(),
    )])
}

/// `dialog.showOpenDialog` for `open-directory-picker`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenDialogOptions {
    pub properties: Vec<String>,
    pub title: String,
    pub default_path: Option<String>,
}

/// `const openDirectoryPickerOptions = (opts) => ({ properties: ["openDirectory", …(opts?.multiple ? ["multiSelections"] : []), "createDirectory"], title: opts?.title ?? nativeT("desktop.dialog.chooseFolder"), defaultPath: opts?.defaultPath })`
pub fn directory_picker_options(
    multiple: bool,
    title: Option<&str>,
    default_path: Option<&str>,
) -> OpenDialogOptions {
    let mut properties = vec!["openDirectory".to_string()];
    if multiple {
        properties.push("multiSelections".to_string());
    }
    properties.push("createDirectory".to_string());
    OpenDialogOptions {
        properties,
        title: title
            .map(str::to_string)
            .unwrap_or_else(|| native_t("desktop.dialog.chooseFolder", &[])),
        default_path: default_path.map(str::to_string),
    }
}

/// `dialog.showOpenDialog` for `open-file-picker`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenFileDialogOptions {
    pub properties: Vec<String>,
    pub title: String,
    pub default_path: Option<String>,
    pub filters: Option<Vec<(String, Vec<String>)>>,
}

pub fn file_picker_options(
    multiple: bool,
    title: Option<&str>,
    default_path: Option<&str>,
    extensions: Option<&[String]>,
) -> OpenFileDialogOptions {
    let mut properties = vec!["openFile".to_string()];
    if multiple {
        properties.push("multiSelections".to_string());
    }
    OpenFileDialogOptions {
        properties,
        title: title
            .map(str::to_string)
            .unwrap_or_else(|| native_t("desktop.dialog.chooseFile", &[])),
        default_path: default_path.map(str::to_string),
        filters: picker_filters(extensions),
    }
}

/// `dialog.showSaveDialog` for `save-file-picker`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveDialogOptions {
    pub title: String,
    pub default_path: Option<String>,
}

pub fn save_picker_options(title: Option<&str>, default_path: Option<&str>) -> SaveDialogOptions {
    SaveDialogOptions {
        title: title
            .map(str::to_string)
            .unwrap_or_else(|| native_t("desktop.dialog.saveFile", &[])),
        default_path: default_path.map(str::to_string),
    }
}

/// `if (result.canceled) return null; return opts?.multiple ? result.filePaths : result.filePaths[0]`
pub fn directory_picker_result(
    canceled: bool,
    file_paths: &[String],
    multiple: bool,
) -> Option<Value> {
    if canceled {
        return None;
    }
    if multiple {
        return Some(Value::from(
            file_paths
                .iter()
                .map(|path| Value::from(path.clone()))
                .collect::<Vec<_>>(),
        ));
    }
    Some(match file_paths.first() {
        Some(path) => Value::from(path.clone()),
        // `result.filePaths[0]` is `undefined` for an empty non-canceled result.
        None => Value::Null,
    })
}

/// `basename(filePath)` — the last path segment.
pub fn basename(path: &str) -> String {
    let trimmed = path.trim_end_matches(['/', '\\']);
    if trimmed.is_empty() {
        return path.to_string();
    }
    match trimmed.rfind(['/', '\\']) {
        Some(index) => trimmed[index + 1..].to_string(),
        None => trimmed.to_string(),
    }
}

/// The `{ path, name, size }` file entries `open-file-picker` builds, given
/// the `stat().size` of each path.
pub fn picked_file_entries(paths: &[String], sizes: &[u64]) -> Vec<(String, String, u64)> {
    paths
        .iter()
        .zip(sizes.iter())
        .map(|(path, size)| (path.clone(), basename(path), *size))
        .collect()
}

/// `process.platform === "darwin" ? ["open", ["-a", app, path]] : [app, [path]]`
pub fn open_path_command(is_darwin: bool, app: &str, path: &str) -> (String, Vec<String>) {
    if is_darwin {
        (
            "open".to_string(),
            vec!["-a".to_string(), app.to_string(), path.to_string()],
        )
    } else {
        (app.to_string(), vec![path.to_string()])
    }
}

/// `reveal-path`: `if (!exists) return false; shell.showItemInFolder(path); return true`
pub fn reveal_path_decision(exists: bool) -> bool {
    exists
}

/// `read-clipboard-image`: an empty image is `null`.
pub fn clipboard_image_result(
    is_empty: bool,
    png: Vec<u8>,
    width: u32,
    height: u32,
) -> Option<Value> {
    if is_empty {
        return None;
    }
    Some(serde_json::json!({
        "buffer": png,
        "width": width,
        "height": height,
    }))
}

/// `store-get`'s value coercion: `undefined`/`null` → `null`, a string passes
/// through, anything else is `JSON.stringify`'d.
pub fn store_get_value(value: Option<&Value>) -> Value {
    match value {
        None | Some(Value::Null) => Value::Null,
        Some(Value::String(text)) => Value::from(text.clone()),
        Some(other) => Value::from(other.to_string()),
    }
}

/// The `get-window-id` handler's two throws, given the window lookup result
/// and that window's id.
pub fn window_id_result(window: Option<&str>, id: Option<&str>) -> Result<String, String> {
    if window.is_none() {
        return Err(WINDOW_NOT_FOUND.to_string());
    }
    let Some(id) = id else {
        return Err(WINDOW_ID_NOT_FOUND.to_string());
    };
    Ok(id.to_string())
}

/// `win?.isFocused() ?? false` / `win?.isFullScreen() ?? false`
pub fn window_flag(window: Option<bool>) -> bool {
    window.unwrap_or(false)
}

/// `set-native-translations`' sender validation, before the bundle is parsed.
pub fn validate_native_translation_sender(
    window_exists: bool,
    window_destroyed: bool,
    is_main_window_contents: bool,
    is_main_frame: bool,
) -> Result<(), String> {
    if !window_exists || window_destroyed || !is_main_window_contents || !is_main_frame {
        return Err(INVALID_NATIVE_TRANSLATION_SENDER.to_string());
    }
    Ok(())
}

/// The full `set-native-translations` handler: sender validation, then
/// `parseDesktopNativeBundle`.
pub fn handle_set_native_translations(
    sender: (bool, bool, bool, bool),
    value: &Value,
) -> Result<crate::preload::types::DesktopNativeBundle, String> {
    validate_native_translation_sender(sender.0, sender.1, sender.2, sender.3)?;
    parse_desktop_native_bundle(value).ok_or_else(|| INVALID_NATIVE_TRANSLATION_BUNDLE.to_string())
}

/// `export function sendMenuCommand(win, id)`
pub fn menu_command_event(id: &str) -> (&'static str, Value) {
    ("menu-command", Value::from(id.to_string()))
}

/// `export function sendDeepLinks(win, urls)`
pub fn deep_link_event(urls: &[String]) -> (&'static str, Value) {
    (
        "deep-link",
        Value::from(
            urls.iter()
                .map(|url| Value::from(url.clone()))
                .collect::<Vec<_>>(),
        ),
    )
}

/// `export function registerIpcHandlers(deps)`
///
/// PROVISIONAL(packages/desktop/src/main/ipc.ts): the `ipcMain` registration
/// loop and the `app` lifecycle hooks. The registrations themselves are the
/// [`HANDLERS`] table; this function exists to document the order (drafts
/// store, updater subscriptions, then the handlers) and to own the
/// `will-quit` / `before-quit` / `browser-window-created` cleanup contract.
pub fn register_ipc_handlers(deps: IpcDeps) -> IpcRegistration {
    IpcRegistration {
        drafts: crate::main::draft_store::create_desktop_draft_store(&deps.drafts_path),
        updater_subscriptions: crate::main::updater_subscriptions::UpdaterSubscriptions::new(),
        picked_files: crate::main::attachment_picker::PickedFileAuthorizations::new(
            crate::main::attachment_picker::MAX_ATTACHMENT_BYTES,
        ),
    }
}

/// The dependency bundle `registerIpcHandlers(deps)` takes.
pub struct IpcDeps {
    /// `join(app.getPath("userData"), "drafts.sqlite")`
    pub drafts_path: String,
}

/// The state `registerIpcHandlers` closes over.
pub struct IpcRegistration {
    /// `const drafts = createDesktopDraftStore(...)`
    pub drafts: crate::main::draft_store::DesktopDraftStore,
    /// `const updaterSubscriptions = createUpdaterSubscriptions()`
    pub updater_subscriptions: crate::main::updater_subscriptions::UpdaterSubscriptions,
    /// `const pickedFiles = createPickedFileAuthorizations()`
    pub picked_files: crate::main::attachment_picker::PickedFileAuthorizations,
}

#[cfg(test)]
mod tests {
    // The source ships no `ipc.test.ts`; these cover the handler table's
    // agreement with the preload bridge and the portable handler logic.
    use super::*;
    use crate::preload::index::IpcKind;
    use crate::preload::index::{wsl_channel_for, METHOD_BINDINGS, WSL_SERVERS_BINDINGS};
    use crate::preload::types::WslServersApiMethod;

    #[test]
    fn every_preload_top_level_channel_has_a_main_handler() {
        for binding in METHOD_BINDINGS {
            if binding.channel.is_empty() {
                // `getPathForFile` is served inside the preload.
                continue;
            }
            if NON_IPC_CHANNELS.contains(&binding.channel) {
                continue;
            }
            let registered = HANDLERS
                .iter()
                .any(|handler| handler.channel == binding.channel);
            assert!(
                registered,
                "preload channel {} has no ipc handler",
                binding.channel
            );
        }
    }

    #[test]
    fn the_bridges_arity_agrees_with_the_handlers() {
        for binding in METHOD_BINDINGS {
            if binding.kind != IpcKind::Invoke && binding.kind != IpcKind::Send {
                continue;
            }
            let Some(handler) = HANDLERS
                .iter()
                .find(|handler| handler.channel == binding.channel)
            else {
                continue;
            };
            assert_eq!(
                handler.arity, binding.arity,
                "{} arity differs between the preload and main",
                binding.channel
            );
        }
    }

    #[test]
    fn the_handler_table_has_no_duplicate_channels() {
        let mut channels: Vec<&str> = HANDLERS.iter().map(|handler| handler.channel).collect();
        let total = channels.len();
        channels.sort_unstable();
        channels.dedup();
        assert_eq!(channels.len(), total);
    }

    #[test]
    fn the_wsl_channels_are_registered_by_their_own_module() {
        // `wsl.ts` registers the `wsl-servers-*` channels, so they must NOT
        // appear in `ipc.ts`'s table.
        for binding in WSL_SERVERS_BINDINGS {
            assert!(
                !HANDLERS
                    .iter()
                    .any(|handler| handler.channel == binding.channel),
                "{} belongs to wsl/ipc, not main/ipc",
                binding.channel
            );
        }
        assert_eq!(
            wsl_channel_for(WslServersApiMethod::GetState),
            Some("wsl-servers-get-state")
        );
    }

    #[test]
    fn the_fire_and_forget_channels_use_ipc_main_on() {
        let ons: Vec<&str> = HANDLERS
            .iter()
            .filter(|handler| handler.kind == HandlerKind::On)
            .map(|handler| handler.channel)
            .collect();
        assert_eq!(ons, vec!["open-external", "open-local-file", "relaunch"]);
    }

    #[test]
    fn the_sender_keyed_handlers_are_the_ones_that_own_state() {
        let keyed: Vec<&str> = HANDLERS
            .iter()
            .filter(|handler| handler.keyed_by_sender)
            .map(|handler| handler.channel)
            .collect();
        assert_eq!(
            keyed,
            vec![
                "updater-subscribe",
                "updater-unsubscribe",
                "open-file-picker",
                "read-picked-file",
                "release-picked-files",
            ]
        );
    }

    #[test]
    fn filters_are_dropped_for_an_empty_extension_list() {
        assert_eq!(picker_filters(None), None);
        assert_eq!(picker_filters(Some(&[])), None);
        let filters = picker_filters(Some(&["rs".to_string(), "ts".to_string()])).unwrap();
        assert_eq!(filters.len(), 1);
        assert_eq!(filters[0].1, vec!["rs".to_string(), "ts".to_string()]);
        // The filter name is a translated string, never hardcoded English.
        assert!(!filters[0].0.is_empty());
    }

    #[test]
    fn picker_properties_follow_the_multiple_flag() {
        let single = directory_picker_options(false, None, None);
        assert_eq!(
            single.properties,
            vec!["openDirectory", "createDirectory"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
        let many = directory_picker_options(true, None, None);
        assert_eq!(
            many.properties,
            vec!["openDirectory", "multiSelections", "createDirectory"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
        let files = file_picker_options(true, None, None, None);
        assert_eq!(
            files.properties,
            vec!["openFile", "multiSelections"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn an_explicit_title_overrides_the_translated_default() {
        assert_eq!(
            directory_picker_options(false, Some("Pick"), None).title,
            "Pick"
        );
        assert_eq!(
            file_picker_options(false, None, None, None).title,
            native_t("desktop.dialog.chooseFile", &[])
        );
        assert_eq!(
            save_picker_options(None, None).title,
            native_t("desktop.dialog.saveFile", &[])
        );
        assert_eq!(save_picker_options(Some("Save"), None).title, "Save");
    }

    #[test]
    fn a_cancelled_picker_returns_null_and_multiple_returns_every_path() {
        let paths = vec!["/a".to_string(), "/b".to_string()];
        assert_eq!(directory_picker_result(true, &paths, true), None);
        assert_eq!(
            directory_picker_result(false, &paths, true),
            Some(serde_json::json!(["/a", "/b"]))
        );
        assert_eq!(
            directory_picker_result(false, &paths, false),
            Some(Value::from("/a"))
        );
        // An empty non-canceled result gives `undefined` → `null`.
        assert_eq!(
            directory_picker_result(false, &[], false),
            Some(Value::Null)
        );
    }

    #[test]
    fn file_entries_carry_the_basename_and_stat_size() {
        let paths = vec!["/tmp/a.png".to_string(), "C:\\tmp\\b.txt".to_string()];
        assert_eq!(
            picked_file_entries(&paths, &[10, 20]),
            vec![
                ("/tmp/a.png".to_string(), "a.png".to_string(), 10),
                ("C:\\tmp\\b.txt".to_string(), "b.txt".to_string(), 20),
            ]
        );
        assert_eq!(basename("/"), "/");
    }

    #[test]
    fn open_path_splits_per_platform() {
        assert_eq!(
            open_path_command(true, "TextEdit", "/a.txt"),
            (
                "open".to_string(),
                vec![
                    "-a".to_string(),
                    "TextEdit".to_string(),
                    "/a.txt".to_string()
                ]
            )
        );
        assert_eq!(
            open_path_command(false, "TextEdit", "/a.txt"),
            ("TextEdit".to_string(), vec!["/a.txt".to_string()])
        );
    }

    #[test]
    fn reveal_path_only_reveals_an_existing_path() {
        assert!(!reveal_path_decision(false));
        assert!(reveal_path_decision(true));
    }

    #[test]
    fn an_empty_clipboard_image_is_null() {
        assert_eq!(clipboard_image_result(true, Vec::new(), 0, 0), None);
        let image = clipboard_image_result(false, vec![1, 2, 3], 4, 5).unwrap();
        assert_eq!(image["width"], serde_json::json!(4));
        assert_eq!(image["height"], serde_json::json!(5));
        assert_eq!(image["buffer"], serde_json::json!([1, 2, 3]));
    }

    #[test]
    fn store_get_stringifies_non_strings_and_nulls_the_rest() {
        assert_eq!(store_get_value(None), Value::Null);
        assert_eq!(store_get_value(Some(&Value::Null)), Value::Null);
        assert_eq!(store_get_value(Some(&Value::from("hi"))), Value::from("hi"));
        // A number is not a string, so it is `JSON.stringify`'d.
        assert_eq!(
            store_get_value(Some(&serde_json::json!(42))),
            Value::from("42")
        );
        assert_eq!(
            store_get_value(Some(&serde_json::json!({"a":1}))),
            Value::from("{\"a\":1}")
        );
    }

    #[test]
    fn the_window_id_handler_reports_which_lookup_failed() {
        // `BrowserWindow.fromWebContents(sender)` found nothing.
        assert_eq!(
            window_id_result(None, None),
            Err(WINDOW_NOT_FOUND.to_string())
        );
        // The window was found but carries no id.
        assert_eq!(
            window_id_result(Some("w1"), None),
            Err(WINDOW_ID_NOT_FOUND.to_string())
        );
        assert_eq!(
            window_id_result(Some("w1"), Some("42")),
            Ok("42".to_string())
        );
        assert!(!window_flag(None));
        assert!(window_flag(Some(true)));
    }

    #[test]
    fn the_native_translation_handler_validates_the_sender_first() {
        let good = (true, false, true, true);
        // A subframe, another window, or a destroyed window is rejected before
        // the bundle is even parsed.
        assert_eq!(
            handle_set_native_translations((false, false, true, true), &Value::Null),
            Err(INVALID_NATIVE_TRANSLATION_SENDER.to_string())
        );
        assert_eq!(
            handle_set_native_translations((true, true, true, true), &Value::Null),
            Err(INVALID_NATIVE_TRANSLATION_SENDER.to_string())
        );
        assert_eq!(
            handle_set_native_translations((true, false, false, true), &Value::Null),
            Err(INVALID_NATIVE_TRANSLATION_SENDER.to_string())
        );
        assert_eq!(
            handle_set_native_translations((true, false, true, false), &Value::Null),
            Err(INVALID_NATIVE_TRANSLATION_SENDER.to_string())
        );
        // A valid sender with an invalid bundle reports the bundle instead.
        let _ = good;
        assert_eq!(
            handle_set_native_translations(good, &Value::Null).unwrap_err(),
            INVALID_NATIVE_TRANSLATION_BUNDLE
        );
    }

    #[test]
    fn the_outbound_events_carry_their_channel_and_payload() {
        assert_eq!(
            menu_command_event("open-settings"),
            ("menu-command", Value::from("open-settings"))
        );
        let (channel, payload) = deep_link_event(&["opencode://a".to_string()]);
        assert_eq!(channel, "deep-link");
        assert_eq!(payload, serde_json::json!(["opencode://a"]));
    }
}
