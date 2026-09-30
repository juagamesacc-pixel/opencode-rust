//! Port of `test/clipboard.test.ts` (opencode v1.18.30).
//!
//! `copyCommand` selection, case for case.
use tui::clipboard::copy_command;

#[test]
fn prefers_wayland_clipboard_when_available() {
    assert_eq!(
        copy_command("linux", true, &|name| name == "wl-copy"),
        Some(vec!["wl-copy".to_string()])
    );
}

#[test]
fn uses_osascript_on_macos() {
    assert_eq!(
        copy_command("darwin", false, &|name| name == "osascript"),
        Some(vec!["osascript".to_string()])
    );
}

#[test]
fn falls_back_through_x11_clipboard_commands() {
    assert_eq!(
        copy_command("linux", true, &|name| name == "xclip"),
        Some(vec![
            "xclip".to_string(),
            "-selection".to_string(),
            "clipboard".to_string()
        ])
    );
    assert_eq!(
        copy_command("linux", false, &|name| name == "xsel"),
        Some(vec![
            "xsel".to_string(),
            "--clipboard".to_string(),
            "--input".to_string()
        ])
    );
}

#[test]
fn returns_none_when_native_clipboard_is_unavailable() {
    assert_eq!(copy_command("linux", false, &|_| false), None);
}
