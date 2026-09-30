//! Rust port of `src/clipboard.ts` (opencode v1.18.30).
//!
//! Clipboard access shells out to platform tools, exactly like the source:
//! image reads via `osascript` (macOS), PowerShell (Windows/WSL),
//! `wl-paste`/`xclip` (Linux), with a text-tool fallback chain standing in
//! for `clipboardy`; writes go through OSC 52 plus the native `copyCommand`
//! tool. `Buffer.toString("base64")` becomes [`base64_encode`]; `which`
//! (from `@opencode-ai/core/util/which`) is a `PATH` search here.
//!
//! Original file: `packages/tui/src/clipboard.ts`

use std::io::{IsTerminal, Write as _};

pub const MIME_IMAGE_PNG: &str = "image/png";
pub const MIME_TEXT_PLAIN: &str = "text/plain";

/// What [`read_clipboard`] returns: base64 PNG bytes or plain text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardRead {
    pub data: String,
    pub mime: String,
}

const BASE64_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Standard base64 with padding (mirrors `Buffer.toString("base64")`).
pub fn base64_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let word = (chunk[0] as u32) << 16
            | (chunk.get(1).copied().unwrap_or(0) as u32) << 8
            | (chunk.get(2).copied().unwrap_or(0) as u32);
        out.push(BASE64_ALPHABET[((word >> 18) & 63) as usize] as char);
        out.push(BASE64_ALPHABET[((word >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 {
            BASE64_ALPHABET[((word >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            BASE64_ALPHABET[(word & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}

/// `command`: spawn `program`, feed optional stdin, collect stdout; any
/// nonzero exit becomes `` `{program} exited with code {code}` ``.
pub fn run_command(
    program: &str,
    args: &[String],
    input: Option<&[u8]>,
) -> Result<Vec<u8>, String> {
    let mut child = std::process::Command::new(program)
        .args(args)
        .stdin(if input.is_some() {
            std::process::Stdio::piped()
        } else {
            std::process::Stdio::null()
        })
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|error| error.to_string())?;
    if let (Some(mut stdin), Some(input)) = (child.stdin.take(), input) {
        use std::io::Write as _;
        let _ = stdin.write_all(input);
    }
    let output = child
        .wait_with_output()
        .map_err(|error| error.to_string())?;
    if output.status.success() {
        return Ok(output.stdout);
    }
    Err(format!(
        "{program} exited with code {}",
        output.status.code().unwrap_or(-1)
    ))
}

/// The OSC 52 payload `writeOsc52` emits for `text`.
pub fn osc52_sequence(text: &str) -> String {
    format!("\x1b]52;c;{}\x07", base64_encode(text.as_bytes()))
}

/// Where `writeOsc52` sends the sequence: skipped without a TTY, wrapped for
/// tmux passthrough under `$TMUX`/`$STY`, plain otherwise.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Osc52Write {
    Skip,
    Sequence(String),
    Passthrough(String),
    Both {
        sequence: String,
        passthrough: String,
    },
}

pub fn osc52_write_for(is_tty: bool, tmux: bool, sty: bool, text: &str) -> Osc52Write {
    if !is_tty {
        return Osc52Write::Skip;
    }
    let sequence = osc52_sequence(text);
    if tmux {
        let passthrough = format!("\x1bPtmux;\x1b{sequence}\x1b\\");
        return Osc52Write::Both {
            sequence,
            passthrough,
        };
    }
    if sty {
        return Osc52Write::Passthrough(format!("\x1bPtmux;\x1b{sequence}\x1b\\"));
    }
    Osc52Write::Sequence(sequence)
}

/// Emit the OSC 52 sequence for `text` following the TTY/TMUX/STY rules.
pub fn write_osc52(text: &str) {
    let is_tty = std::io::stdout().is_terminal();
    let tmux = std::env::var("TMUX").is_ok();
    let sty = std::env::var("STY").is_ok();
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    match osc52_write_for(is_tty, tmux, sty, text) {
        Osc52Write::Skip => {}
        Osc52Write::Sequence(sequence) => {
            let _ = handle.write_all(sequence.as_bytes());
        }
        Osc52Write::Passthrough(passthrough) => {
            let _ = handle.write_all(passthrough.as_bytes());
        }
        Osc52Write::Both {
            sequence,
            passthrough,
        } => {
            let _ = handle.write_all(sequence.as_bytes());
            let _ = handle.write_all(passthrough.as_bytes());
        }
    }
}

/// Node `platform()` in the source's vocabulary.
pub fn node_platform() -> &'static str {
    match std::env::consts::OS {
        "macos" => "darwin",
        "windows" => "win32",
        _ => std::env::consts::OS,
    }
}

/// Whether the Linux kernel reports WSL (`release().includes("WSL")`).
pub fn is_wsl() -> bool {
    if std::env::consts::OS != "linux" {
        return false;
    }
    std::fs::read_to_string("/proc/version")
        .map(|version| version.contains("WSL"))
        .unwrap_or(false)
}

/// `which` (mirrors `@opencode-ai/core/util/which`): first executable `name`
/// on `PATH`, if any.
pub fn which(name: &str) -> Option<String> {
    let path = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path) {
        if dir.as_os_str().is_empty() {
            continue;
        }
        let candidate = dir.join(name);
        if is_executable(&candidate) {
            return Some(candidate.to_string_lossy().into_owned());
        }
    }
    None
}

fn is_executable(path: &std::path::Path) -> bool {
    let Ok(metadata) = std::fs::metadata(path) else {
        return false;
    };
    if !metadata.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        metadata.permissions().mode() & 0o111 != 0
    }
    #[cfg(not(unix))]
    {
        true
    }
}

/// Mirrors `copyCommand`: the native copy tool for `os`
/// (`"darwin"`/`"linux"`/`"win32"`), or `None` when unavailable.
pub fn copy_command(os: &str, wayland: bool, has: &dyn Fn(&str) -> bool) -> Option<Vec<String>> {
    if os == "darwin" && has("osascript") {
        return Some(vec!["osascript".to_string()]);
    }
    if os == "linux" && wayland && has("wl-copy") {
        return Some(vec!["wl-copy".to_string()]);
    }
    if os == "linux" && has("xclip") {
        return Some(vec![
            "xclip".to_string(),
            "-selection".to_string(),
            "clipboard".to_string(),
        ]);
    }
    if os == "linux" && has("xsel") {
        return Some(vec![
            "xsel".to_string(),
            "--clipboard".to_string(),
            "--input".to_string(),
        ]);
    }
    if os == "win32" && has("powershell.exe") {
        return Some(vec![
            "powershell.exe".to_string(),
            "-NonInteractive".to_string(),
            "-NoProfile".to_string(),
            "-Command".to_string(),
            "[Console]::InputEncoding = [System.Text.Encoding]::UTF8; Set-Clipboard -Value ([Console]::In.ReadToEnd())"
                .to_string(),
        ]);
    }
    None
}

const POWERSHELL_IMAGE_SCRIPT: &str = "Add-Type -AssemblyName System.Windows.Forms; $img = [System.Windows.Forms.Clipboard]::GetImage(); if ($img) { $ms = New-Object System.IO.MemoryStream; $img.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png); [System.Convert]::ToBase64String($ms.ToArray()) }";

fn osascript_args(script: &str) -> Vec<String> {
    vec!["-e".to_string(), script.to_string()]
}

/// The `clipboardy.read()` fallback: platform text tools, first hit wins.
fn read_text_fallback() -> Option<ClipboardRead> {
    let text = match node_platform() {
        "darwin" => run_command("pbpaste", &[], None).ok(),
        "win32" => run_command(
            "powershell.exe",
            &[
                "-NonInteractive".to_string(),
                "-NoProfile".to_string(),
                "-Command".to_string(),
                "Get-Clipboard".to_string(),
            ],
            None,
        )
        .ok(),
        _ => run_command("wl-paste", &["--no-newline".to_string()], None)
            .or_else(|_| {
                run_command(
                    "xclip",
                    &[
                        "-selection".to_string(),
                        "clipboard".to_string(),
                        "-o".to_string(),
                    ],
                    None,
                )
            })
            .or_else(|_| {
                run_command(
                    "xsel",
                    &["--clipboard".to_string(), "--output".to_string()],
                    None,
                )
            })
            .ok(),
    }?;
    if text.is_empty() {
        return None;
    }
    Some(ClipboardRead {
        data: String::from_utf8_lossy(&text).into_owned(),
        mime: MIME_TEXT_PLAIN.to_string(),
    })
}

/// Mirrors `read`: image first per platform, then the text fallback.
/// Returns `None` when nothing is readable.
pub fn read() -> Option<ClipboardRead> {
    let platform = node_platform();
    if platform == "darwin" {
        let file = std::env::temp_dir().join("opencode-clipboard.png");
        let name = file.to_string_lossy().into_owned();
        let image = (|| {
            run_command(
                "osascript",
                &[
                    "-e".to_string(),
                    "set imageData to the clipboard as \"PNGf\"".to_string(),
                    "-e".to_string(),
                    format!("set fileRef to open for access POSIX file \"{name}\" with write permission"),
                    "-e".to_string(),
                    "set eof fileRef to 0".to_string(),
                    "-e".to_string(),
                    "write imageData to fileRef".to_string(),
                    "-e".to_string(),
                    "close access fileRef".to_string(),
                ],
                None,
            )
            .ok()?;
            std::fs::read(&file).ok()
        })();
        let _ = std::fs::remove_file(&file);
        if let Some(bytes) = image {
            return Some(ClipboardRead {
                data: base64_encode(&bytes),
                mime: MIME_IMAGE_PNG.to_string(),
            });
        }
    }

    if platform == "win32" || is_wsl() {
        let args: Vec<String> = vec![
            "-NonInteractive".to_string(),
            "-NoProfile".to_string(),
            "-command".to_string(),
            POWERSHELL_IMAGE_SCRIPT.to_string(),
        ];
        if let Ok(image) = run_command("powershell.exe", &args, None) {
            let text = String::from_utf8_lossy(&image).trim().to_string();
            if !text.is_empty() {
                return Some(ClipboardRead {
                    data: text,
                    mime: MIME_IMAGE_PNG.to_string(),
                });
            }
        }
    }

    if platform == "linux" {
        if let Ok(png) = run_command(
            "wl-paste",
            &["-t".to_string(), "image/png".to_string()],
            None,
        ) {
            if !png.is_empty() {
                return Some(ClipboardRead {
                    data: base64_encode(&png),
                    mime: MIME_IMAGE_PNG.to_string(),
                });
            }
        }
        if let Ok(png) = run_command(
            "xclip",
            &[
                "-selection".to_string(),
                "clipboard".to_string(),
                "-t".to_string(),
                "image/png".to_string(),
                "-o".to_string(),
            ],
            None,
        ) {
            if !png.is_empty() {
                return Some(ClipboardRead {
                    data: base64_encode(&png),
                    mime: MIME_IMAGE_PNG.to_string(),
                });
            }
        }
    }

    read_text_fallback()
}

/// Escape text for the `osascript` clipboard setter.
pub fn escape_osascript(text: &str) -> String {
    text.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Mirrors `write`: OSC 52 first, then the native copy tool (or nothing when
/// no tool exists — the clipboardy fallback has no dependency-free
/// equivalent, and OSC 52 already covered terminals).
pub fn write(text: &str) {
    write_osc52(text);
    let os = node_platform();
    let wayland = std::env::var("WAYLAND_DISPLAY").is_ok();
    let method = copy_command(os, wayland, &|name| which(name).is_some());
    let Some(method) = method else {
        return;
    };
    if method.len() == 1 && method[0] == "osascript" {
        let _ = run_command(
            "osascript",
            &osascript_args(&format!(
                "set the clipboard to \"{}\"",
                escape_osascript(text)
            )),
            None,
        );
        return;
    }
    let _ = run_command(&method[0], &method[1..], Some(text.as_bytes()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_matches_known_vectors() {
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
        assert_eq!(base64_encode(b"foob"), "Zm9vYg==");
        assert_eq!(base64_encode(b"opencode"), "b3BlbmNvZGU=");
    }

    #[test]
    fn osc52_wraps_base64_with_terminators() {
        assert_eq!(osc52_sequence("hi"), "\x1b]52;c;aGk=\x07");
        assert_eq!(osc52_write_for(false, true, false, "hi"), Osc52Write::Skip);
        assert_eq!(
            osc52_write_for(true, false, false, "hi"),
            Osc52Write::Sequence("\x1b]52;c;aGk=\x07".to_string())
        );
        // tmux gets both the raw and the passthrough-wrapped sequence.
        match osc52_write_for(true, true, false, "hi") {
            Osc52Write::Both {
                sequence,
                passthrough,
            } => {
                assert_eq!(sequence, "\x1b]52;c;aGk=\x07");
                assert_eq!(passthrough, "\x1bPtmux;\x1b\x1b]52;c;aGk=\x07\x1b\\");
            }
            other => panic!("expected Both, got {other:?}"),
        }
        match osc52_write_for(true, false, true, "hi") {
            Osc52Write::Passthrough(passthrough) => {
                assert_eq!(passthrough, "\x1bPtmux;\x1b\x1b]52;c;aGk=\x07\x1b\\");
            }
            other => panic!("expected Passthrough, got {other:?}"),
        }
    }

    #[test]
    fn copy_command_matches_source_selection() {
        assert_eq!(
            copy_command("linux", true, &|name| name == "wl-copy"),
            Some(vec!["wl-copy".to_string()])
        );
        assert_eq!(
            copy_command("darwin", false, &|name| name == "osascript"),
            Some(vec!["osascript".to_string()])
        );
        assert_eq!(
            copy_command("linux", true, &|name| name == "xclip"),
            Some(vec![
                "xclip".to_string(),
                "-selection".to_string(),
                "clipboard".to_string()
            ])
        );
        assert_eq!(copy_command("linux", false, &|_name| false), None);
    }

    #[test]
    fn osascript_escaping_matches_source() {
        assert_eq!(escape_osascript("a\\b\"c"), "a\\\\b\\\"c");
    }

    #[test]
    fn failing_commands_report_the_exit_code() {
        let error = run_command("false", &[], None).expect_err("false exits nonzero");
        assert!(error.contains("exited with code"), "{error}");
    }
}
