//! Rust port of `src/preload/index.ts` (opencode v1.18.30).
//!
//! The bridge is ported as data: [`METHOD_BINDINGS`] is the full
//! `ElectronAPI` → IPC-channel table, one row per member of
//! `type ElectronAPI` (57) plus the 12 `wslServers` rows and the 3 `updater`
//! rows, carrying the `invoke`/`send` distinction and the argument arity the
//! source passes. The five `on*` members' event channels and the shared
//! updater-listener bookkeeping are ported as live code in
//! [`UpdaterBridge`] and the `EVENT_CHANNELS` table, because their
//! observable behaviour is the refcount logic, not a channel name.
//!
//! PROVISIONAL(packages/desktop/src/preload/index.ts): `contextBridge`,
//! `ipcRenderer`, `webUtils.getPathForFile`, and the `Promise` returned by
//! `ipcRenderer.invoke`. [`IpcTransport`] is the local stand-in: the
//! renderer half of a real Electron build is the preload script itself.
//!
//! Original file: `packages/desktop/src/preload/index.ts`

use crate::preload::types::{
    ElectronApiMethod, UpdaterApiMethod, UpdaterState, WslServersApiMethod,
};
use std::cell::RefCell;
use std::rc::Rc;

/// How a member reaches the main process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcKind {
    /// `ipcRenderer.invoke(channel, ...args)` — awaits a main-process reply.
    Invoke,
    /// `ipcRenderer.send(channel, ...args)` — fire-and-forget.
    Send,
    /// `ipcRenderer.on(channel, handler)` — a main→renderer subscription, so
    /// the channel names an *event* the main process emits and there is no
    /// handler to register.
    On,
    /// No IPC at all: the member is served inside the preload
    /// (`webUtils.getPathForFile`), so it has no channel.
    Local,
}

/// One row of the `ElectronAPI` object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MethodBinding {
    pub method: ElectronApiMethod,
    pub channel: &'static str,
    pub kind: IpcKind,
    /// For [`IpcKind::Invoke`] / [`IpcKind::Send`], the number of arguments the
    /// source forwards; for [`IpcKind::On`], the number of payload arguments
    /// the listener destructures; `0` for [`IpcKind::Local`].
    pub arity: usize,
}

/// The `const api: ElectronAPI` object, in source order.
pub const METHOD_BINDINGS: [MethodBinding; 57] = [
    MethodBinding {
        method: ElectronApiMethod::KillSidecar,
        channel: "kill-sidecar",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBinding {
        method: ElectronApiMethod::InstallCli,
        channel: "install-cli",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBinding {
        method: ElectronApiMethod::AwaitInitialization,
        channel: "await-initialization",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBinding {
        method: ElectronApiMethod::ConsumeInitialDeepLinks,
        channel: "consume-initial-deep-links",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBinding {
        method: ElectronApiMethod::GetDefaultServerUrl,
        channel: "get-default-server-url",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBinding {
        method: ElectronApiMethod::SetDefaultServerUrl,
        channel: "set-default-server-url",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::IsFirstLaunchOnboardingPending,
        channel: "is-first-launch-onboarding-pending",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBinding {
        method: ElectronApiMethod::FinishFirstLaunchOnboarding,
        channel: "finish-first-launch-onboarding",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::IsOldLayoutEligible,
        channel: "is-old-layout-eligible",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBinding {
        method: ElectronApiMethod::GetDisplayBackend,
        channel: "get-display-backend",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBinding {
        method: ElectronApiMethod::SetDisplayBackend,
        channel: "set-display-backend",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::CheckAppExists,
        channel: "check-app-exists",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::ResolveAppPath,
        channel: "resolve-app-path",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::StoreGet,
        channel: "store-get",
        kind: IpcKind::Invoke,
        arity: 2,
    },
    MethodBinding {
        method: ElectronApiMethod::StoreSet,
        channel: "store-set",
        kind: IpcKind::Invoke,
        arity: 3,
    },
    MethodBinding {
        method: ElectronApiMethod::StoreDelete,
        channel: "store-delete",
        kind: IpcKind::Invoke,
        arity: 2,
    },
    MethodBinding {
        method: ElectronApiMethod::StoreClear,
        channel: "store-clear",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::StoreKeys,
        channel: "store-keys",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::StoreLength,
        channel: "store-length",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::DraftGet,
        channel: "draft-get",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::DraftSet,
        channel: "draft-set",
        kind: IpcKind::Invoke,
        arity: 2,
    },
    MethodBinding {
        method: ElectronApiMethod::DraftDelete,
        channel: "draft-delete",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::DraftBlobPut,
        channel: "draft-blob-put",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::DraftBlobGet,
        channel: "draft-blob-get",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::GetWindowId,
        channel: "get-window-id",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBinding {
        method: ElectronApiMethod::OnMenuCommand,
        channel: "menu-command",
        kind: IpcKind::On,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::OnDeepLink,
        channel: "deep-link",
        kind: IpcKind::On,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::OpenDirectoryPicker,
        channel: "open-directory-picker",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::OpenFilePicker,
        channel: "open-file-picker",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::ReadPickedFile,
        channel: "read-picked-file",
        kind: IpcKind::Invoke,
        arity: 2,
    },
    MethodBinding {
        method: ElectronApiMethod::ReleasePickedFiles,
        channel: "release-picked-files",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::GetPathForFile,
        channel: "",
        kind: IpcKind::Local,
        arity: 0,
    },
    MethodBinding {
        method: ElectronApiMethod::SaveFilePicker,
        channel: "save-file-picker",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::OpenExternal,
        channel: "open-external",
        kind: IpcKind::Send,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::OpenLocalFile,
        channel: "open-local-file",
        kind: IpcKind::Send,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::OpenPath,
        channel: "open-path",
        kind: IpcKind::Invoke,
        arity: 2,
    },
    MethodBinding {
        method: ElectronApiMethod::RevealPath,
        channel: "reveal-path",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::ReadClipboardImage,
        channel: "read-clipboard-image",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBinding {
        method: ElectronApiMethod::GetWindowFocused,
        channel: "get-window-focused",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBinding {
        method: ElectronApiMethod::GetWindowFullscreen,
        channel: "get-window-fullscreen",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBinding {
        method: ElectronApiMethod::OnWindowFullscreenChanged,
        channel: "window-fullscreen-changed",
        kind: IpcKind::On,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::SetWindowFocus,
        channel: "set-window-focus",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBinding {
        method: ElectronApiMethod::ShowWindow,
        channel: "show-window",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBinding {
        method: ElectronApiMethod::Relaunch,
        channel: "relaunch",
        kind: IpcKind::Send,
        arity: 0,
    },
    MethodBinding {
        method: ElectronApiMethod::GetZoomFactor,
        channel: "get-zoom-factor",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBinding {
        method: ElectronApiMethod::SetZoomFactor,
        channel: "set-zoom-factor",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::GetPinchZoomEnabled,
        channel: "get-pinch-zoom-enabled",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBinding {
        method: ElectronApiMethod::SetPinchZoomEnabled,
        channel: "set-pinch-zoom-enabled",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::OnPinchZoomEnabledChanged,
        channel: "pinch-zoom-enabled-changed",
        kind: IpcKind::On,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::OnZoomFactorChanged,
        channel: "zoom-factor-changed",
        kind: IpcKind::On,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::SetTitlebar,
        channel: "set-titlebar",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::RunDesktopMenuAction,
        channel: "run-desktop-menu-action",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::SetBackgroundColor,
        channel: "set-background-color",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::ExportDebugLogs,
        channel: "export-debug-logs",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBinding {
        method: ElectronApiMethod::SetForceFocus,
        channel: "set-force-focus",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::RecordFatalRendererError,
        channel: "record-fatal-renderer-error",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBinding {
        method: ElectronApiMethod::SetNativeTranslations,
        channel: "set-native-translations",
        kind: IpcKind::Invoke,
        arity: 1,
    },
];

/// The `wslServers` nested object.
pub const WSL_SERVERS_BINDINGS: [MethodBindingKind; 12] = [
    MethodBindingKind {
        channel: "wsl-servers-get-state",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBindingKind {
        channel: "wsl-servers-subscribe",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBindingKind {
        channel: "wsl-servers-probe-runtime",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBindingKind {
        channel: "wsl-servers-refresh-distros",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBindingKind {
        channel: "wsl-servers-install-wsl",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBindingKind {
        channel: "wsl-servers-install-distro",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBindingKind {
        channel: "wsl-servers-probe-addable",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBindingKind {
        channel: "wsl-servers-install-opencode",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBindingKind {
        channel: "wsl-servers-open-terminal",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBindingKind {
        channel: "wsl-servers-add",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBindingKind {
        channel: "wsl-servers-remove",
        kind: IpcKind::Invoke,
        arity: 1,
    },
    MethodBindingKind {
        channel: "wsl-servers-start",
        kind: IpcKind::Invoke,
        arity: 1,
    },
];

/// The `updater` nested object, minus `subscribe`'s event plumbing.
pub const UPDATER_BINDINGS: [MethodBindingKind; 3] = [
    MethodBindingKind {
        channel: "updater-subscribe",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBindingKind {
        channel: "updater-check",
        kind: IpcKind::Invoke,
        arity: 0,
    },
    MethodBindingKind {
        channel: "updater-install",
        kind: IpcKind::Invoke,
        arity: 0,
    },
];

/// A channel/kind/arity row without the top-level method identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MethodBindingKind {
    pub channel: &'static str,
    pub kind: IpcKind,
    pub arity: usize,
}

/// The `ElectronAPI` method → channel lookup, as the preload performs it.
pub fn channel_for(method: ElectronApiMethod) -> Option<&'static str> {
    METHOD_BINDINGS
        .iter()
        .find(|binding| binding.method == method)
        .map(|binding| binding.channel)
}

/// The `wslServers` method → channel lookup.
pub fn wsl_channel_for(method: WslServersApiMethod) -> Option<&'static str> {
    WSL_SERVERS_BINDINGS
        .get(index_of_wsl(method))
        .map(|binding| binding.channel)
}

/// The `updater` method → channel lookup.
///
/// PROVISIONAL: the `subscribe` method is not in [`UPDATER_BINDINGS`]' index
/// order, so the lookup is spelled out.
pub fn updater_channel_for(method: UpdaterApiMethod) -> Option<&'static str> {
    match method {
        UpdaterApiMethod::Subscribe => Some("updater-subscribe"),
        UpdaterApiMethod::Check => Some("updater-check"),
        UpdaterApiMethod::Install => Some("updater-install"),
    }
}

fn index_of_wsl(method: WslServersApiMethod) -> usize {
    use WslServersApiMethod::*;
    match method {
        GetState => 0,
        Subscribe => 1,
        ProbeRuntime => 2,
        RefreshDistros => 3,
        InstallWsl => 4,
        InstallDistro => 5,
        ProbeAddable => 6,
        InstallOpencode => 7,
        OpenTerminal => 8,
        AddServer => 9,
        RemoveServer => 10,
        StartServer => 11,
    }
}

/// The renderer-side IPC surface the bridge drives.
///
/// PROVISIONAL: stands in for `ipcRenderer`.
pub trait IpcTransport {
    /// `ipcRenderer.invoke(channel, ...args)`
    fn invoke(&mut self, channel: &str, args: &[serde_json::Value]);
    /// `ipcRenderer.send(channel, ...args)`
    fn send(&mut self, channel: &str, args: &[serde_json::Value]);
    /// `ipcRenderer.on(channel, handler)`
    fn on(&mut self, channel: &str, handler: Box<dyn FnMut(&serde_json::Value)>);
    /// `ipcRenderer.removeListener(channel, handler)`
    fn remove_listener(&mut self, channel: &str);
}

/// An updater subscription callback.
pub type UpdaterCallback = Box<dyn FnMut(&UpdaterState)>;

/// `let updaterCallbacks` / `let updaterState` are *module-level* in the
/// source, so the `on("updater-state")` handler and `updater.subscribe` must
/// observe the same storage. They are therefore shared cells rather than
/// plain fields, which is what lets the registered handler call back into the
/// bridge without borrowing it.
pub type CallbackList = Rc<RefCell<Vec<UpdaterCallback>>>;
pub type StateCell = Rc<RefCell<Option<UpdaterState>>>;

/// `const updaterCallbacks` / `let updaterState` / `let updaterSubscription`
/// and `const updaterHandler` — the shared-listener bookkeeping behind
/// `updater.subscribe`.
pub struct UpdaterBridge {
    callbacks: CallbackList,
    state: StateCell,
    subscription: bool,
}

impl Default for UpdaterBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl UpdaterBridge {
    pub fn new() -> Self {
        Self {
            callbacks: Rc::new(RefCell::new(Vec::new())),
            state: Rc::new(RefCell::new(None)),
            subscription: false,
        }
    }

    /// `const updaterHandler = (_, state) => { updaterState = state; updaterCallbacks.forEach(cb => cb(state)) }`
    ///
    /// Takes `&self` because the shared cells are what the registered listener
    /// mutates; a direct call is the same code path the listener runs.
    pub fn handle_state(&self, state: &UpdaterState) {
        *self.state.borrow_mut() = Some(state.clone());
        for callback in self.callbacks.borrow_mut().iter_mut() {
            callback(state);
        }
    }

    /// The `updater-state` payload → [`UpdaterState`] boundary decode, run by
    /// the registered listener before the callbacks are invoked.
    pub fn handle_state_value(&self, payload: &serde_json::Value) {
        if let Some(state) = updater_state_from_value(payload) {
            self.handle_state(&state);
        }
    }

    /// `updater.subscribe(cb)` — the callback is registered, replayed the
    /// cached state if one exists, and the main-process subscription is
    /// opened only for the first subscriber.
    ///
    /// Returns `false` when an `on("updater-state")` listener was newly
    /// registered, which is the source's `if (!updaterSubscription)` arm.
    pub fn subscribe(&mut self, ipc: &mut dyn IpcTransport, callback: UpdaterCallback) -> bool {
        let opened = !self.subscription;
        self.callbacks.borrow_mut().push(callback);
        // `if (updaterState) cb(updaterState)` — replay before the listener is
        // attached, exactly as in the source.
        let cached = self.state.borrow().clone();
        if let Some(state) = cached {
            if let Some(callback) = self.callbacks.borrow_mut().last_mut() {
                callback(&state);
            }
        }
        if opened {
            self.subscription = true;
            let state_cell = Rc::clone(&self.state);
            let callbacks = Rc::clone(&self.callbacks);
            // `ipcRenderer.on("updater-state", updaterHandler)` — the handler
            // runs the module-level bookkeeping for every incoming event.
            ipc.on(
                "updater-state",
                Box::new(move |payload: &serde_json::Value| {
                    let Some(state) = updater_state_from_value(payload) else {
                        return;
                    };
                    *state_cell.borrow_mut() = Some(state.clone());
                    for callback in callbacks.borrow_mut().iter_mut() {
                        callback(&state);
                    }
                }),
            );
            ipc.invoke("updater-subscribe", &[]);
        }
        opened
    }

    /// The disposer `updater.subscribe` returns: drop the callback and, when
    /// it was the last one, tear the main-process subscription down.
    ///
    /// Returns `true` when the teardown arm ran.
    pub fn unsubscribe(&mut self, ipc: &mut dyn IpcTransport, index: usize) -> bool {
        if index >= self.callbacks.borrow().len() {
            return false;
        }
        self.callbacks.borrow_mut().remove(index);
        if !self.callbacks.borrow().is_empty() {
            return false;
        }
        self.subscription = false;
        ipc.remove_listener("updater-state");
        ipc.invoke("updater-unsubscribe", &[]);
        true
    }

    pub fn callback_count(&self) -> usize {
        self.callbacks.borrow().len()
    }

    pub fn is_subscribed(&self) -> bool {
        self.subscription
    }

    pub fn state(&self) -> Option<UpdaterState> {
        self.state.borrow().clone()
    }
}

/// `updater-state` payload → [`UpdaterState`]. The status strings come from
/// [`crate::preload::types::UpdaterStatus::as_str`].
fn updater_state_from_value(payload: &serde_json::Value) -> Option<UpdaterState> {
    use crate::preload::types::UpdaterStatus;
    let status = match payload.get("status")?.as_str()? {
        "idle" => UpdaterStatus::Idle,
        "disabled" => UpdaterStatus::Disabled,
        "checking" => UpdaterStatus::Checking,
        "downloading" => UpdaterStatus::Downloading,
        "ready" => UpdaterStatus::Ready,
        "up-to-date" => UpdaterStatus::UpToDate,
        "installing" => UpdaterStatus::Installing,
        "error" => UpdaterStatus::Error,
        // An unknown status is not a state the bridge knows.
        _ => return None,
    };
    let text = |key: &str| {
        payload
            .get(key)
            .and_then(|value| value.as_str())
            .map(str::to_string)
    };
    Some(UpdaterState {
        status,
        version: text("version"),
        message: text("message"),
    })
}

/// The six `on*`/subscribe members' `(member name, event channel, teardown
/// channel)` triples. The five top-level `on*` members have no teardown
/// channel — their disposer only calls `removeListener` — so the third field
/// is `None` for them.
pub const EVENT_CHANNELS: [(&str, &str, Option<&str>); 6] = [
    ("onMenuCommand", "menu-command", None),
    ("onDeepLink", "deep-link", None),
    (
        "onWindowFullscreenChanged",
        "window-fullscreen-changed",
        None,
    ),
    (
        "onPinchZoomEnabledChanged",
        "pinch-zoom-enabled-changed",
        None,
    ),
    ("onZoomFactorChanged", "zoom-factor-changed", None),
    // `wslServers.subscribe` tears the main-process subscription down too.
    (
        "wslServers.subscribe",
        "wsl-servers-event",
        Some("wsl-servers-unsubscribe"),
    ),
];

#[cfg(test)]
mod tests {
    // The source ships no `preload/index.test.ts`; these cover the channel
    // table's completeness and the updater refcount logic.
    use super::*;
    use crate::preload::types::{
        ELECTRON_API_METHODS, UPDATER_API_METHODS, WSL_SERVERS_API_METHODS,
    };
    use serde_json::Value;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    #[derive(Default)]
    struct Recorder {
        invokes: Vec<String>,
        sends: Vec<String>,
        listeners: Vec<String>,
        removed: Vec<String>,
        // `ipcRenderer` keeps the registered handlers, so the recorder does too
        // and a test can deliver an event to them.
        handlers: HashMap<String, Box<dyn FnMut(&Value)>>,
    }

    impl Recorder {
        /// `webContents.emit(channel, payload)` — hand the payload to the
        /// registered listener, as the real transport does.
        fn deliver(&mut self, channel: &str, payload: Value) -> bool {
            match self.handlers.get_mut(channel) {
                Some(handler) => {
                    handler(&payload);
                    true
                }
                None => false,
            }
        }
    }

    impl IpcTransport for Recorder {
        fn invoke(&mut self, channel: &str, _args: &[Value]) {
            self.invokes.push(channel.to_string());
        }
        fn send(&mut self, channel: &str, _args: &[Value]) {
            self.sends.push(channel.to_string());
        }
        fn on(&mut self, channel: &str, handler: Box<dyn FnMut(&Value)>) {
            self.listeners.push(channel.to_string());
            self.handlers.insert(channel.to_string(), handler);
        }
        fn remove_listener(&mut self, channel: &str) {
            self.removed.push(channel.to_string());
            self.handlers.remove(channel);
        }
    }

    #[test]
    fn every_api_member_has_exactly_one_binding() {
        assert_eq!(METHOD_BINDINGS.len(), ELECTRON_API_METHODS.len());
        for method in ELECTRON_API_METHODS {
            let matches: Vec<&MethodBinding> = METHOD_BINDINGS
                .iter()
                .filter(|binding| binding.method == method)
                .collect();
            assert_eq!(matches.len(), 1, "method {method:?} needs one binding");
        }
    }

    #[test]
    fn channels_are_unique_and_kebab_cased() {
        let mut channels: Vec<&str> = METHOD_BINDINGS.iter().map(|b| b.channel).collect();
        // `getPathForFile` is the one member that never touches IPC.
        channels.retain(|channel| !channel.is_empty());
        let total = channels.len();
        channels.sort_unstable();
        channels.dedup();
        assert_eq!(channels.len(), total, "duplicate channel name");
        for channel in &channels {
            assert!(
                channel
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
                "channel {channel} is not kebab-case"
            );
        }
    }

    #[test]
    fn the_fire_and_forget_members_send_rather_than_invoke() {
        let senders: Vec<&str> = METHOD_BINDINGS
            .iter()
            .filter(|binding| binding.kind == IpcKind::Send)
            .map(|binding| binding.channel)
            .collect();
        assert_eq!(
            senders,
            vec!["open-external", "open-local-file", "relaunch"]
        );
    }

    #[test]
    fn the_argument_arity_matches_the_source_calls() {
        let arity = |method: ElectronApiMethod| {
            METHOD_BINDINGS
                .iter()
                .find(|binding| binding.method == method)
                .map(|binding| binding.arity)
        };
        // `ipcRenderer.invoke("store-set", name, key, value)`
        assert_eq!(arity(ElectronApiMethod::StoreSet), Some(3));
        assert_eq!(arity(ElectronApiMethod::StoreGet), Some(2));
        // `ipcRenderer.invoke("open-path", path, app)`
        assert_eq!(arity(ElectronApiMethod::OpenPath), Some(2));
        // `ipcRenderer.invoke("read-picked-file", token, path)`
        assert_eq!(arity(ElectronApiMethod::ReadPickedFile), Some(2));
        assert_eq!(arity(ElectronApiMethod::KillSidecar), Some(0));
        assert_eq!(arity(ElectronApiMethod::Relaunch), Some(0));
    }

    #[test]
    fn the_only_member_that_never_touches_ipc_is_get_path_for_file() {
        // The source uses `webUtils.getPathForFile(file)` there, so it has no
        // channel; every other member must name one.
        let non_ipc: Vec<ElectronApiMethod> = METHOD_BINDINGS
            .iter()
            .filter(|binding| binding.channel.is_empty())
            .map(|binding| binding.method)
            .collect();
        assert_eq!(non_ipc, vec![ElectronApiMethod::GetPathForFile]);
    }

    #[test]
    fn the_nested_objects_resolve_their_own_channels() {
        for method in WSL_SERVERS_API_METHODS {
            assert!(wsl_channel_for(method).is_some(), "wsl method {method:?}");
        }
        for method in UPDATER_API_METHODS {
            assert!(
                updater_channel_for(method).is_some(),
                "updater method {method:?}"
            );
        }
        assert_eq!(
            wsl_channel_for(WslServersApiMethod::StartServer),
            Some("wsl-servers-start")
        );
        assert_eq!(
            wsl_channel_for(WslServersApiMethod::AddServer),
            Some("wsl-servers-add")
        );
    }

    #[test]
    fn the_first_subscriber_opens_the_main_process_subscription() {
        let mut bridge = UpdaterBridge::new();
        let mut ipc = Recorder::default();
        assert!(bridge.subscribe(&mut ipc, Box::new(|_| {})));
        assert_eq!(ipc.listeners, vec!["updater-state".to_string()]);
        assert_eq!(ipc.invokes, vec!["updater-subscribe".to_string()]);
        assert!(bridge.is_subscribed());

        // A second subscriber reuses the open subscription.
        assert!(!bridge.subscribe(&mut ipc, Box::new(|_| {})));
        assert_eq!(ipc.listeners.len(), 1);
        assert_eq!(ipc.invokes.len(), 1);
        assert_eq!(bridge.callback_count(), 2);
    }

    #[test]
    fn only_the_last_disposer_tears_the_subscription_down() {
        let mut bridge = UpdaterBridge::new();
        let mut ipc = Recorder::default();
        bridge.subscribe(&mut ipc, Box::new(|_| {}));
        bridge.subscribe(&mut ipc, Box::new(|_| {}));

        assert!(!bridge.unsubscribe(&mut ipc, 0));
        assert!(bridge.is_subscribed());
        assert!(ipc.removed.is_empty());

        assert!(bridge.unsubscribe(&mut ipc, 0));
        assert!(!bridge.is_subscribed());
        assert_eq!(ipc.removed, vec!["updater-state".to_string()]);
        assert_eq!(
            ipc.invokes,
            vec!["updater-subscribe", "updater-unsubscribe"]
        );
    }

    #[test]
    fn a_late_subscriber_is_replayed_the_cached_state() {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let mut bridge = UpdaterBridge::new();
        let mut ipc = Recorder::default();
        let early = Rc::clone(&seen);
        bridge.subscribe(
            &mut ipc,
            Box::new(move |state: &UpdaterState| {
                early.borrow_mut().push(state.status);
            }),
        );

        let checking = UpdaterState {
            status: crate::preload::types::UpdaterStatus::Checking,
            version: None,
            message: None,
        };
        bridge.handle_state(&checking);
        assert_eq!(
            *seen.borrow(),
            vec![crate::preload::types::UpdaterStatus::Checking]
        );

        // The next subscriber gets the cached state replayed on subscribe.
        let late = Rc::clone(&seen);
        bridge.subscribe(
            &mut ipc,
            Box::new(move |state: &UpdaterState| {
                late.borrow_mut().push(state.status);
            }),
        );
        assert_eq!(
            *seen.borrow(),
            vec![
                crate::preload::types::UpdaterStatus::Checking,
                crate::preload::types::UpdaterStatus::Checking
            ]
        );
        assert_eq!(bridge.state(), Some(checking));
    }

    #[test]
    fn an_incoming_event_reaches_every_subscriber() {
        use crate::preload::types::UpdaterStatus;
        let seen = Rc::new(RefCell::new(Vec::new()));
        let mut bridge = UpdaterBridge::new();
        let mut ipc = Recorder::default();
        for _ in 0..2 {
            let seen = Rc::clone(&seen);
            bridge.subscribe(
                &mut ipc,
                Box::new(move |state: &UpdaterState| {
                    seen.borrow_mut().push(state.status);
                }),
            );
        }

        // A main→renderer `updater-state` event is routed into the callbacks
        // that the registered listener closes over.
        assert!(ipc.deliver(
            "updater-state",
            serde_json::json!({ "status": "downloading", "version": "1.2.3" })
        ));
        assert_eq!(*seen.borrow(), vec![UpdaterStatus::Downloading; 2]);
        assert_eq!(bridge.state().unwrap().version, Some("1.2.3".to_string()));

        // A payload with an unknown status is not a state the bridge knows, so
        // it is dropped and the cached state is left alone.
        assert!(ipc.deliver("updater-state", serde_json::json!({ "status": "??" })));
        assert_eq!(seen.borrow().len(), 2);
        assert_eq!(bridge.state().unwrap().status, UpdaterStatus::Downloading);

        // Tearing the subscription down removes the listener with it: the
        // first disposer only drops its callback, the last one closes it.
        assert!(!bridge.unsubscribe(&mut ipc, 0));
        assert!(bridge.unsubscribe(&mut ipc, 0));
        assert!(!ipc.deliver("updater-state", serde_json::json!({ "status": "ready" })));
        assert_eq!(seen.borrow().len(), 2);
    }
}
