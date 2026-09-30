//! Rust port of `src/preload/types.ts` (opencode v1.18.30).
//!
//! `DesktopMenuAction`, `UpdaterState`, the `Wsl*` family,
//! `WslServersPlatform`, and `DesktopNativeBundle` are imported from
//! `@opencode-ai/app/{desktop-menu,updater,wsl/types,i18n/desktop-native}`
//! in the source. Those definitions live in the app package, outside this
//! lane's scope, so their shapes are mirrored here as local types — every
//! such item carries a
//! `// PROVISIONAL(packages/app/src/…): canonical definition lives in the
//! app package` tag. Field names are the source's, in `snake_case`.
//!
//! The `ElectronAPI`/`UpdaterAPI`/`WslServersAPI` call surfaces are
//! mirrored as traits with the source's method names and parameters.
//! Promise returns become their resolved value (async delivery is
//! PROVISIONAL — no async runtime binding exists here); `send`-backed
//! fire-and-forget calls return `()`; subscribe-style calls take the
//! callback and return the unsubscribe closure, exactly as in the source.
//!
//! Original file: `packages/desktop/src/preload/types.ts`

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Cross-package mirrors: @opencode-ai/app/desktop-menu
// ---------------------------------------------------------------------------

// PROVISIONAL(packages/app/src/desktop-menu.ts): canonical definition lives
// in the app package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DesktopMenuAction {
    AppCheckForUpdates,
    AppRelaunch,
    WindowNew,
    WindowClose,
    WindowMinimize,
    WindowToggleMaximize,
    ViewReload,
    ViewToggleDevTools,
    ViewResetZoom,
    ViewZoomIn,
    ViewZoomOut,
    ViewToggleFullscreen,
    EditUndo,
    EditRedo,
    EditCut,
    EditCopy,
    EditPaste,
    EditDelete,
    EditSelectAll,
}

impl DesktopMenuAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopMenuAction::AppCheckForUpdates => "app.checkForUpdates",
            DesktopMenuAction::AppRelaunch => "app.relaunch",
            DesktopMenuAction::WindowNew => "window.new",
            DesktopMenuAction::WindowClose => "window.close",
            DesktopMenuAction::WindowMinimize => "window.minimize",
            DesktopMenuAction::WindowToggleMaximize => "window.toggleMaximize",
            DesktopMenuAction::ViewReload => "view.reload",
            DesktopMenuAction::ViewToggleDevTools => "view.toggleDevTools",
            DesktopMenuAction::ViewResetZoom => "view.resetZoom",
            DesktopMenuAction::ViewZoomIn => "view.zoomIn",
            DesktopMenuAction::ViewZoomOut => "view.zoomOut",
            DesktopMenuAction::ViewToggleFullscreen => "view.toggleFullscreen",
            DesktopMenuAction::EditUndo => "edit.undo",
            DesktopMenuAction::EditRedo => "edit.redo",
            DesktopMenuAction::EditCut => "edit.cut",
            DesktopMenuAction::EditCopy => "edit.copy",
            DesktopMenuAction::EditPaste => "edit.paste",
            DesktopMenuAction::EditDelete => "edit.delete",
            DesktopMenuAction::EditSelectAll => "edit.selectAll",
        }
    }

    pub fn from_str(value: &str) -> Option<DesktopMenuAction> {
        match value {
            "app.checkForUpdates" => Some(DesktopMenuAction::AppCheckForUpdates),
            "app.relaunch" => Some(DesktopMenuAction::AppRelaunch),
            "window.new" => Some(DesktopMenuAction::WindowNew),
            "window.close" => Some(DesktopMenuAction::WindowClose),
            "window.minimize" => Some(DesktopMenuAction::WindowMinimize),
            "window.toggleMaximize" => Some(DesktopMenuAction::WindowToggleMaximize),
            "view.reload" => Some(DesktopMenuAction::ViewReload),
            "view.toggleDevTools" => Some(DesktopMenuAction::ViewToggleDevTools),
            "view.resetZoom" => Some(DesktopMenuAction::ViewResetZoom),
            "view.zoomIn" => Some(DesktopMenuAction::ViewZoomIn),
            "view.zoomOut" => Some(DesktopMenuAction::ViewZoomOut),
            "view.toggleFullscreen" => Some(DesktopMenuAction::ViewToggleFullscreen),
            "edit.undo" => Some(DesktopMenuAction::EditUndo),
            "edit.redo" => Some(DesktopMenuAction::EditRedo),
            "edit.cut" => Some(DesktopMenuAction::EditCut),
            "edit.copy" => Some(DesktopMenuAction::EditCopy),
            "edit.paste" => Some(DesktopMenuAction::EditPaste),
            "edit.delete" => Some(DesktopMenuAction::EditDelete),
            "edit.selectAll" => Some(DesktopMenuAction::EditSelectAll),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Cross-package mirrors: @opencode-ai/app/updater
// ---------------------------------------------------------------------------

// PROVISIONAL(packages/app/src/updater.ts): canonical definition lives in
// the app package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpdaterStatus {
    Idle,
    Disabled,
    Checking,
    Downloading,
    Ready,
    UpToDate,
    Installing,
    Error,
}

impl UpdaterStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            UpdaterStatus::Idle => "idle",
            UpdaterStatus::Disabled => "disabled",
            UpdaterStatus::Checking => "checking",
            UpdaterStatus::Downloading => "downloading",
            UpdaterStatus::Ready => "ready",
            UpdaterStatus::UpToDate => "up-to-date",
            UpdaterStatus::Installing => "installing",
            UpdaterStatus::Error => "error",
        }
    }
}

// PROVISIONAL(packages/app/src/updater.ts): canonical definition lives in
// the app package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdaterState {
    pub status: UpdaterStatus,
    pub version: Option<String>,
    pub message: Option<String>,
}

impl UpdaterState {
    pub fn idle() -> Self {
        Self {
            status: UpdaterStatus::Idle,
            version: None,
            message: None,
        }
    }

    pub fn disabled() -> Self {
        Self {
            status: UpdaterStatus::Disabled,
            version: None,
            message: None,
        }
    }

    pub fn checking() -> Self {
        Self {
            status: UpdaterStatus::Checking,
            version: None,
            message: None,
        }
    }

    pub fn downloading(version: String) -> Self {
        Self {
            status: UpdaterStatus::Downloading,
            version: Some(version),
            message: None,
        }
    }

    pub fn ready(version: String) -> Self {
        Self {
            status: UpdaterStatus::Ready,
            version: Some(version),
            message: None,
        }
    }

    pub fn up_to_date() -> Self {
        Self {
            status: UpdaterStatus::UpToDate,
            version: None,
            message: None,
        }
    }

    pub fn installing(version: String) -> Self {
        Self {
            status: UpdaterStatus::Installing,
            version: Some(version),
            message: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            status: UpdaterStatus::Error,
            version: None,
            message: Some(message),
        }
    }
}

// ---------------------------------------------------------------------------
// Cross-package mirrors: @opencode-ai/app/wsl/types
// ---------------------------------------------------------------------------

// PROVISIONAL(packages/app/src/wsl/types.ts): canonical definitions live in
// the app package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WslServerConfig {
    pub id: String,
    pub distro: String,
}

// PROVISIONAL(packages/app/src/wsl/types.ts): canonical definition lives in
// the app package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WslServerRuntime {
    Stopped,
    Starting,
    Ready {
        url: String,
        username: Option<String>,
        password: Option<String>,
    },
    Failed {
        message: String,
    },
}

impl WslServerRuntime {
    pub fn kind(&self) -> &'static str {
        match self {
            WslServerRuntime::Stopped => "stopped",
            WslServerRuntime::Starting => "starting",
            WslServerRuntime::Ready { .. } => "ready",
            WslServerRuntime::Failed { .. } => "failed",
        }
    }
}

// PROVISIONAL(packages/app/src/wsl/types.ts): canonical definition lives in
// the app package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WslServerItem {
    pub config: WslServerConfig,
    pub runtime: WslServerRuntime,
}

// PROVISIONAL(packages/app/src/wsl/types.ts): canonical definition lives in
// the app package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WslRuntimeCheck {
    pub available: bool,
    pub version: Option<String>,
    pub error: Option<String>,
}

// PROVISIONAL(packages/app/src/wsl/types.ts): canonical definition lives in
// the app package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WslInstalledDistro {
    pub name: String,
    pub version: Option<i64>,
    pub is_default: bool,
}

// PROVISIONAL(packages/app/src/wsl/types.ts): canonical definition lives in
// the app package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WslOnlineDistro {
    pub name: String,
    pub label: String,
}

// PROVISIONAL(packages/app/src/wsl/types.ts): canonical definition lives in
// the app package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WslDistroProbe {
    pub name: String,
    pub can_execute: bool,
    pub has_bash: bool,
    pub has_curl: bool,
    pub error: Option<String>,
}

// PROVISIONAL(packages/app/src/wsl/types.ts): canonical definition lives in
// the app package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WslOpencodeCheck {
    pub distro: String,
    pub resolved_path: Option<String>,
    pub version: Option<String>,
    pub expected_version: String,
    pub matches_desktop: Option<bool>,
    pub error: Option<String>,
}

// PROVISIONAL(packages/app/src/wsl/types.ts): canonical definition lives in
// the app package.
#[derive(Debug, Clone, PartialEq)]
pub enum WslJob {
    Runtime {
        started_at: f64,
    },
    Distros {
        started_at: f64,
    },
    InstallWsl {
        started_at: f64,
    },
    InstallDistro {
        distro: String,
        started_at: f64,
    },
    ProbeAddable {
        distros: Vec<String>,
        started_at: f64,
    },
    InstallOpencode {
        distro: String,
        started_at: f64,
    },
}

impl WslJob {
    pub fn kind(&self) -> &'static str {
        match self {
            WslJob::Runtime { .. } => "runtime",
            WslJob::Distros { .. } => "distros",
            WslJob::InstallWsl { .. } => "install-wsl",
            WslJob::InstallDistro { .. } => "install-distro",
            WslJob::ProbeAddable { .. } => "probe-addable",
            WslJob::InstallOpencode { .. } => "install-opencode",
        }
    }
}

// PROVISIONAL(packages/app/src/wsl/types.ts): canonical definition lives in
// the app package.
#[derive(Debug, Clone, PartialEq)]
pub struct WslServersState {
    pub runtime: Option<WslRuntimeCheck>,
    pub installed: Vec<WslInstalledDistro>,
    pub online: Vec<WslOnlineDistro>,
    pub distro_probes: HashMap<String, WslDistroProbe>,
    pub opencode_checks: HashMap<String, WslOpencodeCheck>,
    pub pending_restart: bool,
    pub servers: Vec<WslServerItem>,
    pub job: Option<WslJob>,
}

// PROVISIONAL(packages/app/src/wsl/types.ts): canonical definition lives in
// the app package.
#[derive(Debug, Clone, PartialEq)]
pub struct WslServersEvent {
    #[allow(dead_code)]
    pub event_type: String,
    pub state: WslServersState,
}

impl WslServersEvent {
    pub fn state(state: WslServersState) -> Self {
        Self {
            event_type: "state".to_string(),
            state,
        }
    }
}

// ---------------------------------------------------------------------------
// Cross-package mirrors: @opencode-ai/app/i18n/desktop-native
// ---------------------------------------------------------------------------

// PROVISIONAL(packages/app/src/i18n/desktop-native.ts): canonical
// definition lives in the app package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopNativeBundle {
    pub locale: String,
    pub messages: HashMap<String, String>,
}

// ---------------------------------------------------------------------------
// Desktop-owned types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerReadyData {
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxDisplayBackend {
    Wayland,
    Auto,
}

impl LinuxDisplayBackend {
    pub fn as_str(&self) -> &'static str {
        match self {
            LinuxDisplayBackend::Wayland => "wayland",
            LinuxDisplayBackend::Auto => "auto",
        }
    }

    pub fn from_str(value: &str) -> Option<LinuxDisplayBackend> {
        match value {
            "wayland" => Some(LinuxDisplayBackend::Wayland),
            "auto" => Some(LinuxDisplayBackend::Auto),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TitlebarMode {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TitlebarScheme {
    System,
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TitlebarTheme {
    pub mode: TitlebarMode,
    pub scheme: Option<TitlebarScheme>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FatalRendererError {
    pub error: String,
    pub url: String,
    pub version: Option<String>,
    pub platform: String,
    pub os: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryPickerOptions {
    pub multiple: bool,
    pub title: Option<String>,
    pub default_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectoryPickerResult {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilePickerOptions {
    pub multiple: bool,
    pub title: Option<String>,
    pub default_path: Option<String>,
    pub extensions: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PickedFile {
    pub path: String,
    pub name: String,
    pub size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PickedFiles {
    pub token: String,
    pub files: Vec<PickedFile>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavePickerOptions {
    pub title: Option<String>,
    pub default_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardImage {
    pub buffer: Vec<u8>,
    pub width: u64,
    pub height: u64,
}

// PROVISIONAL(packages/desktop/src/preload/types.ts): the DOM `File`
// object has no in-workspace binding.
pub struct WebFile {
    _private: (),
}

// ---------------------------------------------------------------------------
// API method identity (used by the preload channel tables)
// ---------------------------------------------------------------------------

/// One member of `type ElectronAPI`, in source order. The bridge resolves
/// each to its IPC channel (see `crate::preload::index::METHOD_BINDINGS`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElectronApiMethod {
    KillSidecar,
    InstallCli,
    AwaitInitialization,
    ConsumeInitialDeepLinks,
    GetDefaultServerUrl,
    SetDefaultServerUrl,
    IsFirstLaunchOnboardingPending,
    FinishFirstLaunchOnboarding,
    IsOldLayoutEligible,
    GetDisplayBackend,
    SetDisplayBackend,
    CheckAppExists,
    ResolveAppPath,
    StoreGet,
    StoreSet,
    StoreDelete,
    StoreClear,
    StoreKeys,
    StoreLength,
    DraftGet,
    DraftSet,
    DraftDelete,
    DraftBlobPut,
    DraftBlobGet,
    GetWindowId,
    OnMenuCommand,
    OnDeepLink,
    OpenDirectoryPicker,
    OpenFilePicker,
    ReadPickedFile,
    ReleasePickedFiles,
    GetPathForFile,
    SaveFilePicker,
    OpenExternal,
    OpenLocalFile,
    OpenPath,
    RevealPath,
    ReadClipboardImage,
    GetWindowFocused,
    GetWindowFullscreen,
    OnWindowFullscreenChanged,
    SetWindowFocus,
    ShowWindow,
    Relaunch,
    GetZoomFactor,
    SetZoomFactor,
    GetPinchZoomEnabled,
    SetPinchZoomEnabled,
    OnPinchZoomEnabledChanged,
    OnZoomFactorChanged,
    SetTitlebar,
    RunDesktopMenuAction,
    SetBackgroundColor,
    ExportDebugLogs,
    SetForceFocus,
    RecordFatalRendererError,
    SetNativeTranslations,
}

pub const ELECTRON_API_METHODS: [ElectronApiMethod; 57] = [
    ElectronApiMethod::KillSidecar,
    ElectronApiMethod::InstallCli,
    ElectronApiMethod::AwaitInitialization,
    ElectronApiMethod::ConsumeInitialDeepLinks,
    ElectronApiMethod::GetDefaultServerUrl,
    ElectronApiMethod::SetDefaultServerUrl,
    ElectronApiMethod::IsFirstLaunchOnboardingPending,
    ElectronApiMethod::FinishFirstLaunchOnboarding,
    ElectronApiMethod::IsOldLayoutEligible,
    ElectronApiMethod::GetDisplayBackend,
    ElectronApiMethod::SetDisplayBackend,
    ElectronApiMethod::CheckAppExists,
    ElectronApiMethod::ResolveAppPath,
    ElectronApiMethod::StoreGet,
    ElectronApiMethod::StoreSet,
    ElectronApiMethod::StoreDelete,
    ElectronApiMethod::StoreClear,
    ElectronApiMethod::StoreKeys,
    ElectronApiMethod::StoreLength,
    ElectronApiMethod::DraftGet,
    ElectronApiMethod::DraftSet,
    ElectronApiMethod::DraftDelete,
    ElectronApiMethod::DraftBlobPut,
    ElectronApiMethod::DraftBlobGet,
    ElectronApiMethod::GetWindowId,
    ElectronApiMethod::OnMenuCommand,
    ElectronApiMethod::OnDeepLink,
    ElectronApiMethod::OpenDirectoryPicker,
    ElectronApiMethod::OpenFilePicker,
    ElectronApiMethod::ReadPickedFile,
    ElectronApiMethod::ReleasePickedFiles,
    ElectronApiMethod::GetPathForFile,
    ElectronApiMethod::SaveFilePicker,
    ElectronApiMethod::OpenExternal,
    ElectronApiMethod::OpenLocalFile,
    ElectronApiMethod::OpenPath,
    ElectronApiMethod::RevealPath,
    ElectronApiMethod::ReadClipboardImage,
    ElectronApiMethod::GetWindowFocused,
    ElectronApiMethod::GetWindowFullscreen,
    ElectronApiMethod::OnWindowFullscreenChanged,
    ElectronApiMethod::SetWindowFocus,
    ElectronApiMethod::ShowWindow,
    ElectronApiMethod::Relaunch,
    ElectronApiMethod::GetZoomFactor,
    ElectronApiMethod::SetZoomFactor,
    ElectronApiMethod::GetPinchZoomEnabled,
    ElectronApiMethod::SetPinchZoomEnabled,
    ElectronApiMethod::OnPinchZoomEnabledChanged,
    ElectronApiMethod::OnZoomFactorChanged,
    ElectronApiMethod::SetTitlebar,
    ElectronApiMethod::RunDesktopMenuAction,
    ElectronApiMethod::SetBackgroundColor,
    ElectronApiMethod::ExportDebugLogs,
    ElectronApiMethod::SetForceFocus,
    ElectronApiMethod::RecordFatalRendererError,
    ElectronApiMethod::SetNativeTranslations,
];

/// One member of the nested `wslServers` object (`WslServersPlatform`).
// PROVISIONAL(packages/app/src/wsl/types.ts): canonical definition lives in
// the app package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WslServersApiMethod {
    GetState,
    Subscribe,
    ProbeRuntime,
    RefreshDistros,
    InstallWsl,
    InstallDistro,
    ProbeAddable,
    InstallOpencode,
    OpenTerminal,
    AddServer,
    RemoveServer,
    StartServer,
}

pub const WSL_SERVERS_API_METHODS: [WslServersApiMethod; 12] = [
    WslServersApiMethod::GetState,
    WslServersApiMethod::Subscribe,
    WslServersApiMethod::ProbeRuntime,
    WslServersApiMethod::RefreshDistros,
    WslServersApiMethod::InstallWsl,
    WslServersApiMethod::InstallDistro,
    WslServersApiMethod::ProbeAddable,
    WslServersApiMethod::InstallOpencode,
    WslServersApiMethod::OpenTerminal,
    WslServersApiMethod::AddServer,
    WslServersApiMethod::RemoveServer,
    WslServersApiMethod::StartServer,
];

/// One member of the nested `updater` object (`UpdaterAPI`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpdaterApiMethod {
    Subscribe,
    Check,
    Install,
}

pub const UPDATER_API_METHODS: [UpdaterApiMethod; 3] = [
    UpdaterApiMethod::Subscribe,
    UpdaterApiMethod::Check,
    UpdaterApiMethod::Install,
];

// ---------------------------------------------------------------------------
// API call surfaces
// ---------------------------------------------------------------------------
// The live `ElectronAPI`/`UpdaterAPI`/`WslServersAPI` surface is the
// channel table in `crate::preload::index` (`METHOD_BINDINGS`,
// `UPDATER_BINDINGS`, `WSL_SERVERS_BINDINGS` plus `UpdaterBridge`);
// method identity lives in the enums above. No parallel trait surface is
// kept, so there is exactly one mirror of the source's `const api`.

// PROVISIONAL(packages/desktop/src/preload/index.ts): `ElectronAPI` needs
// `ipcRenderer.invoke`/`send`/`on`/`removeListener`,
// `contextBridge.exposeInMainWorld("api", …)`, and
// `webUtils.getPathForFile`.
// Call-signature mirror of `const api: ElectronAPI`: one provided method
// per source member with identical parameters (Promise returns become
// their resolved value; see the module header). The channel-identity
// mirror is the enum table consumed by `crate::preload::index`.
#[allow(unused_variables)]
pub trait ElectronApi {
    fn kill_sidecar(&self) {
        unimplemented!("ipcRenderer.invoke(\"kill-sidecar\") binding")
    }
    fn install_cli(&self) -> String {
        unimplemented!("ipcRenderer.invoke(\"install-cli\") binding")
    }
    fn await_initialization(&self) -> ServerReadyData {
        unimplemented!("ipcRenderer.invoke(\"await-initialization\") binding")
    }
    fn consume_initial_deep_links(&self) -> Vec<String> {
        unimplemented!("ipcRenderer.invoke(\"consume-initial-deep-links\") binding")
    }
    fn get_default_server_url(&self) -> Option<String> {
        unimplemented!("ipcRenderer.invoke(\"get-default-server-url\") binding")
    }
    fn set_default_server_url(&self, url: Option<&str>) {
        unimplemented!("ipcRenderer.invoke(\"set-default-server-url\") binding")
    }
    fn is_first_launch_onboarding_pending(&self) -> bool {
        unimplemented!("ipcRenderer.invoke(\"is-first-launch-onboarding-pending\") binding")
    }
    fn finish_first_launch_onboarding(&self, create_default_project: bool) -> Option<String> {
        unimplemented!("ipcRenderer.invoke(\"finish-first-launch-onboarding\") binding")
    }
    fn is_old_layout_eligible(&self) -> bool {
        unimplemented!("ipcRenderer.invoke(\"is-old-layout-eligible\") binding")
    }
    fn get_display_backend(&self) -> Option<LinuxDisplayBackend> {
        unimplemented!("ipcRenderer.invoke(\"get-display-backend\") binding")
    }
    fn set_display_backend(&self, backend: Option<LinuxDisplayBackend>) {
        unimplemented!("ipcRenderer.invoke(\"set-display-backend\") binding")
    }
    fn check_app_exists(&self, app_name: &str) -> bool {
        unimplemented!("ipcRenderer.invoke(\"check-app-exists\") binding")
    }
    fn resolve_app_path(&self, app_name: &str) -> Option<String> {
        unimplemented!("ipcRenderer.invoke(\"resolve-app-path\") binding")
    }
    fn store_get(&self, name: &str, key: &str) -> Option<String> {
        unimplemented!("ipcRenderer.invoke(\"store-get\") binding")
    }
    fn store_set(&self, name: &str, key: &str, value: &str) {
        unimplemented!("ipcRenderer.invoke(\"store-set\") binding")
    }
    fn store_delete(&self, name: &str, key: &str) {
        unimplemented!("ipcRenderer.invoke(\"store-delete\") binding")
    }
    fn store_clear(&self, name: &str) {
        unimplemented!("ipcRenderer.invoke(\"store-clear\") binding")
    }
    fn store_keys(&self, name: &str) -> Vec<String> {
        unimplemented!("ipcRenderer.invoke(\"store-keys\") binding")
    }
    fn store_length(&self, name: &str) -> u64 {
        unimplemented!("ipcRenderer.invoke(\"store-length\") binding")
    }
    fn draft_get(&self, key: &str) -> Option<String> {
        unimplemented!("ipcRenderer.invoke(\"draft-get\") binding")
    }
    fn draft_set(&self, key: &str, value: &str) {
        unimplemented!("ipcRenderer.invoke(\"draft-set\") binding")
    }
    fn draft_delete(&self, key: &str) {
        unimplemented!("ipcRenderer.invoke(\"draft-delete\") binding")
    }
    fn draft_blob_put(&self, data: &[u8]) -> String {
        unimplemented!("ipcRenderer.invoke(\"draft-blob-put\") binding")
    }
    fn draft_blob_get(&self, id: &str) -> Option<Vec<u8>> {
        unimplemented!("ipcRenderer.invoke(\"draft-blob-get\") binding")
    }
    fn get_window_id(&self) -> String {
        unimplemented!("ipcRenderer.invoke(\"get-window-id\") binding")
    }
    fn on_menu_command(&self, callback: Box<dyn Fn(String)>) -> Box<dyn FnOnce()> {
        unimplemented!("ipcRenderer.on(\"menu-command\") binding")
    }
    fn on_deep_link(&self, callback: Box<dyn Fn(Vec<String>)>) -> Box<dyn FnOnce()> {
        unimplemented!("ipcRenderer.on(\"deep-link\") binding")
    }
    fn open_directory_picker(
        &self,
        opts: Option<&DirectoryPickerOptions>,
    ) -> Option<DirectoryPickerResult> {
        unimplemented!("ipcRenderer.invoke(\"open-directory-picker\") binding")
    }
    fn open_file_picker(&self, opts: Option<&FilePickerOptions>) -> Option<PickedFiles> {
        unimplemented!("ipcRenderer.invoke(\"open-file-picker\") binding")
    }
    fn read_picked_file(&self, token: &str, path: &str) -> Vec<u8> {
        unimplemented!("ipcRenderer.invoke(\"read-picked-file\") binding")
    }
    fn release_picked_files(&self, token: &str) {
        unimplemented!("ipcRenderer.invoke(\"release-picked-files\") binding")
    }
    fn get_path_for_file(&self, file: &WebFile) -> String {
        unimplemented!("webUtils.getPathForFile binding")
    }
    fn save_file_picker(&self, opts: Option<&SavePickerOptions>) -> Option<String> {
        unimplemented!("ipcRenderer.invoke(\"save-file-picker\") binding")
    }
    fn open_external(&self, url: &str) {
        unimplemented!("ipcRenderer.send(\"open-external\") binding")
    }
    fn open_local_file(&self, url: &str) {
        unimplemented!("ipcRenderer.send(\"open-local-file\") binding")
    }
    fn open_path(&self, path: &str, app: Option<&str>) {
        unimplemented!("ipcRenderer.invoke(\"open-path\") binding")
    }
    fn reveal_path(&self, path: &str) -> bool {
        unimplemented!("ipcRenderer.invoke(\"reveal-path\") binding")
    }
    fn read_clipboard_image(&self) -> Option<ClipboardImage> {
        unimplemented!("ipcRenderer.invoke(\"read-clipboard-image\") binding")
    }
    fn get_window_focused(&self) -> bool {
        unimplemented!("ipcRenderer.invoke(\"get-window-focused\") binding")
    }
    fn get_window_fullscreen(&self) -> bool {
        unimplemented!("ipcRenderer.invoke(\"get-window-fullscreen\") binding")
    }
    fn on_window_fullscreen_changed(&self, callback: Box<dyn Fn(bool)>) -> Box<dyn FnOnce()> {
        unimplemented!("ipcRenderer.on(\"window-fullscreen-changed\") binding")
    }
    fn set_window_focus(&self) {
        unimplemented!("ipcRenderer.invoke(\"set-window-focus\") binding")
    }
    fn show_window(&self) {
        unimplemented!("ipcRenderer.invoke(\"show-window\") binding")
    }
    fn relaunch(&self) {
        unimplemented!("ipcRenderer.send(\"relaunch\") binding")
    }
    fn get_zoom_factor(&self) -> f64 {
        unimplemented!("ipcRenderer.invoke(\"get-zoom-factor\") binding")
    }
    fn set_zoom_factor(&self, factor: f64) {
        unimplemented!("ipcRenderer.invoke(\"set-zoom-factor\") binding")
    }
    fn get_pinch_zoom_enabled(&self) -> bool {
        unimplemented!("ipcRenderer.invoke(\"get-pinch-zoom-enabled\") binding")
    }
    fn set_pinch_zoom_enabled(&self, enabled: bool) {
        unimplemented!("ipcRenderer.invoke(\"set-pinch-zoom-enabled\") binding")
    }
    fn on_pinch_zoom_enabled_changed(&self, callback: Box<dyn Fn(bool)>) -> Box<dyn FnOnce()> {
        unimplemented!("ipcRenderer.on(\"pinch-zoom-enabled-changed\") binding")
    }
    fn on_zoom_factor_changed(&self, callback: Box<dyn Fn(f64)>) -> Box<dyn FnOnce()> {
        unimplemented!("ipcRenderer.on(\"zoom-factor-changed\") binding")
    }
    fn set_titlebar(&self, theme: &TitlebarTheme) {
        unimplemented!("ipcRenderer.invoke(\"set-titlebar\") binding")
    }
    fn run_desktop_menu_action(&self, action: DesktopMenuAction) {
        unimplemented!("ipcRenderer.invoke(\"run-desktop-menu-action\") binding")
    }
    fn set_background_color(&self, color: &str) {
        unimplemented!("ipcRenderer.invoke(\"set-background-color\") binding")
    }
    fn export_debug_logs(&self) -> String {
        unimplemented!("ipcRenderer.invoke(\"export-debug-logs\") binding")
    }
    fn set_force_focus(&self, enabled: bool) {
        unimplemented!("ipcRenderer.invoke(\"set-force-focus\") binding")
    }
    fn record_fatal_renderer_error(&self, error: &FatalRendererError) {
        unimplemented!("ipcRenderer.invoke(\"record-fatal-renderer-error\") binding")
    }
    fn set_native_translations(&self, bundle: &DesktopNativeBundle) {
        unimplemented!("ipcRenderer.invoke(\"set-native-translations\") binding")
    }
}
