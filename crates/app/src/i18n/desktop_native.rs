//! Rust port of `packages/app/src/i18n/desktop-native.ts` (opencode v1.18.30).
//!
//! 1:1 data tables + pure helpers. Solid/Electron-free logic ported verbatim;
//! `Intl.Locale`/`Intl.PluralRules` surfaces are PROVISIONAL (pending host
//! locale crate) — mirrors `packages/app/src/i18n/desktop-native.ts`.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Mirrors `DESKTOP_NATIVE_LOCALES` (source order preserved).
pub const DESKTOP_NATIVE_LOCALES: &[&str] = &[
    "en", "zh", "zht", "ko", "de", "es", "fr", "da", "ja", "pl", "ru", "uk", "bs", "ar", "no",
    "br", "th", "tr", "hi", "nl", "id", "vi", "it", "ur", "pa", "az", "fi", "sv", "am", "bg", "bn",
    "ca", "cs", "dv", "dz", "el", "et", "fa", "fo", "hr", "hu", "hy", "is", "ka", "km", "lo", "lt",
    "lv", "mk", "mn", "ms", "my", "ne", "ro", "si", "sk", "sl", "sq", "sr", "tg", "tk", "uz",
];

/// Mirrors `DESKTOP_NATIVE_LABELS`.
pub const DESKTOP_NATIVE_LABELS: &[(&str, &str)] = &[
    ("en", "English"),
    ("zh", "简体中文"),
    ("zht", "繁體中文"),
    ("ko", "한국어"),
    ("de", "Deutsch"),
    ("es", "Español"),
    ("fr", "Français"),
    ("da", "Dansk"),
    ("ja", "日本語"),
    ("pl", "Polski"),
    ("ru", "Русский"),
    ("uk", "Українська"),
    ("bs", "Bosanski"),
    ("ar", "العربية"),
    ("no", "Norsk"),
    ("br", "Português (Brasil)"),
    ("th", "ไทย"),
    ("tr", "Türkçe"),
    ("hi", "हिन्दी"),
    ("nl", "Nederlands"),
    ("id", "Bahasa Indonesia"),
    ("vi", "Tiếng Việt"),
    ("it", "Italiano"),
    ("ur", "اردو"),
    ("pa", "پنجابی"),
    ("az", "Azərbaycanca"),
    ("fi", "Suomi"),
    ("sv", "Svenska"),
    ("am", "አማርኛ"),
    ("bg", "Български"),
    ("bn", "বাংলা"),
    ("ca", "Català"),
    ("cs", "Čeština"),
    ("dv", "ދިވެހި"),
    ("dz", "རྫོང་ཁ"),
    ("el", "Ελληνικά"),
    ("et", "Eesti"),
    ("fa", "فارسی"),
    ("fo", "Føroyskt"),
    ("hr", "Hrvatski"),
    ("hu", "Magyar"),
    ("hy", "Հայերեն"),
    ("is", "Íslenska"),
    ("ka", "ქართული"),
    ("km", "ខ្មែរ"),
    ("lo", "ລາວ"),
    ("lt", "Lietuvių"),
    ("lv", "Latviešu"),
    ("mk", "Македонски"),
    ("mn", "Монгол"),
    ("ms", "Bahasa Melayu"),
    ("my", "မြန်မာ"),
    ("ne", "नेपाली"),
    ("ro", "Română"),
    ("si", "සිංහල"),
    ("sk", "Slovenčina"),
    ("sl", "Slovenščina"),
    ("sq", "Shqip"),
    ("sr", "Српски"),
    ("tg", "Тоҷикӣ"),
    ("tk", "Türkmençe"),
    ("uz", "Oʻzbekcha"),
];

/// Mirrors `DESKTOP_NATIVE_LOCALE_TAGS`.
pub const DESKTOP_NATIVE_LOCALE_TAGS: &[(&str, &str)] = &[
    ("en", "en"),
    ("zh", "zh-Hans"),
    ("zht", "zh-Hant"),
    ("ko", "ko"),
    ("de", "de"),
    ("es", "es"),
    ("fr", "fr"),
    ("da", "da"),
    ("ja", "ja"),
    ("pl", "pl"),
    ("ru", "ru"),
    ("uk", "uk"),
    ("bs", "bs"),
    ("ar", "ar"),
    ("no", "nb-NO"),
    ("br", "pt-BR"),
    ("th", "th"),
    ("tr", "tr"),
    ("hi", "hi-IN"),
    ("nl", "nl-NL"),
    ("id", "id-ID"),
    ("vi", "vi-VN"),
    ("it", "it-IT"),
    ("ur", "ur-PK"),
    ("pa", "pa-Arab-PK"),
    ("az", "az-Latn-AZ"),
    ("fi", "fi-FI"),
    ("sv", "sv-SE"),
    ("am", "am-ET"),
    ("bg", "bg-BG"),
    ("bn", "bn-BD"),
    ("ca", "ca-AD"),
    ("cs", "cs-CZ"),
    ("dv", "dv-MV"),
    ("dz", "dz-BT"),
    ("el", "el-GR"),
    ("et", "et-EE"),
    ("fa", "fa-IR"),
    ("fo", "fo-FO"),
    ("hr", "hr-HR"),
    ("hu", "hu-HU"),
    ("hy", "hy-AM"),
    ("is", "is-IS"),
    ("ka", "ka-GE"),
    ("km", "km-KH"),
    ("lo", "lo-LA"),
    ("lt", "lt-LT"),
    ("lv", "lv-LV"),
    ("mk", "mk-MK"),
    ("mn", "mn-MN"),
    ("ms", "ms-MY"),
    ("my", "my-MM"),
    ("ne", "ne-NP"),
    ("ro", "ro-RO"),
    ("si", "si-LK"),
    ("sk", "sk-SK"),
    ("sl", "sl-SI"),
    ("sq", "sq-AL"),
    ("sr", "sr-Cyrl-RS"),
    ("tg", "tg-Cyrl-TJ"),
    ("tk", "tk-Latn-TM"),
    ("uz", "uz-Latn-UZ"),
];

/// Mirrors `DESKTOP_NATIVE_ENGLISH` (source key order preserved).
pub const DESKTOP_NATIVE_ENGLISH: &[(&str, &str)] = &[
    ("desktop.menu.app", "OpenCode"),
    ("desktop.menu.file", "File"),
    ("desktop.menu.edit", "Edit"),
    ("desktop.menu.view", "View"),
    ("desktop.menu.go", "Go"),
    ("desktop.menu.window", "Window"),
    ("desktop.menu.help", "Help"),
    ("desktop.menu.checkForUpdates", "Check for Updates..."),
    ("desktop.menu.settings", "Settings"),
    ("desktop.menu.reloadWebview", "Reload Webview"),
    ("desktop.menu.restart", "Restart"),
    ("desktop.menu.exportLogs", "Export Logs..."),
    ("desktop.menu.newSession", "New Session"),
    ("desktop.menu.openProject", "Open Project..."),
    ("desktop.menu.newWindow", "New Window"),
    ("desktop.menu.closeWindow", "Close Window"),
    ("desktop.menu.undo", "Undo"),
    ("desktop.menu.redo", "Redo"),
    ("desktop.menu.cut", "Cut"),
    ("desktop.menu.copy", "Copy"),
    ("desktop.menu.paste", "Paste"),
    ("desktop.menu.delete", "Delete"),
    ("desktop.menu.selectAll", "Select All"),
    ("desktop.menu.toggleSidebar", "Toggle Sidebar"),
    ("desktop.menu.toggleTerminal", "Toggle Terminal"),
    ("desktop.menu.toggleFileTree", "Toggle File Tree"),
    ("desktop.menu.reload", "Reload"),
    ("desktop.menu.toggleDeveloperTools", "Toggle Developer Tools"),
    ("desktop.menu.actualSize", "Actual Size"),
    ("desktop.menu.zoomIn", "Zoom In"),
    ("desktop.menu.zoomOut", "Zoom Out"),
    ("desktop.menu.toggleFullScreen", "Toggle Full Screen"),
    ("desktop.menu.back", "Back"),
    ("desktop.menu.forward", "Forward"),
    ("desktop.menu.previousSession", "Previous Session"),
    ("desktop.menu.nextSession", "Next Session"),
    ("desktop.menu.previousProject", "Previous Project"),
    ("desktop.menu.nextProject", "Next Project"),
    ("desktop.menu.minimize", "Minimize"),
    ("desktop.menu.maximize", "Maximize"),
    ("desktop.menu.documentation", "OpenCode Documentation"),
    ("desktop.menu.supportForum", "Support Forum"),
    ("desktop.menu.shareFeedback", "Share Feedback"),
    ("desktop.menu.reportBug", "Report a Bug"),
    ("desktop.menu.ariaLabel", "OpenCode menu"),
    ("desktop.updater.dialog.checkFailed.message", "Update check failed."),
    ("desktop.updater.dialog.checkFailed.title", "Update Error"),
    ("desktop.updater.dialog.upToDate.message", "You're up to date."),
    ("desktop.updater.dialog.upToDate.title", "No Updates"),
    ("desktop.updater.dialog.ready.message", "Update {{version}} downloaded. Restart now?"),
    ("desktop.updater.dialog.ready.title", "Update Ready"),
    ("desktop.updater.dialog.restart", "Restart"),
    ("desktop.updater.dialog.later", "Later"),
    ("desktop.recovery.action.relaunch", "Relaunch"),
    ("desktop.recovery.action.exportLogs", "Export Logs"),
    ("desktop.recovery.action.keepWaiting", "Keep Waiting"),
    ("desktop.recovery.action.quit", "Quit"),
    ("desktop.recovery.loadFailed", "OpenCode failed to load"),
    ("desktop.recovery.terminated", "OpenCode window terminated unexpectedly"),
    ("desktop.recovery.unresponsive", "OpenCode is not responding"),
    ("desktop.recovery.unresponsive.detail", "You can relaunch the app, open the logs, or keep waiting."),
    ("desktop.recovery.loadFailed.detail", "Window: {{window}}\\nURL: {{url}}\\nError: {{code}} {{description}}"),
    ("desktop.recovery.terminated.detail", "Window: {{window}}\\nReason: {{reason}}\\nCode: {{code}}"),
    ("desktop.recovery.unknown", "<unknown>"),
    ("desktop.dialog.chooseFolder", "Choose a folder"),
    ("desktop.dialog.chooseFile", "Choose a file"),
    ("desktop.dialog.saveFile", "Save file"),
    ("desktop.dialog.files", "Files"),
    ("desktop.server.local", "Local Server"),
    ("desktop.wsl.error.windowsOnly", "WSL is only available on Windows"),
    ("desktop.wsl.error.unavailable", "WSL is unavailable"),
    ("desktop.wsl.error.listInstalled", "Failed to list installed WSL distros"),
    ("desktop.wsl.error.listOnline", "Failed to list online WSL distros"),
    ("desktop.wsl.error.executeDistro", "Cannot execute commands in distro"),
    ("desktop.wsl.error.installWsl", "WSL installation failed"),
    ("desktop.wsl.error.installDistro", "Failed to install distro: {{distro}}"),
    ("desktop.wsl.error.installOpencode", "OpenCode installation failed"),
    ("desktop.wsl.error.alreadyAdded", "{{distro}} is already added"),
    ("desktop.wsl.error.opencodeMissing", "opencode is not installed in this distro"),
    ("desktop.wsl.error.opencodeCannotRun", "opencode is installed but could not run"),
    ("desktop.wsl.error.opencodeNotInstalled", "OpenCode is not installed in {{distro}}"),
    ("desktop.wsl.error.updateVersion", "OpenCode update finished but {{distro}} still reports {{installed}}; expected {{expected}}"),
    ("desktop.wsl.error.noVersion", "no version"),
    ("desktop.wsl.error.serverExited", "WSL server exited after startup (code={{code}} signal={{signal}})"),
    ("desktop.wsl.error.serverExitedBeforeHealthy", "WSL server exited before becoming healthy (code={{code}} signal={{signal}}){{output}}"),
    ("desktop.wsl.error.healthTimeout", "Sidecar for {{distro}} health check timed out after {{timeout}}ms"),
    ("desktop.wsl.error.commandTimeout", "{{command}} {{args}} timed out after {{timeout}}ms"),
    ("desktop.wsl.error.failedPort", "Failed to get port"),
    ("desktop.picker.error.notSelected", "File was not selected by the picker"),
    ("desktop.picker.error.sizeLimit", "Selected attachments exceed the {{limit}} MB limit"),
];

/// Mirrors `DESKTOP_NATIVE_KEYS` (insertion order of `DESKTOP_NATIVE_ENGLISH`).
pub const DESKTOP_NATIVE_KEYS: &[&str] = &[
    "desktop.menu.app",
    "desktop.menu.file",
    "desktop.menu.edit",
    "desktop.menu.view",
    "desktop.menu.go",
    "desktop.menu.window",
    "desktop.menu.help",
    "desktop.menu.checkForUpdates",
    "desktop.menu.settings",
    "desktop.menu.reloadWebview",
    "desktop.menu.restart",
    "desktop.menu.exportLogs",
    "desktop.menu.newSession",
    "desktop.menu.openProject",
    "desktop.menu.newWindow",
    "desktop.menu.closeWindow",
    "desktop.menu.undo",
    "desktop.menu.redo",
    "desktop.menu.cut",
    "desktop.menu.copy",
    "desktop.menu.paste",
    "desktop.menu.delete",
    "desktop.menu.selectAll",
    "desktop.menu.toggleSidebar",
    "desktop.menu.toggleTerminal",
    "desktop.menu.toggleFileTree",
    "desktop.menu.reload",
    "desktop.menu.toggleDeveloperTools",
    "desktop.menu.actualSize",
    "desktop.menu.zoomIn",
    "desktop.menu.zoomOut",
    "desktop.menu.toggleFullScreen",
    "desktop.menu.back",
    "desktop.menu.forward",
    "desktop.menu.previousSession",
    "desktop.menu.nextSession",
    "desktop.menu.previousProject",
    "desktop.menu.nextProject",
    "desktop.menu.minimize",
    "desktop.menu.maximize",
    "desktop.menu.documentation",
    "desktop.menu.supportForum",
    "desktop.menu.shareFeedback",
    "desktop.menu.reportBug",
    "desktop.menu.ariaLabel",
    "desktop.updater.dialog.checkFailed.message",
    "desktop.updater.dialog.checkFailed.title",
    "desktop.updater.dialog.upToDate.message",
    "desktop.updater.dialog.upToDate.title",
    "desktop.updater.dialog.ready.message",
    "desktop.updater.dialog.ready.title",
    "desktop.updater.dialog.restart",
    "desktop.updater.dialog.later",
    "desktop.recovery.action.relaunch",
    "desktop.recovery.action.exportLogs",
    "desktop.recovery.action.keepWaiting",
    "desktop.recovery.action.quit",
    "desktop.recovery.loadFailed",
    "desktop.recovery.terminated",
    "desktop.recovery.unresponsive",
    "desktop.recovery.unresponsive.detail",
    "desktop.recovery.loadFailed.detail",
    "desktop.recovery.terminated.detail",
    "desktop.recovery.unknown",
    "desktop.dialog.chooseFolder",
    "desktop.dialog.chooseFile",
    "desktop.dialog.saveFile",
    "desktop.dialog.files",
    "desktop.server.local",
    "desktop.wsl.error.windowsOnly",
    "desktop.wsl.error.unavailable",
    "desktop.wsl.error.listInstalled",
    "desktop.wsl.error.listOnline",
    "desktop.wsl.error.executeDistro",
    "desktop.wsl.error.installWsl",
    "desktop.wsl.error.installDistro",
    "desktop.wsl.error.installOpencode",
    "desktop.wsl.error.alreadyAdded",
    "desktop.wsl.error.opencodeMissing",
    "desktop.wsl.error.opencodeCannotRun",
    "desktop.wsl.error.opencodeNotInstalled",
    "desktop.wsl.error.updateVersion",
    "desktop.wsl.error.noVersion",
    "desktop.wsl.error.serverExited",
    "desktop.wsl.error.serverExitedBeforeHealthy",
    "desktop.wsl.error.healthTimeout",
    "desktop.wsl.error.commandTimeout",
    "desktop.wsl.error.failedPort",
    "desktop.picker.error.notSelected",
    "desktop.picker.error.sizeLimit",
];

/// Mirrors `DESKTOP_NATIVE_MAX_PAYLOAD_BYTES`.
pub const DESKTOP_NATIVE_MAX_PAYLOAD_BYTES: usize = 64 * 1024;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesktopNativeBundle {
    pub locale: String,
    pub messages: std::collections::HashMap<String, String>,
}

/// Mirrors `createDesktopNativeBundle(locale, translate)`.
#[allow(non_snake_case)]
pub fn createDesktopNativeBundle(
    locale: &str,
    translate: impl Fn(&str) -> String,
) -> DesktopNativeBundle {
    let mut messages = std::collections::HashMap::new();
    for key in DESKTOP_NATIVE_KEYS {
        messages.insert((*key).to_string(), translate(key));
    }
    DesktopNativeBundle {
        locale: locale.to_string(),
        messages,
    }
}

/// Mirrors `parseDesktopNativeBundle(value)`.
#[allow(non_snake_case)]
pub fn parseDesktopNativeBundle(
    bundle: &DesktopNativeBundle,
    byte_len: usize,
) -> Option<DesktopNativeBundle> {
    if byte_len > DESKTOP_NATIVE_MAX_PAYLOAD_BYTES {
        return None;
    }
    if !DESKTOP_NATIVE_LOCALES.contains(&bundle.locale.as_str()) {
        return None;
    }
    if bundle.messages.len() != DESKTOP_NATIVE_KEYS.len() {
        return None;
    }
    for key in DESKTOP_NATIVE_KEYS {
        if !bundle.messages.contains_key(*key) {
            return None;
        }
    }
    Some(bundle.clone())
}

/// Mirrors `formatDesktopNativeMessage(message, params)` — `{{key}}` interpolation verbatim.
#[allow(non_snake_case)]
pub fn formatDesktopNativeMessage(
    message: &str,
    params: Option<&std::collections::HashMap<String, String>>,
) -> String {
    let Some(params) = params else {
        return message.to_string();
    };
    // Verbatim regex: /\{\{([^{}]+)\}\}/g — placeholder keys contain no braces;
    // unknown keys are left as-is.
    let chars: Vec<char> = message.chars().collect();
    let mut out = String::new();
    let mut j = 0;
    while j < chars.len() {
        if chars[j] == '{' && chars.get(j + 1) == Some(&'{') {
            let mut k = j + 2;
            let mut key = String::new();
            let mut closed = false;
            while k < chars.len() {
                if chars[k] == '}' && chars.get(k + 1) == Some(&'}') {
                    closed = true;
                    break;
                }
                if chars[k] == '{' || chars[k] == '}' {
                    break;
                }
                key.push(chars[k]);
                k += 1;
            }
            if closed {
                if let Some(v) = params.get(&key) {
                    out.push_str(v);
                } else {
                    out.push_str(&chars[j..k + 2].iter().collect::<String>());
                }
                j = k + 2;
                continue;
            }
        }
        out.push(chars[j]);
        j += 1;
    }
    out
}

// PROVISIONAL: pending host-locale (Intl) crate — mirrors detectDesktopNativeLocale/desktopNativePluralCategories from packages/app/src/i18n/desktop-native.ts
/// Locale-tag lookup backing `detectDesktopNativeLocale` (tag table only; Intl maximize pending).
pub fn desktop_native_locale_tag(locale: &str) -> Option<&'static str> {
    DESKTOP_NATIVE_LOCALE_TAGS
        .iter()
        .find(|(k, _)| *k == locale)
        .map(|(_, v)| *v)
}
