//! Rust port of `src/main/native-translations.ts` (opencode v1.18.30).
//!
//! The message *templates* (`DESKTOP_NATIVE_ENGLISH`,
//! `formatDesktopNativeMessage`, `parseDesktopNativeBundle`) are owned by
//! `@opencode-ai/app/i18n/desktop-native`, outside this lane's scope. The
//! bundle registry (`setNativeTranslations` change detection) and the
//! `nativeT` lookup are fully ported; template substitution reuses
//! `crate::renderer::i18n::resolve_template` (`{{name}}`), matching every
//! placeholder shape observable in scope. A missing key falls back to the
//! key itself (documented boundary — the app bundle always carries the 44
//! keys below in production).
//!
//! `DESKTOP_NATIVE_KEYS` enumerates exactly the keys `packages/desktop/src`
//! references, so app-side bundle drift is detectable by diffing this list
//! (plan §10 R4).
//!
//! Original file: `packages/desktop/src/main/native-translations.ts`

use crate::preload::types::DesktopNativeBundle;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub const DESKTOP_NATIVE_KEYS: [&str; 44] = [
    "desktop.dialog.files",
    "desktop.dialog.chooseFolder",
    "desktop.dialog.chooseFile",
    "desktop.dialog.saveFile",
    "desktop.picker.error.notSelected",
    "desktop.picker.error.sizeLimit",
    "desktop.updater.dialog.checkFailed.title",
    "desktop.updater.dialog.checkFailed.message",
    "desktop.updater.dialog.upToDate.title",
    "desktop.updater.dialog.upToDate.message",
    "desktop.updater.dialog.ready.title",
    "desktop.updater.dialog.ready.message",
    "desktop.updater.dialog.restart",
    "desktop.updater.dialog.later",
    "desktop.recovery.action.relaunch",
    "desktop.recovery.action.exportLogs",
    "desktop.recovery.action.keepWaiting",
    "desktop.recovery.action.quit",
    "desktop.recovery.loadFailed",
    "desktop.recovery.loadFailed.detail",
    "desktop.recovery.terminated",
    "desktop.recovery.terminated.detail",
    "desktop.recovery.unknown",
    "desktop.recovery.unresponsive",
    "desktop.recovery.unresponsive.detail",
    "desktop.wsl.error.windowsOnly",
    "desktop.wsl.error.installWsl",
    "desktop.wsl.error.installDistro",
    "desktop.wsl.error.installOpencode",
    "desktop.wsl.error.alreadyAdded",
    "desktop.wsl.error.opencodeMissing",
    "desktop.wsl.error.opencodeCannotRun",
    "desktop.wsl.error.serverExited",
    "desktop.wsl.error.updateVersion",
    "desktop.wsl.error.noVersion",
    "desktop.wsl.error.commandTimeout",
    "desktop.wsl.error.unavailable",
    "desktop.wsl.error.listInstalled",
    "desktop.wsl.error.listOnline",
    "desktop.wsl.error.executeDistro",
    "desktop.wsl.error.opencodeNotInstalled",
    "desktop.wsl.error.healthTimeout",
    "desktop.wsl.error.failedPort",
    "desktop.wsl.error.serverExitedBeforeHealthy",
];

fn bundle_cell() -> &'static Mutex<DesktopNativeBundle> {
    static BUNDLE: OnceLock<Mutex<DesktopNativeBundle>> = OnceLock::new();
    BUNDLE.get_or_init(|| {
        Mutex::new(DesktopNativeBundle {
            // Mirrors the initial `{ locale: "en", … }`; the English
            // message templates themselves are app-owned
            // (`DESKTOP_NATIVE_ENGLISH`).
            locale: "en".to_string(),
            messages: HashMap::new(),
        })
    })
}

pub fn set_native_translations(next: DesktopNativeBundle) -> bool {
    let mut bundle = bundle_cell().lock().expect("native bundle lock");
    if next.locale == bundle.locale
        && DESKTOP_NATIVE_KEYS
            .iter()
            .all(|key| next.messages.get(*key) == bundle.messages.get(*key))
    {
        return false;
    }
    *bundle = next;
    true
}

pub fn native_t(key: &str, params: &[(&str, String)]) -> String {
    let bundle = bundle_cell().lock().expect("native bundle lock");
    match bundle.messages.get(key) {
        Some(template) => crate::renderer::i18n::resolve_template(template, params),
        None => key.to_string(),
    }
}

// PROVISIONAL(packages/app/src/i18n/desktop-native.ts): the canonical
// `parseDesktopNativeBundle` validator lives in the app package. This
// mirror accepts the same shape (`{ locale: string, messages:
// Record<string, string> }`).
pub fn parse_desktop_native_bundle(value: &serde_json::Value) -> Option<DesktopNativeBundle> {
    let record = value.as_object()?;
    let locale = record.get("locale")?.as_str()?;
    let messages = record.get("messages")?.as_object()?;
    let mut table = HashMap::new();
    for (key, message) in messages {
        table.insert(key.clone(), message.as_str()?.to_string());
    }
    Some(DesktopNativeBundle {
        locale: locale.to_string(),
        messages: table,
    })
}

#[cfg(test)]
mod tests {
    // No `src/main/native-translations.test.ts` exists in the source; the
    // cases below pin the ported change detection.
    use super::*;

    #[test]
    fn set_native_translations_reports_changes() {
        // Start from a known bundle (tests share the process-global
        // registry, so reset it first).
        set_native_translations(DesktopNativeBundle {
            locale: "en".to_string(),
            messages: HashMap::new(),
        });
        assert!(!set_native_translations(DesktopNativeBundle {
            locale: "en".to_string(),
            messages: HashMap::new(),
        }));
        assert!(set_native_translations(DesktopNativeBundle {
            locale: "de".to_string(),
            messages: HashMap::new(),
        }));
        assert!(set_native_translations(DesktopNativeBundle {
            locale: "de".to_string(),
            messages: HashMap::from([("desktop.dialog.files".to_string(), "Dateien".to_string())]),
        }));
        assert_eq!(native_t("desktop.dialog.files", &[]), "Dateien".to_string());
        assert_eq!(
            native_t("desktop.missing.key", &[]),
            "desktop.missing.key".to_string()
        );
    }
}
