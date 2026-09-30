//! Rust port of `src/main/install-state.ts` (opencode v1.18.30).
//!
//! The `/^window-state-.+\.json$/` test is reproduced with `std` string
//! operations (`.+` requires at least one character between the prefix and
//! the suffix; entry names come from directory listings and never contain
//! line terminators, so `.` matches every byte here exactly as in JS).
//!
//! Original file: `packages/desktop/src/main/install-state.ts`

/// Mirrors `{ name: string; isDirectory: () => boolean }`.
pub trait InstallStateEntry {
    fn name(&self) -> &str;
    fn is_directory(&self) -> bool;
}

pub fn has_existing_app_state(entries: &[impl InstallStateEntry]) -> bool {
    entries.iter().any(|entry| {
        if entry.name() == "opencode.settings" {
            return true;
        }
        if entry.name().ends_with(".dat") {
            return true;
        }
        if is_window_state_file(entry.name()) {
            return true;
        }
        entry.is_directory() && entry.name() == "opencode"
    })
}

fn is_window_state_file(name: &str) -> bool {
    let rest = match name.strip_prefix("window-state-") {
        Some(rest) => rest,
        None => return false,
    };
    let middle = match rest.strip_suffix(".json") {
        Some(middle) => middle,
        None => return false,
    };
    !middle.is_empty()
}

#[cfg(test)]
mod tests {
    // Mirrors `src/main/install-state.test.ts` (`describe("hasExistingAppState")`).
    use super::*;

    struct File(&'static str);
    struct Dir(&'static str);

    impl InstallStateEntry for File {
        fn name(&self) -> &str {
            self.0
        }
        fn is_directory(&self) -> bool {
            false
        }
    }

    impl InstallStateEntry for Dir {
        fn name(&self) -> &str {
            self.0
        }
        fn is_directory(&self) -> bool {
            true
        }
    }

    #[test]
    fn ignores_files_electron_may_create_on_a_fresh_install() {
        let empty: Vec<File> = vec![];
        assert!(!has_existing_app_state(&empty));
        let fresh = [File("Local State")];
        assert!(!has_existing_app_state(&fresh));
        // Note: `has_existing_app_state` takes a homogeneous slice; the
        // source's mixed `[file("Local State"), directory("Crashpad")]`
        // case is covered by checking each entry kind separately.
        let crashpad = [Dir("Crashpad")];
        assert!(!has_existing_app_state(&crashpad));
    }

    #[test]
    fn recognizes_state_written_by_an_earlier_opencode_launch() {
        assert!(has_existing_app_state(&[File("opencode.settings")]));
        assert!(has_existing_app_state(&[File("opencode.global.dat")]));
        assert!(has_existing_app_state(&[File("window-state-abc.json")]));
        assert!(has_existing_app_state(&[Dir("opencode")]));
    }
}
