//! Port of packages/app/src/components/updater-action.ts
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]
use serde::{Deserialize, Serialize};
// PROVISIONAL: SolidJS createMemo/usePlatform pending — mirrors packages/app/src/components/updater-action.ts

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdaterState {
    pub status: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdaterAction {
    pub label: String,
    pub run: Option<String>,
}

pub fn updater_action(state: Option<&UpdaterState>) -> UpdaterAction {
    match state {
        None => UpdaterAction {
            label: "settings.updates.action.checkNow".to_string(),
            run: None,
        },
        Some(s) => match s.status.as_str() {
            "checking" => UpdaterAction {
                label: "settings.updates.action.checking".to_string(),
                run: None,
            },
            "downloading" => UpdaterAction {
                label: "settings.updates.action.downloading".to_string(),
                run: None,
            },
            "ready" => UpdaterAction {
                label: "toast.update.action.installRestart".to_string(),
                run: Some("install".to_string()),
            },
            "installing" => UpdaterAction {
                label: "settings.updates.action.installing".to_string(),
                run: None,
            },
            "disabled" => UpdaterAction {
                label: "settings.updates.action.checkNow".to_string(),
                run: None,
            },
            _ => UpdaterAction {
                label: "settings.updates.action.checkNow".to_string(),
                run: Some("check".to_string()),
            },
        },
    }
}
