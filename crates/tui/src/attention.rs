//! Rust port of `src/attention.ts` (opencode v1.18.30).
//!
//! Attention notification + sound logic. The source drives `@opentui/core`'s
//! renderer for focus/blur events and notifications; the port uses a trait
//! object for the renderer and the ported `audio` module for sound playback.
//!
//! Original file: `packages/tui/src/attention.ts`

use std::collections::HashMap;

use super::audio::{self, AudioPlayOptions};
use super::config::ResolvedAttention;

const DEFAULT_TITLE: &str = "opencode";
const DEFAULT_PACK_ID: &str = "opencode.default";
const KV_SOUND_PACK: &str = "attention_sound_pack";
const TITLE_LIMIT: usize = 80;
const MESSAGE_LIMIT: usize = 240;

/// Focus state of the terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusState {
    Unknown,
    Focused,
    Blurred,
}

/// The renderer interface attention needs (mirrors `AttentionRenderer`).
pub trait AttentionRenderer {
    fn is_destroyed(&self) -> bool;
    fn on_focus(&self, listener: Box<dyn Fn() + Send>);
    fn off_focus(&self, listener: Box<dyn Fn() + Send>);
    fn on_blur(&self, listener: Box<dyn Fn() + Send>);
    fn off_blur(&self, listener: Box<dyn Fn() + Send>);
    fn trigger_notification(&self, message: &str, title: &str) -> bool;
}

/// A registered sound pack.
#[derive(Debug, Clone)]
pub struct RegisteredSoundPack {
    pub id: String,
    pub name: Option<String>,
    pub builtin: bool,
    pub sounds: HashMap<String, String>,
}

/// The builtin pack with default sound paths.
pub fn builtin_pack() -> RegisteredSoundPack {
    RegisteredSoundPack {
        id: DEFAULT_PACK_ID.to_string(),
        name: Some("OpenCode Default".to_string()),
        builtin: true,
        sounds: HashMap::from([
            (
                "default".to_string(),
                "@opencode-ai/ui/audio/bip-bop-01.mp3".to_string(),
            ),
            (
                "question".to_string(),
                "@opencode-ai/ui/audio/bip-bop-03.mp3".to_string(),
            ),
            (
                "permission".to_string(),
                "@opencode-ai/ui/audio/staplebops-06.mp3".to_string(),
            ),
            (
                "error".to_string(),
                "@opencode-ai/ui/audio/nope-03.mp3".to_string(),
            ),
            (
                "done".to_string(),
                "@opencode-ai/ui/audio/bip-bop-01.mp3".to_string(),
            ),
            (
                "subagent_done".to_string(),
                "@opencode-ai/ui/audio/yup-01.mp3".to_string(),
            ),
        ]),
    }
}

/// Skip reason for a notify call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkipReason {
    AttentionDisabled,
    RendererDestroyed,
    EmptyMessage,
    FocusUnknown,
    Focused,
    Blurred,
}

/// Result of a notify call.
#[derive(Debug, Clone, PartialEq)]
pub struct NotifyResult {
    pub ok: bool,
    pub notification: bool,
    pub sound: bool,
    pub skipped: Option<SkipReason>,
}

/// Input for a notify call.
#[derive(Debug, Clone, Default)]
pub struct NotifyInput {
    pub message: String,
    pub title: Option<String>,
    pub notification: Option<NotificationRequest>,
    pub sound: Option<SoundRequest>,
}

/// Notification request: false, true, or options object.
#[derive(Debug, Clone, PartialEq)]
pub enum NotificationRequest {
    False,
    True,
    Options { when: AttentionWhen },
}

/// Sound request: false, true, or options object.
#[derive(Debug, Clone, PartialEq)]
pub enum SoundRequest {
    False,
    True,
    Options {
        when: AttentionWhen,
        name: Option<String>,
        volume: Option<f64>,
    },
}

/// When to trigger attention.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttentionWhen {
    Always,
    Focused,
    Blurred,
}

/// Sound pack info for listing.
#[derive(Debug, Clone, PartialEq)]
pub struct SoundPackInfo {
    pub id: String,
    pub name: Option<String>,
    pub active: bool,
    pub builtin: bool,
}

/// The attention host (mirrors `TuiAttentionHost`).
pub struct TuiAttentionHost {
    renderer: Box<dyn AttentionRenderer + Send>,
    config: ResolvedAttention,
    kv: Option<Box<dyn KvStore>>,
    focus: FocusState,
    disposed: bool,
    active_pack_id: Option<String>,
    packs: HashMap<String, (RegisteredSoundPack, u64)>,
    next_pack_instance: u64,
}

/// Unregister guard for a sound pack (mirrors the disposer `registerPack`
/// returns; removal only applies while the same registration is stored).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackUnregister {
    pub id: String,
    instance: u64,
}

/// KV store interface for persisting the active sound pack.
pub trait KvStore {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&self, key: &str, value: &str);
}

fn skipped(reason: SkipReason) -> NotifyResult {
    NotifyResult {
        ok: false,
        notification: false,
        sound: false,
        skipped: Some(reason),
    }
}

fn normalize_text(input: &str, fallback: &str, limit: usize) -> String {
    let stripped = strip_ansi(input);
    let cleaned: String = stripped
        .chars()
        .map(|c| {
            if c == '\n' || c == '\r' || c == '\t' {
                ' '
            } else {
                c
            }
        })
        .collect();
    let cleaned: String = cleaned
        .chars()
        .filter(|c| {
            let code = *c as u32;
            !(code <= 0x09
                || code == 0x0B
                || code == 0x0C
                || (0x0E..=0x1F).contains(&code)
                || (0x7F..=0x9F).contains(&code))
        })
        .collect();
    let trimmed = cleaned.trim();
    let normalized = if trimmed.is_empty() {
        fallback
    } else {
        trimmed
    };
    normalized.chars().take(limit).collect()
}

fn strip_ansi(input: &str) -> String {
    let mut out = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            for nc in chars.by_ref() {
                if nc == 'm' {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn clamp_volume(volume: f64) -> f64 {
    if !volume.is_finite() {
        return 0.0;
    }
    volume.clamp(0.0, 1.0)
}

fn sound_volume(input: &NotifyInput, config: &ResolvedAttention) -> Option<f64> {
    if !config.sound {
        return None;
    }
    match &input.sound {
        None => Some(clamp_volume(config.volume)),
        Some(SoundRequest::False) => None,
        Some(SoundRequest::True) => Some(clamp_volume(config.volume)),
        Some(SoundRequest::Options { volume, .. }) => {
            Some(clamp_volume(volume.unwrap_or(config.volume)))
        }
    }
}

fn normalize_pack(pack: &RegisteredSoundPack) -> Option<RegisteredSoundPack> {
    let id = pack.id.trim().to_string();
    if id.is_empty() {
        return None;
    }
    let mut sounds = HashMap::new();
    for (key, value) in &pack.sounds {
        if is_valid_sound_name(key) && !value.trim().is_empty() {
            sounds.insert(key.clone(), value.clone());
        }
    }
    Some(RegisteredSoundPack {
        id,
        name: pack
            .name
            .as_ref()
            .map(|n| n.trim().to_string())
            .filter(|n| !n.is_empty()),
        builtin: false,
        sounds,
    })
}

fn is_valid_sound_name(name: &str) -> bool {
    matches!(
        name,
        "default" | "question" | "permission" | "error" | "done" | "subagent_done"
    )
}

fn focus_skip(when: AttentionWhen, focus: FocusState) -> Option<SkipReason> {
    match when {
        AttentionWhen::Always => None,
        _ => match focus {
            FocusState::Unknown => Some(SkipReason::FocusUnknown),
            FocusState::Focused => {
                if when == AttentionWhen::Blurred {
                    Some(SkipReason::Focused)
                } else {
                    None
                }
            }
            FocusState::Blurred => {
                if when == AttentionWhen::Focused {
                    Some(SkipReason::Blurred)
                } else {
                    None
                }
            }
        },
    }
}

impl TuiAttentionHost {
    /// Create a new attention host.
    pub fn new(
        renderer: Box<dyn AttentionRenderer + Send>,
        config: ResolvedAttention,
        kv: Option<Box<dyn KvStore>>,
    ) -> Self {
        let mut packs = HashMap::new();
        packs.insert(DEFAULT_PACK_ID.to_string(), (builtin_pack(), 0));
        Self {
            renderer,
            config,
            kv,
            focus: FocusState::Unknown,
            disposed: false,
            active_pack_id: None,
            packs,
            next_pack_instance: 1,
        }
    }

    fn configured_pack_id(&self) -> String {
        self.active_pack_id
            .clone()
            .or_else(|| self.kv.as_ref().and_then(|kv| kv.get(KV_SOUND_PACK)))
            .unwrap_or_else(|| self.config.sound_pack.clone())
    }

    fn current_pack(&self) -> &RegisteredSoundPack {
        self.packs
            .get(&self.configured_pack_id())
            .map(|(pack, _)| pack)
            .unwrap_or_else(|| &self.packs.get(DEFAULT_PACK_ID).unwrap().0)
    }

    fn sound_candidates(&self, name: &str) -> Vec<String> {
        let mut seen = Vec::new();
        let mut out = Vec::new();
        for source in [
            self.config.sounds.get(name).cloned(),
            self.current_pack().sounds.get(name).cloned(),
            self.packs
                .get(DEFAULT_PACK_ID)
                .and_then(|(p, _)| p.sounds.get(name).cloned()),
        ] {
            if let Some(path) = source.filter(|path| !seen.contains(path)) {
                seen.push(path.clone());
                out.push(path);
            }
        }
        out
    }

    async fn play_sound(&self, name: &str, volume: f64) -> bool {
        for file in self.sound_candidates(name) {
            let sound = audio::load_sound_file(&file);
            if self.disposed {
                return false;
            }
            if let Some(s) = sound {
                let options = AudioPlayOptions {
                    volume: Some(volume),
                };
                if audio::play(&s, Some(options)).is_some() {
                    return true;
                }
            }
        }
        false
    }

    /// Notify (mirrors `notify`).
    pub async fn notify(&self, request: NotifyInput) -> NotifyResult {
        if !self.config.enabled {
            return skipped(SkipReason::AttentionDisabled);
        }
        if self.disposed || self.renderer.is_destroyed() {
            return skipped(SkipReason::RendererDestroyed);
        }

        let message = normalize_text(&request.message, "", MESSAGE_LIMIT);
        if message.is_empty() {
            return skipped(SkipReason::EmptyMessage);
        }

        let notification_when = request
            .notification
            .as_ref()
            .map(|n| match n {
                NotificationRequest::Options { when } => *when,
                _ => AttentionWhen::Blurred,
            })
            .unwrap_or(AttentionWhen::Blurred);
        let notification_skip = focus_skip(notification_when, self.focus);
        let notification_requested = self.config.notifications
            && !matches!(request.notification, Some(NotificationRequest::False));
        let should_notify = notification_requested && notification_skip.is_none();
        let notification = if should_notify {
            let title = normalize_text(
                request.title.as_deref().unwrap_or(""),
                DEFAULT_TITLE,
                TITLE_LIMIT,
            );
            self.renderer.trigger_notification(&message, &title)
        } else {
            false
        };

        let volume = sound_volume(&request, &self.config);
        let sound_when = request
            .sound
            .as_ref()
            .map(|s| match s {
                SoundRequest::Options { when, .. } => *when,
                _ => AttentionWhen::Always,
            })
            .unwrap_or(AttentionWhen::Always);
        let sound_skip = volume.and_then(|_| focus_skip(sound_when, self.focus));
        let sound_name = request
            .sound
            .as_ref()
            .and_then(|s| match s {
                SoundRequest::Options { name, .. } => name.as_deref(),
                _ => None,
            })
            .filter(|n| is_valid_sound_name(n))
            .unwrap_or("default");
        let sound = match (volume, &sound_skip) {
            (Some(v), None) => self.play_sound(sound_name, v).await,
            _ => false,
        };

        if !notification && !sound {
            if notification_requested {
                if let Some(reason) = notification_skip {
                    return skipped(reason);
                }
            }
            if let Some(reason) = sound_skip {
                return skipped(reason);
            }
        }

        NotifyResult {
            ok: notification || sound,
            notification,
            sound,
            skipped: None,
        }
    }

    /// Soundboard: register a pack, returning its unregister guard.
    pub fn register_pack(&mut self, pack: RegisteredSoundPack) -> Option<PackUnregister> {
        let normalized = normalize_pack(&pack)?;
        let id = normalized.id.clone();
        let instance = self.next_pack_instance;
        self.next_pack_instance += 1;
        self.packs.insert(id.clone(), (normalized, instance));
        Some(PackUnregister { id, instance })
    }

    /// Soundboard: run an unregister guard (removes only the same registration).
    pub fn unregister_pack(&mut self, guard: PackUnregister) {
        let matches = self
            .packs
            .get(&guard.id)
            .map(|(_, instance)| *instance == guard.instance)
            .unwrap_or(false);
        if matches {
            self.packs.remove(&guard.id);
        }
    }

    /// Soundboard: activate a pack.
    pub fn activate(&mut self, id: &str, persist: bool) -> bool {
        if !self.packs.contains_key(id) {
            return false;
        }
        self.active_pack_id = Some(id.to_string());
        if persist {
            if let Some(ref kv) = self.kv {
                kv.set(KV_SOUND_PACK, id);
            }
        }
        true
    }

    /// Soundboard: current pack id.
    pub fn current(&self) -> String {
        self.current_pack().id.clone()
    }

    /// Soundboard: list packs.
    pub fn list(&self) -> Vec<SoundPackInfo> {
        let current = self.current_pack().id.clone();
        self.packs
            .values()
            .map(|(pack, _)| SoundPackInfo {
                id: pack.id.clone(),
                name: pack.name.clone(),
                active: pack.id == current,
                builtin: pack.builtin,
            })
            .collect()
    }

    /// Dispose the host.
    pub fn dispose(&mut self) {
        if self.disposed {
            return;
        }
        self.disposed = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_text_strips_ansi_and_controls() {
        assert_eq!(normalize_text("\x1b[31mhello\x1b[0m", "", 240), "hello");
        assert_eq!(normalize_text("hello\nworld", "", 240), "hello world");
        assert_eq!(normalize_text("", "fallback", 240), "fallback");
        assert_eq!(normalize_text("abc", "", 2), "ab");
    }

    #[test]
    fn clamp_volume_bounds() {
        assert_eq!(clamp_volume(0.5), 0.5);
        assert_eq!(clamp_volume(1.5), 1.0);
        assert_eq!(clamp_volume(-0.1), 0.0);
        assert_eq!(clamp_volume(f64::NAN), 0.0);
    }

    #[test]
    fn focus_skip_always_never_skips() {
        assert_eq!(focus_skip(AttentionWhen::Always, FocusState::Unknown), None);
        assert_eq!(focus_skip(AttentionWhen::Always, FocusState::Focused), None);
        assert_eq!(focus_skip(AttentionWhen::Always, FocusState::Blurred), None);
    }

    #[test]
    fn focus_skip_unknown_focus() {
        assert_eq!(
            focus_skip(AttentionWhen::Blurred, FocusState::Unknown),
            Some(SkipReason::FocusUnknown)
        );
        assert_eq!(
            focus_skip(AttentionWhen::Focused, FocusState::Unknown),
            Some(SkipReason::FocusUnknown)
        );
    }

    #[test]
    fn focus_skip_focused_terminal() {
        assert_eq!(
            focus_skip(AttentionWhen::Blurred, FocusState::Focused),
            Some(SkipReason::Focused)
        );
        assert_eq!(
            focus_skip(AttentionWhen::Focused, FocusState::Focused),
            None
        );
    }

    #[test]
    fn focus_skip_blurred_terminal() {
        assert_eq!(
            focus_skip(AttentionWhen::Focused, FocusState::Blurred),
            Some(SkipReason::Blurred)
        );
        assert_eq!(
            focus_skip(AttentionWhen::Blurred, FocusState::Blurred),
            None
        );
    }

    #[test]
    fn normalize_pack_filters_invalid_sounds() {
        let pack = RegisteredSoundPack {
            id: "custom".to_string(),
            name: Some("Custom".to_string()),
            builtin: false,
            sounds: HashMap::from([
                ("default".to_string(), "/path/to/default.mp3".to_string()),
                ("invalid".to_string(), "/path/to/invalid.mp3".to_string()),
                ("error".to_string(), "".to_string()),
            ]),
        };
        let normalized = normalize_pack(&pack).unwrap();
        assert_eq!(normalized.sounds.len(), 1);
        assert!(normalized.sounds.contains_key("default"));
    }

    #[test]
    fn normalize_pack_rejects_empty_id() {
        let pack = RegisteredSoundPack {
            id: "   ".to_string(),
            name: None,
            builtin: false,
            sounds: HashMap::new(),
        };
        assert!(normalize_pack(&pack).is_none());
    }

    #[test]
    fn builtin_pack_has_all_sounds() {
        let pack = builtin_pack();
        assert_eq!(pack.sounds.len(), 6);
        assert!(pack.builtin);
    }
}
