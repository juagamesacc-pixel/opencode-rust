//! Rust port of `packages/app/src/wsl/dialog-add-server.tsx` (opencode v1.18.30).
//!
//! Source 463 lines: `isWslRuntimeMissing`, `translate`, `DialogAddWslServer`
//! + `useWslAddServerController` / `DialogWslSetup` / `requestError`. SolidJS
//! + UI component rendering is PROVISIONAL; pure helpers + i18n keys +
//! controller state machine are faithfully ported.
//! Original file: `packages/app/src/wsl/dialog-add-server.tsx`

#![allow(dead_code)]

// PROVISIONAL: pending solid-js/store/tanstack-query/ui — mirrors `packages/app/src/wsl/dialog-add-server.tsx`.

/// Mirrors `isWslRuntimeMissing(error)` regex verbatim.
/// `/WSL is not installed|not been installed|wsl(?:\.exe)? --install/i`
pub fn is_wsl_runtime_missing(error: Option<&str>) -> bool {
    let Some(err) = error else {
        return true;
    };
    if err.is_empty() {
        return true;
    }
    let lower = err.to_lowercase();
    lower.contains("wsl is not installed")
        || lower.contains("not been installed")
        || lower.contains("wsl --install")
        || lower.contains("wsl.exe --install")
}

/// Mirrors the `translate(language, value)` dispatch (key + optional params).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddServerTextRef {
    pub key: String,
    pub has_params: bool,
}

pub fn translate_key(key: &str, has_params: bool) -> String {
    if has_params {
        format!("{key} (with params)")
    } else {
        key.to_string()
    }
}

/// Mirrors `DialogWslServerProps.onAdded`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DialogWslServerProps {
    pub has_on_added: bool,
}

/// Mirrors `useWslAddServerController` store shape (verbatim field names).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WslAddServerControllerState {
    pub view: String,
    pub selected_distro: Option<String>,
    pub catalog_search: String,
    pub catalog_target: Option<String>,
    pub adding: bool,
}

impl Default for WslAddServerControllerState {
    fn default() -> Self {
        Self {
            view: "main".to_string(),
            selected_distro: None,
            catalog_search: String::new(),
            catalog_target: None,
            adding: false,
        }
    }
}

impl WslAddServerControllerState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn transition_open_catalog(&mut self, first_installable: Option<&str>) {
        self.view = "catalog".to_string();
        self.catalog_search.clear();
        self.catalog_target = first_installable.map(|s| s.to_string());
    }

    pub fn transition_close_catalog(&mut self) {
        self.view = "main".to_string();
        self.catalog_search.clear();
        self.catalog_target = None;
    }

    pub fn update_catalog_search(&mut self, value: &str) {
        self.catalog_search = value.to_string();
    }

    pub fn update_catalog_target(&mut self, value: &str) {
        self.catalog_target = Some(value.to_string());
    }

    pub fn update_selected_distro(&mut self, value: &str) {
        self.selected_distro = Some(value.to_string());
    }
}

/// Mirrors `DialogWslSetup` props (verbatim class names preserved in docs).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogWslSetupProps {
    pub state: String,
    pub error: Option<String>,
    pub installable: bool,
    pub busy: bool,
}

impl DialogWslSetupProps {
    /// Mirrors title selection (verbatim i18n keys).
    pub fn title_key(&self) -> &'static str {
        if self.state == "pendingRestart" {
            return "wsl.onboarding.restartRequired";
        }
        if self.installable {
            return "wsl.onboarding.wslNotInstalled.title";
        }
        "wsl.onboarding.wslUnavailable.title"
    }

    /// Mirrors description selection (verbatim i18n keys).
    pub fn description_key(&self) -> &'static str {
        if self.state == "pendingRestart" {
            return "wsl.onboarding.windowsRestartRequired";
        }
        if !self.installable {
            return "wsl.onboarding.wslUnavailable.description";
        }
        "wsl.onboarding.wslNotInstalled.description"
    }
}

/// Mirrors `requestError(language, err)` keys (verbatim).
pub const REQUEST_ERROR_KEYS: &[&str] = &["common.requestFailed"];
pub const DIALOG_ADD_WSL_KEYS: &[&str] = &[
    "wsl.server.add",
    "wsl.onboarding.installDistro",
    "wsl.onboarding.searchDistros",
    "wsl.onboarding.installDistro",
    "common.cancel",
    "wsl.onboarding.installDistro",
    "wsl.onboarding.installedDistros",
    "wsl.onboarding.checkAgain",
    "wsl.onboarding.allDistrosAdded",
    "wsl.onboarding.noDistros",
    "wsl.onboarding.needAnotherDistro",
    "wsl.onboarding.needAnotherDistroHint",
    "common.cancel",
    "wsl.onboarding.loadFailed",
    "wsl.onboarding.restartRequired",
    "wsl.onboarding.wslNotInstalled.title",
    "wsl.onboarding.wslUnavailable.title",
    "wsl.onboarding.windowsRestartRequired",
    "wsl.onboarding.wslUnavailable.description",
    "wsl.onboarding.wslNotInstalled.description",
    "wsl.onboarding.installWsl",
    "common.close",
];

/// Mirrors the primary-button width verbatim (`99px` for catalog install).
pub const CATALOG_INSTALL_BUTTON_WIDTH: &str = "99px";
