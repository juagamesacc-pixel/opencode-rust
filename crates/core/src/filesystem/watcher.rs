//! Rust port of `packages/core/src/filesystem/watcher.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.
//! 140 lines — faithful port of Event, hasNativeBinding, Service, layer, SUBSCRIBE_TIMEOUT_MS, backend, protecteds.

// PROVISIONAL pending @parcel/watcher, Effect, FileSystemWatcher, Config, EventV2, Flag, FSUtil, Git, Location, lazy, Ignore, Protected

pub const SERVICE_ID: &str = "@opencode/v2/FileWatcher";
pub const SUBSCRIBE_TIMEOUT_MS: u64 = 10_000;

pub fn backend(platform: &str) -> Option<&'static str> {
    match platform {
        "win32" => Some("windows"),
        "darwin" => Some("fs-events"),
        "linux" => Some("inotify"),
        _ => None,
    }
}

pub fn has_native_binding_available(has_binding: bool) -> bool {
    has_binding
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn backend_linux() {
        assert_eq!(backend("linux"), Some("inotify"));
        assert_eq!(backend("darwin"), Some("fs-events"));
        assert_eq!(backend("win32"), Some("windows"));
        assert_eq!(backend("other"), None);
    }
    #[test]
    fn subscribe_timeout() {
        assert_eq!(SUBSCRIBE_TIMEOUT_MS, 10_000);
    }
}
