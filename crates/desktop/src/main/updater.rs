//! Rust port of `src/main/updater.ts` (opencode v1.18.30).
//!
//! Fully ported: the `"ready"` persistence key, the `autoUpdater`
//! configuration values, and the `showUpdaterDialog` state dispatch.
//! PROVISIONAL: `setupAutoUpdater` (`electron-updater`, `electron-store`
//! `opencode.updater`, `app.getVersion()`) and the `dialog.showMessageBox`
//! calls in `showUpdaterDialog`.
//!
//! Original file: `packages/desktop/src/main/updater.ts`

use crate::main::updater_controller::UpdaterReadyRecord;
use crate::preload::types::UpdaterStatus;

pub const UPDATER_STORE_NAME: &str = "opencode.updater";
pub const READY_KEY: &str = "ready";

pub const AUTO_UPDATER_CHANNEL: &str = "latest";
pub const AUTO_UPDATER_ALLOW_PRERELEASE: bool = false;
pub const AUTO_UPDATER_ALLOW_DOWNGRADE: bool = true;
pub const AUTO_UPDATER_AUTO_DOWNLOAD: bool = false;
pub const AUTO_UPDATER_AUTO_INSTALL_ON_APP_QUIT: bool = false;

/// Mirrors the `persistence.get` validation in `setupAutoUpdater`: the
/// stored value must be an object with a string `version`.
pub fn parse_ready_record(value: &serde_json::Value) -> Option<UpdaterReadyRecord> {
    let version = value.as_object()?.get("version")?.as_str()?;
    Some(UpdaterReadyRecord {
        version: version.to_string(),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdaterDialogAction {
    /// `error` state with `alertOnFail`: error message box.
    ShowCheckFailed,
    /// `up-to-date` state with `alertOnFail`: info message box.
    ShowUpToDate,
    /// `ready` state: restart/later message box.
    OfferRestart,
    /// Anything else (or alerts suppressed): no dialog.
    None,
}

/// Mirrors the `showUpdaterDialog` state dispatch (`alertOnFail` gates the
/// `error`/`up-to-date` dialogs; only `ready` offers restart).
pub fn updater_dialog_action(status: UpdaterStatus, alert_on_fail: bool) -> UpdaterDialogAction {
    match status {
        UpdaterStatus::Error => {
            if alert_on_fail {
                UpdaterDialogAction::ShowCheckFailed
            } else {
                UpdaterDialogAction::None
            }
        }
        UpdaterStatus::UpToDate => {
            if alert_on_fail {
                UpdaterDialogAction::ShowUpToDate
            } else {
                UpdaterDialogAction::None
            }
        }
        UpdaterStatus::Ready => UpdaterDialogAction::OfferRestart,
        _ => UpdaterDialogAction::None,
    }
}

/// Mirrors the restart-button index (`response.response === 0` installs).
pub fn should_install_on_dialog_response(response_index: usize) -> bool {
    response_index == 0
}

// PROVISIONAL(packages/desktop/src/main/updater.ts):
// `setupAutoUpdater(stop)` needs `electron-updater` (`autoUpdater` with
// `channel: "latest"`, prerelease/downgrade/download/install flags,
// `checkForUpdates`/`downloadUpdate`/`quitAndInstall` with the
// `setAppQuitting()` guard dance), the `opencode.updater` store keyed at
// `"ready"`, `app.getVersion()`, and the `"auto updater configured"` log.
pub fn setup_auto_updater() {
    unimplemented!("electron-updater + electron-store binding")
}

// PROVISIONAL(packages/desktop/src/main/updater.ts): `showUpdaterDialog`
// needs `controller.check()` plus `dialog.showMessageBox` (error/info
// dialogs, restart/later buttons with `defaultId: 0, cancelId: 1`).
// Dispatch is `updater_dialog_action` above; the `nativeT` dialog keys are
// `desktop.updater.dialog.checkFailed.*`, `.upToDate.*`, `.ready.*`,
// `.restart`, `.later`.
pub fn show_updater_dialog(_alert_on_fail: bool) {
    unimplemented!("Electron dialog binding")
}

#[cfg(test)]
mod tests {
    // No `src/main/updater.test.ts` exists in the source; the cases below
    // pin the ported dialog dispatch and record parsing.
    use super::*;

    #[test]
    fn dialog_action_dispatch_matches_states() {
        assert_eq!(
            updater_dialog_action(UpdaterStatus::Error, true),
            UpdaterDialogAction::ShowCheckFailed
        );
        assert_eq!(
            updater_dialog_action(UpdaterStatus::Error, false),
            UpdaterDialogAction::None
        );
        assert_eq!(
            updater_dialog_action(UpdaterStatus::UpToDate, true),
            UpdaterDialogAction::ShowUpToDate
        );
        assert_eq!(
            updater_dialog_action(UpdaterStatus::Ready, false),
            UpdaterDialogAction::OfferRestart
        );
        assert_eq!(
            updater_dialog_action(UpdaterStatus::Checking, true),
            UpdaterDialogAction::None
        );
        assert!(should_install_on_dialog_response(0));
        assert!(!should_install_on_dialog_response(1));
    }

    #[test]
    fn ready_record_requires_a_version_string() {
        assert!(parse_ready_record(&serde_json::json!({ "version": "2.0.0" })).is_some());
        assert!(parse_ready_record(&serde_json::json!({ "version": 2 })).is_none());
        assert!(parse_ready_record(&serde_json::json!(null)).is_none());
    }
}
