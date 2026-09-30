//! Rust port of `src/audio.ts` (opencode v1.18.30).
//!
//! Lazy singleton audio backend with a file cache. The source drives
//! `@opentui/core`'s `Audio` mixer; without that native binding the port
//! plays sound files through the platform player (`afplay` on macOS, the
//! first available of `paplay`/`aplay`/`play`/`ffplay` on Linux, a
//! PowerShell `SoundPlayer` one-liner on Windows), spawned detached per
//! voice. Lazy-init failure degrades to `None` exactly like the source's
//! `audio = null` path; `dispose` clears the cache and stops all voices.
//!
//! Original file: `packages/tui/src/audio.ts`

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Playback options. Mirrors the `AudioPlayOptions` fields the TUI passes
/// (`play(sound, { volume })`); the native type carries more, none of which
/// the TUI uses.
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioPlayOptions {
    pub volume: Option<f64>,
}

/// A loaded sound: the file it was read from plus its bytes.
#[derive(Debug, Clone)]
pub struct AudioSound {
    pub file: String,
    pub bytes: Vec<u8>,
}

/// A playing voice, identified so [`stop_voice`] can silence it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AudioVoice(pub u64);

struct Backend {
    sounds: HashMap<String, AudioSound>,
    voices: HashMap<u64, std::process::Child>,
    next_voice: u64,
    started: bool,
}

impl Backend {
    fn new() -> Self {
        Self {
            sounds: HashMap::new(),
            voices: HashMap::new(),
            next_voice: 1,
            started: false,
        }
    }
}

static BACKEND: OnceLock<Mutex<Backend>> = OnceLock::new();

fn backend() -> &'static Mutex<Backend> {
    BACKEND.get_or_init(|| Mutex::new(Backend::new()))
}

/// `which` without new dependencies: search `PATH` for an executable file.
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
        #[cfg(windows)]
        for extension in ["exe", "bat", "cmd", "ps1"] {
            let with_extension = candidate.with_extension(extension);
            if is_executable(&with_extension) {
                return Some(with_extension.to_string_lossy().into_owned());
            }
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

/// The platform player command for `file`, if one is installed.
///
/// Volume is intentionally not forwarded: the candidate players disagree on
/// flag shapes, and the TUI already clamps volume at the attention layer.
pub fn player_command(os: &str, file: &str) -> Option<(String, Vec<String>)> {
    match os {
        "darwin" => which("afplay").map(|player| (player, vec![file.to_string()])),
        "win32" => which("powershell.exe").map(|player| {
            (
                player,
                vec![
                    "-NonInteractive".to_string(),
                    "-NoProfile".to_string(),
                    "-Command".to_string(),
                    format!(
                        "$player = New-Object System.Media.SoundPlayer \"{}\"; $player.PlaySync()",
                        file.replace('"', "\"\"")
                    ),
                ],
            )
        }),
        _ => ["paplay", "aplay", "play", "ffplay"]
            .iter()
            .find_map(|name| which(name))
            .map(|player| {
                let mut args = Vec::new();
                if player.ends_with("ffplay") {
                    args.push("-nodisp".to_string());
                    args.push("-autoexit".to_string());
                    args.push("-loglevel".to_string());
                    args.push("quiet".to_string());
                }
                args.push(file.to_string());
                (player, args)
            }),
    }
}

fn node_platform() -> &'static str {
    match std::env::consts::OS {
        "macos" => "darwin",
        "windows" => "win32",
        _ => std::env::consts::OS,
    }
}

/// `getAudio` + `start`: mark the backend started when a player exists.
/// Returns `false` (and leaves the backend stopped) when no player is
/// installed, matching the source's `!current.start()` early return.
fn ensure_started(backend: &mut Backend) -> bool {
    if backend.started {
        return true;
    }
    // Probe only; the per-play lookup re-resolves so `PATH` changes apply.
    let probe = match node_platform() {
        "darwin" => which("afplay").is_some(),
        "win32" => which("powershell.exe").is_some(),
        _ => ["paplay", "aplay", "play", "ffplay"]
            .iter()
            .any(|name| which(name).is_some()),
    };
    backend.started = probe;
    probe
}

/// Mirrors `loadSoundFile`: read `file` once, cache the bytes, return `None`
/// (after a debug log, like the source) when the file cannot be read.
pub fn load_sound_file(file: &str) -> Option<AudioSound> {
    let mut backend = backend().lock().ok()?;
    if let Some(cached) = backend.sounds.get(file) {
        return Some(cached.clone());
    }
    match std::fs::read(file) {
        Ok(bytes) => {
            let sound = AudioSound {
                file: file.to_string(),
                bytes,
            };
            backend.sounds.insert(file.to_string(), sound.clone());
            Some(sound)
        }
        Err(error) => {
            eprintln!("failed to load tui sound {file}: {error}");
            None
        }
    }
}

/// Mirrors `play`: start the backend on first use, then spawn the platform
/// player for `sound`. Returns `None` when no player exists or the spawn
/// fails.
pub fn play(sound: &AudioSound, options: Option<AudioPlayOptions>) -> Option<AudioVoice> {
    let _ = options;
    let mut backend = backend().lock().ok()?;
    if !ensure_started(&mut backend) {
        return None;
    }
    // Write through a temp file only when the sound did not come from one:
    // `sound.file` is already on disk (the loader only reads files), so the
    // player can open it directly.
    let (program, args) = player_command(node_platform(), &sound.file)?;
    let child = std::process::Command::new(program)
        .args(args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .ok()?;
    let voice = AudioVoice(backend.next_voice);
    backend.next_voice += 1;
    backend.voices.insert(voice.0, child);
    Some(voice)
}

/// Mirrors `stopVoice`: silence a tracked voice, `false` for unknown ones.
pub fn stop_voice(voice: AudioVoice) -> bool {
    let Ok(mut backend) = backend().lock() else {
        return false;
    };
    let Some(mut child) = backend.voices.remove(&voice.0) else {
        return false;
    };
    let _ = child.kill();
    let _ = child.wait();
    true
}

/// Mirrors `dispose`: stop every voice and drop the sound cache.
pub fn dispose() {
    let Ok(mut backend) = backend().lock() else {
        return;
    };
    for (_, mut child) in backend.voices.drain() {
        let _ = child.kill();
        let _ = child.wait();
    }
    backend.sounds.clear();
    backend.started = false;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_sound(contents: &[u8]) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "tui-audio-test-{}-{}.mp3",
            std::process::id(),
            contents.len()
        ));
        std::fs::write(&path, contents).expect("write temp sound");
        path
    }

    #[test]
    fn missing_files_load_to_none() {
        assert!(load_sound_file("/definitely/not/a/tui/sound.mp3").is_none());
    }

    #[test]
    fn loaded_sounds_are_cached() {
        let path = temp_sound(b"ID3fake");
        let name = path.to_string_lossy().into_owned();
        let first = load_sound_file(&name).expect("load temp sound");
        assert_eq!(first.bytes, b"ID3fake");
        let second = load_sound_file(&name).expect("cached temp sound");
        assert_eq!(second.bytes, first.bytes);
        dispose();
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn stopping_an_unknown_voice_is_false() {
        assert!(!stop_voice(AudioVoice(u64::MAX)));
    }

    #[test]
    fn player_commands_follow_the_platform() {
        // macOS always shells to afplay when present.
        let darwin = player_command("darwin", "x.mp3");
        if which("afplay").is_some() {
            let (program, args) = darwin.expect("afplay command");
            assert!(program.ends_with("afplay"));
            assert_eq!(args, vec!["x.mp3".to_string()]);
        } else {
            assert!(darwin.is_none());
        }
        // Unknown platforms fall through to the Linux candidate list.
        let other = player_command("freebsd", "x.mp3");
        let any_player = ["paplay", "aplay", "play", "ffplay"]
            .iter()
            .any(|name| which(name).is_some());
        assert_eq!(other.is_some(), any_player);
    }
}
