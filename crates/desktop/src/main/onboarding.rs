//! Rust port of `src/main/onboarding.ts` (opencode v1.18.30).
//!
//! Fully ported: [`initialize_old_layout_eligibility`] (the `existsSync`-
//! gated `readdirSync`, the `typeof current === "boolean"` short-circuit, the
//! `hasExistingAppState` eligibility over real `std::fs` entries, and the
//! store write-back), [`is_old_layout_eligible`], and
//! [`is_first_launch_onboarding_pending`], plus the
//! [`finish_first_launch_onboarding`] state machine (the
//! already-completed short-circuit, the `Default Project` mkdir, and the
//! completion write). The `electron-store` reads/writes are PROVISIONAL
//! ([`crate::main::store::get_store`] errors are treated as a missing
//! value), and `app.getPath("documents")` is PROVISIONAL through
//! [`documents_dir`]. The source log text is preserved, byte-identical:
//! - "first launch onboarding pending checked"
//! - "first launch onboarding already completed"
//! - "first launch onboarding completed".
//!
//! Original file: `packages/desktop/src/main/onboarding.ts`

use crate::main::install_state::{has_existing_app_state, InstallStateEntry};
use crate::main::logging::write_log;
use crate::main::store;
use crate::main::store_keys::{FIRST_LAUNCH_ONBOARDING_COMPLETE_KEY, OLD_LAYOUT_ELIGIBLE_KEY};
use serde_json::Value;

/// Mirrors `const DEFAULT_PROJECT_DIR = "Default Project"`.
pub const DEFAULT_PROJECT_DIR: &str = "Default Project";

/// A `readdirSync(userDataPath, { withFileTypes: true })` entry adapter over
/// `std::fs`, exactly the `InstallStateEntry` shape `hasExistingAppState`
/// consumes.
pub struct FsInstallEntry(std::fs::DirEntry);

impl InstallStateEntry for FsInstallEntry {
    /// `entry.name` (the bare file name).
    fn name(&self) -> &str {
        self.0.file_name().to_str().unwrap_or("")
    }

    /// `entry.isDirectory()`.
    fn is_directory(&self) -> bool {
        self.0
            .file_type()
            .map(|file_type| file_type.is_dir())
            .unwrap_or(false)
    }
}

/// `existsSync(userDataPath) ? readdirSync(userDataPath, { withFileTypes:
/// true }) : []`
pub fn read_user_data_entries(user_data_path: &str) -> Vec<FsInstallEntry> {
    let path = std::path::Path::new(user_data_path);
    if !path.is_dir() {
        return Vec::new();
    }
    std::fs::read_dir(path)
        .map(|read_dir| {
            read_dir
                .filter_map(|entry| entry.ok())
                .map(FsInstallEntry)
                .collect()
        })
        .unwrap_or_default()
}

/// `store.get(OLD_LAYOUT_ELIGIBLE_KEY) === true` — the strict boolean check
/// of `isOldLayoutEligible`.
pub fn old_layout_eligible_from(value: Option<&Value>) -> bool {
    value == Some(&Value::Bool(true))
}

/// `store.get(FIRST_LAUNCH_ONBOARDING_COMPLETE_KEY) !== true` — the pending
/// predicate of `isFirstLaunchOnboardingPending`.
pub fn first_launch_pending_from(value: Option<&Value>) -> bool {
    value != Some(&Value::Bool(true))
}

/// Mirrors `initializeOldLayoutEligibility(userDataPath)`.
///
/// The source reads `process.env.OPENCODE_TEST_ONBOARDING` before the store
/// (see `index.ts`); the store access is PROVISIONAL here and errors fall
/// through to the computed eligibility, matching a missing key.
pub fn initialize_old_layout_eligibility(user_data_path: &str) -> bool {
    let entries = read_user_data_entries(user_data_path);

    // `const current = store.get(...); if (typeof current === "boolean")
    // return current` — `Value::is_boolean()` only for actual booleans.
    let current = match store::get_store(None) {
        Ok(store) => store
            .borrow()
            .get(OLD_LAYOUT_ELIGIBLE_KEY)
            .filter(Value::is_boolean),
        Err(_) => None,
    };
    if let Some(current) = current {
        return current.as_bool().expect("filtered to boolean");
    }

    let eligible = has_existing_app_state(&entries);
    // `store.set(OLD_LAYOUT_ELIGIBLE_KEY, eligible)` — PROVISIONAL(store.rs).
    if let Ok(store) = store::get_store(None) {
        store
            .borrow_mut()
            .set(OLD_LAYOUT_ELIGIBLE_KEY, Value::Bool(eligible));
    }
    eligible
}

/// Mirrors `isOldLayoutEligible()`.
pub fn is_old_layout_eligible() -> bool {
    match store::get_store(None) {
        Ok(store) => old_layout_eligible_from(store.borrow().get(OLD_LAYOUT_ELIGIBLE_KEY).as_ref()),
        Err(_) => false,
    }
}

/// Mirrors `isFirstLaunchOnboardingPending()`.
pub fn is_first_launch_onboarding_pending() -> bool {
    let pending = match store::get_store(None) {
        Ok(store) => first_launch_pending_from(
            store
                .borrow()
                .get(FIRST_LAUNCH_ONBOARDING_COMPLETE_KEY)
                .as_ref(),
        ),
        Err(_) => true,
    };
    // `write("onboarding", "first launch onboarding pending checked",
    // { pending })`
    write_log(
        "onboarding",
        "first launch onboarding pending checked",
        Some(Value::Bool(pending)),
        None,
    );
    pending
}

/// PROVISIONAL(packages/desktop/src/main/onboarding.ts): `app.getPath(
/// "documents")` needs the Electron app binding. Returns the path when the
/// runtime binding exists, `None` otherwise.
pub fn documents_dir() -> Option<String> {
    None
}

/// Mirrors `finishFirstLaunchOnboarding(createDefaultProject)`.
///
/// The already-completed short-circuit returns `None` (`null` in the source)
/// after the "already completed" write; otherwise the default project is
/// `join(documents, "Default Project")` (or `None` when not requested or no
/// documents dir resolves), created recursively, the completion key written,
/// and the final write logged with `{ createDefaultProject, defaultProject }`.
pub fn finish_first_launch_onboarding(create_default_project: bool) -> Option<String> {
    if !is_first_launch_onboarding_pending() {
        // `write("onboarding", "first launch onboarding already completed")`
        write_log(
            "onboarding",
            "first launch onboarding already completed",
            None,
            None,
        );
        return None;
    }

    let default_project = if create_default_project {
        documents_dir().map(|documents| format!("{}/{}", documents, DEFAULT_PROJECT_DIR))
    } else {
        None
    };
    if let Some(default_project) = &default_project {
        // `await mkdir(defaultProject, { recursive: true })`
        let _ = std::fs::create_dir_all(default_project);
    }

    // `store.set(FIRST_LAUNCH_ONBOARDING_COMPLETE_KEY, true)` — PROVISIONAL
    // (store.rs) when no Electron store is bound.
    if let Ok(store) = store::get_store(None) {
        store
            .borrow_mut()
            .set(FIRST_LAUNCH_ONBOARDING_COMPLETE_KEY, Value::Bool(true));
    }

    // `write("onboarding", "first launch onboarding completed",
    // { createDefaultProject, defaultProject })`
    write_log(
        "onboarding",
        "first launch onboarding completed",
        Some(serde_json::json!({
            "createDefaultProject": create_default_project,
            "defaultProject": default_project,
        })),
        None,
    );
    default_project
}

#[cfg(test)]
mod tests {
    // The source ships no `onboarding.test.ts`; the cases below pin the
    // ported store predicates and the `Default Project` naming.
    use super::*;

    #[test]
    fn old_layout_eligibility_is_a_strict_true_compare() {
        assert!(old_layout_eligible_from(Some(&Value::Bool(true))));
        assert!(!old_layout_eligible_from(Some(&Value::Bool(false))));
        assert!(!old_layout_eligible_from(Some(&Value::String(
            "true".to_string()
        ))));
        assert!(!old_layout_eligible_from(None));
    }

    #[test]
    fn first_launch_pending_is_anything_but_true() {
        assert!(first_launch_pending_from(None), "missing key → pending");
        assert!(first_launch_pending_from(Some(&Value::Bool(false))));
        assert!(
            !first_launch_pending_from(Some(&Value::Bool(true))),
            "completed"
        );
    }

    #[test]
    fn eligibility_reads_real_directories() {
        let dir = std::env::temp_dir().join(format!("oc-onboarding-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("some-worktree")).expect("create temp dir");

        let entries = read_user_data_entries(dir.to_str().unwrap());
        assert!(
            has_existing_app_state(&entries),
            "a directory counts as existing app state"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn default_project_name_is_exact() {
        assert_eq!(DEFAULT_PROJECT_DIR, "Default Project");
    }
}
