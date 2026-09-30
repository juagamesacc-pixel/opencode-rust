//! Rust port of `src/main/migrate.ts` (opencode v1.18.30).
//!
//! Ported exactly: the `TAURI_MIGRATED_KEY`, the three per-OS `tauriDir`
//! branches (macOS `Library/Application Support`, Windows `APPDATA` with the
//! `homedir()/AppData/Roaming` fallback, everything else `XDG_DATA_HOME`
//! with the `homedir()/.local/share` fallback), the `TAURI_APP_IDS` table,
//! the packaged/unpackaged `tauriAppId` decision, the
//! `opencode.settings.dat` → `opencode.settings` store-name special case, and
//! the per-key "skip if already present" merge order.
//!
//! PROVISIONAL: `existsSync`/`readdirSync`/`readFileSync` (Node `fs`),
//! `homedir()` (Node `os`), `getStore` (Electron) and `electron-log` have no
//! in-workspace Rust binding.
//!
//! Original file: `packages/desktop/src/main/migrate.ts`

use crate::main::constants::{channel, Channel};
use crate::main::store::get_store;

/// Mirrors `const TAURI_MIGRATED_KEY = "tauriMigrated"`.
pub const TAURI_MIGRATED_KEY: &str = "tauriMigrated";

/// The `process.platform` branches of `tauriDir`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Darwin,
    Win32,
    Other,
}

impl Platform {
    /// Mirrors the `process.platform` read. The source's `switch` only
    /// special-cases `darwin` and `win32`; everything else takes `default`.
    pub fn current() -> Self {
        match std::env::consts::OS {
            "macos" => Platform::Darwin,
            "windows" => Platform::Win32,
            _ => Platform::Other,
        }
    }
}

/// Mirrors `TAURI_APP_IDS`.
pub const TAURI_APP_IDS: [(Channel, &str); 3] = [
    (Channel::Dev, "ai.opencode.desktop.dev"),
    (Channel::Beta, "ai.opencode.desktop.beta"),
    (Channel::Prod, "ai.opencode.desktop"),
];

/// Mirrors `tauriDir(id)` for one platform, given the resolved environment.
///
/// `homedir()` is passed in because Node's `os.homedir()` has no
/// in-workspace Rust binding.
pub fn tauri_dir(
    platform: Platform,
    id: &str,
    home: &str,
    env: &dyn Fn(&str) -> Option<String>,
) -> String {
    let separator = match platform {
        Platform::Win32 => '\\',
        _ => '/',
    };
    match platform {
        Platform::Darwin => join(&[home, "Library", "Application Support", id], separator),
        Platform::Win32 => {
            let appdata =
                env("APPDATA").unwrap_or_else(|| join(&[home, "AppData", "Roaming"], separator));
            join(&[&appdata, id], separator)
        }
        Platform::Other => {
            let xdg =
                env("XDG_DATA_HOME").unwrap_or_else(|| join(&[home, ".local", "share"], separator));
            join(&[&xdg, id], separator)
        }
    }
}

/// Mirrors `function tauriAppId()`.
pub fn tauri_app_id(is_packaged: bool) -> String {
    if !is_packaged {
        return "ai.opencode.desktop.dev".to_string();
    }
    let channel = channel();
    TAURI_APP_IDS
        .iter()
        .find(|(candidate, _)| *candidate == channel)
        .map(|(_, id)| id.to_string())
        // `TAURI_APP_IDS[CHANNEL]` is always defined for the three channels.
        .unwrap_or_else(|| "ai.opencode.desktop.dev".to_string())
}

/// Mirrors the `storeName` special case in `migrateFile`.
pub fn store_name_for(filename: &str) -> String {
    if filename == "opencode.settings.dat" {
        "opencode.settings".to_string()
    } else {
        filename.to_string()
    }
}

/// Mirrors the `migrated`/`skipped` split of `migrateFile`: an existing key
/// is never overwritten, and the migrated entries keep the `(key, value)`
/// pairs `Object.entries(data)` produced.
#[derive(Debug, Clone, PartialEq)]
pub struct MergeOutcome {
    pub migrated: Vec<(String, serde_json::Value)>,
    pub skipped: Vec<String>,
}

/// Mirrors `migrateFile`'s per-key loop, given the target store's existing
/// keys and returning the writes to apply.
pub fn merge_keys(existing_keys: &[String], data: &[(String, serde_json::Value)]) -> MergeOutcome {
    let mut migrated = Vec::new();
    let mut skipped = Vec::new();
    for (key, value) in data {
        // Don't overwrite values the user has already set in the Electron app.
        if existing_keys.contains(key) {
            skipped.push(key.clone());
            continue;
        }
        migrated.push((key.clone(), value.clone()));
    }
    MergeOutcome { migrated, skipped }
}

/// Mirrors the `for (const filename of readdirSync(dir))` filter.
pub fn is_migratable(filename: &str) -> bool {
    filename.ends_with(".dat")
}

/// A `node:path` `join` for the platform's separator. Node's `path.join`
/// resolves against `path.sep`, so a win32 `join` emits backslashes even when
/// the inputs carry forward slashes.
fn join(parts: &[&str], separator: char) -> String {
    let mut out = String::new();
    for part in parts {
        if out.is_empty() {
            out.push_str(part);
            continue;
        }
        if out.ends_with('/') || out.ends_with('\\') {
            out.push_str(part);
            continue;
        }
        out.push(separator);
        out.push_str(part);
    }
    out
}

/// JavaScript truthiness, as `if (getStore().get(TAURI_MIGRATED_KEY))`
/// evaluates it.
pub fn is_truthy(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Null => false,
        serde_json::Value::Bool(flag) => *flag,
        serde_json::Value::Number(number) => number.as_f64().is_some_and(|number| number != 0.0),
        serde_json::Value::String(text) => !text.is_empty(),
        // Every object and array is truthy in JavaScript, including `[]`.
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => true,
    }
}

/// Mirrors `export function migrate()`.
///
/// PROVISIONAL(packages/desktop/src/main/migrate.ts): needs `getStore`,
/// `existsSync`, `readdirSync` and `readFileSync`.
pub fn migrate() -> Result<(), String> {
    let store = get_store(None)?;
    // `if (getStore().get(TAURI_MIGRATED_KEY))` — JavaScript truthiness, not
    // `=== true`.
    let already_migrated = store
        .borrow()
        .get(TAURI_MIGRATED_KEY)
        .is_some_and(|value| is_truthy(&value));
    if already_migrated {
        // PROVISIONAL(packages/desktop/src/main/migrate.ts):
        // `log.log("tauri migration: already done, skipping")`.
        return Ok(());
    }
    Err("migrate() needs the Node filesystem and electron-store".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> + '_ {
        move |key| {
            pairs
                .iter()
                .find(|(name, _)| *name == key)
                .map(|(_, value)| value.to_string())
        }
    }

    #[test]
    fn resolves_the_tauri_directory_per_os() {
        assert_eq!(
            tauri_dir(
                Platform::Darwin,
                "ai.opencode.desktop",
                "/Users/luke",
                &env(&[])
            ),
            "/Users/luke/Library/Application Support/ai.opencode.desktop"
        );
        // `path.join` on win32 emits backslashes, so the APPDATA branch is
        // `C:\Roaming\id` rather than `C:\Roaming/id`.
        assert_eq!(
            tauri_dir(
                Platform::Win32,
                "id",
                "C:\\Users\\luke",
                &env(&[("APPDATA", "C:\\Roaming")])
            ),
            "C:\\Roaming\\id"
        );
        assert_eq!(
            tauri_dir(Platform::Win32, "id", "C:\\Users\\luke", &env(&[])),
            "C:\\Users\\luke\\AppData\\Roaming\\id"
        );
        assert_eq!(
            tauri_dir(
                Platform::Other,
                "id",
                "/home/luke",
                &env(&[("XDG_DATA_HOME", "/xdg")])
            ),
            "/xdg/id"
        );
        assert_eq!(
            tauri_dir(Platform::Other, "id", "/home/luke", &env(&[])),
            "/home/luke/.local/share/id"
        );
    }

    #[test]
    fn the_app_identifier_follows_the_channel_only_when_packaged() {
        assert_eq!(tauri_app_id(false), "ai.opencode.desktop.dev");
        // `CHANNEL` is read from `OPENCODE_CHANNEL`; assert the table itself
        // so the test does not depend on the ambient environment.
        assert_eq!(TAURI_APP_IDS[0], (Channel::Dev, "ai.opencode.desktop.dev"));
        assert_eq!(
            TAURI_APP_IDS[1],
            (Channel::Beta, "ai.opencode.desktop.beta")
        );
        assert_eq!(TAURI_APP_IDS[2], (Channel::Prod, "ai.opencode.desktop"));
    }

    #[test]
    fn the_settings_dat_maps_to_the_bare_settings_store() {
        assert_eq!(store_name_for("opencode.settings.dat"), "opencode.settings");
        assert_eq!(store_name_for("opencode.global.dat"), "opencode.global.dat");
        assert_eq!(store_name_for("default.dat"), "default.dat");
    }

    #[test]
    fn existing_keys_are_never_overwritten() {
        let outcome = merge_keys(
            &["theme".to_string()],
            &[
                ("theme".to_string(), serde_json::json!("dark")),
                ("language".to_string(), serde_json::json!("ja")),
            ],
        );
        assert_eq!(outcome.skipped, vec!["theme".to_string()]);
        assert_eq!(
            outcome.migrated,
            vec![("language".to_string(), serde_json::json!("ja"))]
        );
    }

    #[test]
    fn the_migrated_flag_uses_javascript_truthiness() {
        assert!(!is_truthy(&serde_json::Value::Null));
        assert!(!is_truthy(&serde_json::json!(false)));
        assert!(!is_truthy(&serde_json::json!(0)));
        assert!(!is_truthy(&serde_json::json!("")));
        assert!(is_truthy(&serde_json::json!(true)));
        assert!(is_truthy(&serde_json::json!(1)));
        assert!(is_truthy(&serde_json::json!("yes")));
        // Empty containers are still truthy in JavaScript.
        assert!(is_truthy(&serde_json::json!([])));
        assert!(is_truthy(&serde_json::json!({})));
    }

    #[test]
    fn only_dat_files_are_migrated() {
        assert!(is_migratable("opencode.settings.dat"));
        assert!(is_migratable("default.dat"));
        assert!(!is_migratable("opencode.global"));
        assert!(!is_migratable("readme.txt"));
    }
}
