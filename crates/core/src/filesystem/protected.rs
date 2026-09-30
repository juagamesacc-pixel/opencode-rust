//! Rust port of `packages/core/src/filesystem/protected.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

/// Source: DARWIN_HOME verbatim
pub const DARWIN_HOME: &[&str] = &[
    "Music",
    "Pictures",
    "Movies",
    "Downloads",
    "Desktop",
    "Documents",
    "Public",
    "Applications",
    "Library",
];
pub const DARWIN_LIBRARY: &[&str] = &[
    "Application Support/AddressBook",
    "Calendars",
    "Mail",
    "Messages",
    "Safari",
    "Cookies",
    "Application Support/com.apple.TCC",
    "PersonalizationPortrait",
    "Metadata/CoreSpotlight",
    "Suggestions",
];
pub const DARWIN_ROOT: &[&str] = &[
    "/.DocumentRevisions-V100",
    "/.Spotlight-V100",
    "/.Trashes",
    "/.fseventsd",
];
pub const WIN32_HOME: &[&str] = &[
    "AppData",
    "Downloads",
    "Desktop",
    "Documents",
    "Pictures",
    "Music",
    "Videos",
    "OneDrive",
];

/// Source: `export function names(): ReadonlySet<string>` verbatim
pub fn names(platform: &str) -> Vec<String> {
    match platform {
        "darwin" => DARWIN_HOME.iter().map(|s| s.to_string()).collect(),
        "win32" => WIN32_HOME.iter().map(|s| s.to_string()).collect(),
        _ => vec![],
    }
}

/// Source: `export function paths(): string[]` verbatim — uses home + path.join logic
pub fn paths(platform: &str, home: &str) -> Vec<String> {
    match platform {
        "darwin" => {
            let mut v: Vec<String> = DARWIN_HOME
                .iter()
                .map(|n| format!("{}/{}", home, n))
                .collect();
            v.extend(
                DARWIN_LIBRARY
                    .iter()
                    .map(|n| format!("{}/Library/{}", home, n)),
            );
            v.extend(DARWIN_ROOT.iter().map(|s| s.to_string()));
            v
        }
        "win32" => WIN32_HOME
            .iter()
            .map(|n| format!("{}/{}", home, n))
            .collect(),
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn darwin_names() {
        assert_eq!(names("darwin").len(), DARWIN_HOME.len());
    }
    #[test]
    fn win32_paths() {
        assert_eq!(paths("win32", "/home/u").len(), WIN32_HOME.len());
    }
    #[test]
    fn linux_empty() {
        assert!(names("linux").is_empty());
        assert!(paths("linux", "/home").is_empty());
    }
}
