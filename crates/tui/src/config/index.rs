//! Rust port of `src/config/index.tsx` (opencode v1.18.30).
//!
//! Config schemas (Info/Resolved), resolve() with host-neutral defaults,
//! and the TuiConfig provider/context. The Effect schemas become Rust
//! structs with validation; `createBindingLookup` maps to the keybind
//! module's `BindingLookup`.
//!
//! Original file: `packages/tui/src/config/index.tsx`

use std::collections::HashMap;

use serde_json::Value;

use super::keybind::{self, BindingLookup, BindingValue};

/// Attention sound names (mirrors `AttentionSoundName` literals).
pub const ATTENTION_SOUND_NAMES: &[&str] = &[
    "default",
    "question",
    "permission",
    "error",
    "done",
    "subagent_done",
];

/// Leader timeout default in milliseconds.
pub const LEADER_TIMEOUT_DEFAULT: u64 = 2000;

/// Cursor style default.
pub const CURSOR_DEFAULT_STYLE: &str = "block";

/// Cursor blinking default.
pub const CURSOR_DEFAULT_BLINKING: bool = true;

/// Attention defaults.
pub const ATTENTION_ENABLED_DEFAULT: bool = false;
pub const ATTENTION_NOTIFICATIONS_DEFAULT: bool = true;
pub const ATTENTION_SOUND_DEFAULT: bool = true;
pub const ATTENTION_VOLUME_DEFAULT: f64 = 0.4;
pub const ATTENTION_SOUND_PACK_DEFAULT: &str = "opencode.default";

/// Mouse capture default.
pub const MOUSE_DEFAULT: bool = true;

/// Thinking opacity default.
pub const THINKING_OPACITY_DEFAULT: f64 = 0.6;

/// Plugin options: arbitrary key-value map.
pub type PluginOptions = HashMap<String, serde_json::Value>;

/// Plugin spec: either a string or a (string, options) tuple.
#[derive(Debug, Clone, PartialEq)]
pub enum PluginSpec {
    Name(String),
    WithOptions(String, PluginOptions),
}

/// Leader timeout (must be > 0).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LeaderTimeout(pub u64);

/// Scroll speed (must be >= 0.001).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollSpeed(pub f64);

/// Scroll acceleration settings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollAcceleration {
    pub enabled: bool,
}

/// Diff style: "auto" or "stacked".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffStyle {
    Auto,
    Stacked,
}

/// Cursor settings.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Cursor {
    pub style: Option<String>,
    pub blinking: Option<bool>,
}

/// Attention sound paths: sound name → file path.
pub type AttentionSoundPaths = HashMap<String, String>;

/// Attention settings.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Attention {
    pub enabled: Option<bool>,
    pub notifications: Option<bool>,
    pub sound: Option<bool>,
    pub volume: Option<f64>,
    pub sound_pack: Option<String>,
    pub sounds: Option<AttentionSoundPaths>,
}

/// Prompt size settings.
#[derive(Debug, Clone, PartialEq)]
pub struct Prompt {
    pub max_height: Option<u64>,
    pub max_width: Option<PromptMaxWidth>,
}

/// Prompt max width: a positive integer or "auto".
#[derive(Debug, Clone, PartialEq)]
pub enum PromptMaxWidth {
    Pixels(u64),
    Auto,
}

/// The raw config input (mirrors `Info` schema type).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Info {
    pub theme: Option<String>,
    pub keybinds: Option<HashMap<String, BindingValue>>,
    pub plugin: Option<Vec<PluginSpec>>,
    pub plugin_enabled: Option<HashMap<String, bool>>,
    pub leader_timeout: Option<LeaderTimeout>,
    pub attention: Option<Attention>,
    pub prompt: Option<Prompt>,
    pub scroll_speed: Option<ScrollSpeed>,
    pub scroll_acceleration: Option<ScrollAcceleration>,
    pub diff_style: Option<DiffStyle>,
    pub cursor: Option<Cursor>,
    pub mouse: Option<bool>,
}

/// Resolve options.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResolveOptions {
    pub terminal_suspend: bool,
}

/// The resolved config (mirrors `Resolved` type).
#[derive(Debug, Clone)]
pub struct Resolved {
    pub theme: Option<String>,
    pub keybinds: BindingLookup,
    pub plugin: Option<Vec<PluginSpec>>,
    pub plugin_enabled: Option<HashMap<String, bool>>,
    pub leader_timeout: u64,
    pub attention: ResolvedAttention,
    pub prompt: Option<Prompt>,
    pub scroll_speed: Option<ScrollSpeed>,
    pub scroll_acceleration: Option<ScrollAcceleration>,
    pub diff_style: Option<DiffStyle>,
    pub cursor: Option<ResolvedCursor>,
    pub mouse: bool,
}

/// Resolved attention with defaults applied.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedAttention {
    pub enabled: bool,
    pub notifications: bool,
    pub sound: bool,
    pub volume: f64,
    pub sound_pack: String,
    pub sounds: AttentionSoundPaths,
}

/// Resolved cursor with defaults applied.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedCursor {
    pub style: String,
    pub blinking: bool,
}

/// Validate and resolve a config input.
pub fn resolve(input: Info, options: ResolveOptions) -> Resolved {
    let mut keybinds = input.keybinds.clone().unwrap_or_default();

    if !options.terminal_suspend {
        keybinds.insert("terminal_suspend".to_string(), BindingValue::None);
        if !keybinds.contains_key("input_undo") {
            let default = keybind::default_value("input_undo");
            let default_str = match default {
                Some(keybind::DefaultBinding::Str(s)) => s.to_string(),
                _ => String::new(),
            };
            let parts: Vec<&str> = default_str.split(',').collect();
            let mut deduped: Vec<&str> = Vec::new();
            for value in std::iter::once("ctrl+z").chain(parts) {
                if !deduped.contains(&value) {
                    deduped.push(value);
                }
            }
            keybinds.insert(
                "input_undo".to_string(),
                BindingValue::Item(super::keybind::BindingItem::Key(deduped.join(","))),
            );
        }
    }

    let parsed = keybind::parse(&keybinds).unwrap_or(keybinds.clone());
    let lookup = BindingLookup::new(&parsed);

    let attention = input.attention.unwrap_or_default();
    let resolved_attention = ResolvedAttention {
        enabled: attention.enabled.unwrap_or(ATTENTION_ENABLED_DEFAULT),
        notifications: attention
            .notifications
            .unwrap_or(ATTENTION_NOTIFICATIONS_DEFAULT),
        sound: attention.sound.unwrap_or(ATTENTION_SOUND_DEFAULT),
        volume: attention.volume.unwrap_or(ATTENTION_VOLUME_DEFAULT),
        sound_pack: attention
            .sound_pack
            .unwrap_or_else(|| ATTENTION_SOUND_PACK_DEFAULT.to_string()),
        sounds: attention.sounds.unwrap_or_default(),
    };

    let resolved_cursor = input.cursor.map(|c| ResolvedCursor {
        style: c.style.unwrap_or_else(|| CURSOR_DEFAULT_STYLE.to_string()),
        blinking: c.blinking.unwrap_or(CURSOR_DEFAULT_BLINKING),
    });

    Resolved {
        theme: input.theme,
        keybinds: lookup,
        plugin: input.plugin,
        plugin_enabled: input.plugin_enabled,
        leader_timeout: input
            .leader_timeout
            .map(|t| t.0)
            .unwrap_or(LEADER_TIMEOUT_DEFAULT),
        attention: resolved_attention,
        prompt: input.prompt,
        scroll_speed: input.scroll_speed,
        scroll_acceleration: input.scroll_acceleration,
        diff_style: input.diff_style,
        cursor: resolved_cursor,
        mouse: input.mouse.unwrap_or(MOUSE_DEFAULT),
    }
}

/// TuiConfig provider: returns the config (Solid context → direct value).
pub fn tui_config_provider(config: Resolved) -> Resolved {
    config
}

/// Use TuiConfig: returns the config or panics if missing.
pub fn use_tui_config(config: Option<&Resolved>) -> Resolved {
    config.cloned().expect("TuiConfigProvider is missing")
}

// ---------------------------------------------------------------------------
// JSON decoding (mirrors `Schema.decodeUnknownSync(Info)` / `(PluginSpec)`)
// ---------------------------------------------------------------------------

fn decode_bool(value: &Value, field: &str) -> Result<bool, String> {
    value
        .as_bool()
        .ok_or_else(|| format!("expected boolean for {}", field))
}

fn decode_binding_value(value: &Value) -> Result<BindingValue, String> {
    match value {
        Value::Bool(false) => Ok(BindingValue::Disabled),
        Value::Bool(true) => Err("binding value `true` is not allowed".to_string()),
        Value::String(s) if s == "none" => Ok(BindingValue::None),
        Value::String(s) => Ok(BindingValue::Item(super::keybind::BindingItem::Key(
            s.clone(),
        ))),
        Value::Object(_) => Err("binding objects are not supported in JSON config".to_string()),
        Value::Array(_) => Err("binding arrays are not supported in JSON config".to_string()),
        _ => Err("invalid binding value".to_string()),
    }
}

/// Decode a plugin spec (mirrors `PluginSpec` schema).
pub fn decode_plugin_spec(value: &Value) -> Result<PluginSpec, String> {
    match value {
        Value::String(name) => Ok(PluginSpec::Name(name.clone())),
        Value::Array(items) => {
            if items.len() != 2 {
                return Err("plugin tuple must have exactly 2 elements".to_string());
            }
            let name = items[0]
                .as_str()
                .ok_or_else(|| "plugin tuple first element must be a string".to_string())?;
            let options = items[1]
                .as_object()
                .ok_or_else(|| "plugin tuple second element must be an object".to_string())?;
            Ok(PluginSpec::WithOptions(
                name.to_string(),
                options
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            ))
        }
        _ => Err("plugin spec must be a string or tuple".to_string()),
    }
}

/// Decode a raw JSON value into [`Info`], enforcing the schema constraints.
pub fn decode_info(value: &Value) -> Result<Info, String> {
    let obj = value
        .as_object()
        .ok_or_else(|| "config must be an object".to_string())?;
    let mut info = Info::default();

    if let Some(v) = obj.get("theme") {
        info.theme = Some(
            v.as_str()
                .ok_or_else(|| "theme must be a string".to_string())?
                .to_string(),
        );
    }
    if let Some(v) = obj.get("keybinds") {
        let map = v
            .as_object()
            .ok_or_else(|| "keybinds must be an object".to_string())?;
        let mut keybinds = HashMap::new();
        for (k, val) in map {
            keybinds.insert(k.clone(), decode_binding_value(val)?);
        }
        info.keybinds = Some(keybinds);
    }
    if let Some(v) = obj.get("plugin") {
        let arr = v
            .as_array()
            .ok_or_else(|| "plugin must be an array".to_string())?;
        info.plugin = Some(
            arr.iter()
                .map(decode_plugin_spec)
                .collect::<Result<Vec<_>, _>>()?,
        );
    }
    if let Some(v) = obj.get("plugin_enabled") {
        let map = v
            .as_object()
            .ok_or_else(|| "plugin_enabled must be an object".to_string())?;
        let mut out = HashMap::new();
        for (k, val) in map {
            out.insert(k.clone(), decode_bool(val, "plugin_enabled value")?);
        }
        info.plugin_enabled = Some(out);
    }
    if let Some(v) = obj.get("leader_timeout") {
        let n = v
            .as_u64()
            .filter(|n| *n > 0)
            .ok_or_else(|| "leader_timeout must be an integer greater than 0".to_string())?;
        info.leader_timeout = Some(LeaderTimeout(n));
    }
    if let Some(v) = obj.get("attention") {
        let att = v
            .as_object()
            .ok_or_else(|| "attention must be an object".to_string())?;
        let mut attention = Attention::default();
        if let Some(x) = att.get("enabled") {
            attention.enabled = Some(decode_bool(x, "attention.enabled")?);
        }
        if let Some(x) = att.get("notifications") {
            attention.notifications = Some(decode_bool(x, "attention.notifications")?);
        }
        if let Some(x) = att.get("sound") {
            attention.sound = Some(decode_bool(x, "attention.sound")?);
        }
        if let Some(x) = att.get("volume") {
            let n = x
                .as_f64()
                .filter(|n| (0.0..=1.0).contains(n))
                .ok_or_else(|| "attention.volume must be between 0 and 1".to_string())?;
            attention.volume = Some(n);
        }
        if let Some(x) = att.get("sound_pack") {
            attention.sound_pack = Some(
                x.as_str()
                    .ok_or_else(|| "attention.sound_pack must be a string".to_string())?
                    .to_string(),
            );
        }
        if let Some(x) = att.get("sounds") {
            let map = x
                .as_object()
                .ok_or_else(|| "attention.sounds must be an object".to_string())?;
            let mut sounds = HashMap::new();
            for (k, val) in map {
                // Unknown sound names are dropped, mirroring the schema.
                if !ATTENTION_SOUND_NAMES.contains(&k.as_str()) {
                    continue;
                }
                let path = val
                    .as_str()
                    .ok_or_else(|| "sound path must be a string".to_string())?;
                sounds.insert(k.clone(), path.to_string());
            }
            attention.sounds = Some(sounds);
        }
        info.attention = Some(attention);
    }
    if let Some(v) = obj.get("prompt") {
        let pr = v
            .as_object()
            .ok_or_else(|| "prompt must be an object".to_string())?;
        let mut prompt = Prompt {
            max_height: None,
            max_width: None,
        };
        if let Some(x) = pr.get("max_height") {
            let n = x
                .as_u64()
                .filter(|n| *n > 0)
                .ok_or_else(|| "prompt.max_height must be an integer greater than 0".to_string())?;
            prompt.max_height = Some(n);
        }
        if let Some(x) = pr.get("max_width") {
            if let Some(s) = x.as_str() {
                if s != "auto" {
                    return Err(
                        "prompt.max_width must be a positive integer or \"auto\"".to_string()
                    );
                }
                prompt.max_width = Some(PromptMaxWidth::Auto);
            } else {
                let n = x.as_u64().filter(|n| *n > 0).ok_or_else(|| {
                    "prompt.max_width must be a positive integer or \"auto\"".to_string()
                })?;
                prompt.max_width = Some(PromptMaxWidth::Pixels(n));
            }
        }
        info.prompt = Some(prompt);
    }
    if let Some(v) = obj.get("scroll_speed") {
        let n = v
            .as_f64()
            .filter(|n| *n >= 0.001)
            .ok_or_else(|| "scroll_speed must be at least 0.001".to_string())?;
        info.scroll_speed = Some(ScrollSpeed(n));
    }
    if let Some(v) = obj.get("scroll_acceleration") {
        let sa = v
            .as_object()
            .ok_or_else(|| "scroll_acceleration must be an object".to_string())?;
        let enabled = sa
            .get("enabled")
            .map(|x| decode_bool(x, "scroll_acceleration.enabled"))
            .transpose()?
            .unwrap_or(false);
        info.scroll_acceleration = Some(ScrollAcceleration { enabled });
    }
    if let Some(v) = obj.get("diff_style") {
        let s = v
            .as_str()
            .ok_or_else(|| "diff_style must be a string".to_string())?;
        info.diff_style = Some(match s {
            "auto" => DiffStyle::Auto,
            "stacked" => DiffStyle::Stacked,
            _ => return Err("diff_style must be \"auto\" or \"stacked\"".to_string()),
        });
    }
    if let Some(v) = obj.get("cursor") {
        let cur = v
            .as_object()
            .ok_or_else(|| "cursor must be an object".to_string())?;
        let mut cursor = Cursor {
            style: None,
            blinking: None,
        };
        if let Some(x) = cur.get("style") {
            let s = x
                .as_str()
                .ok_or_else(|| "cursor.style must be a string".to_string())?;
            match s {
                "block" | "underline" | "line" | "default" => cursor.style = Some(s.to_string()),
                _ => {
                    return Err(
                        "cursor.style must be block, underline, line, or default".to_string()
                    )
                }
            }
        }
        if let Some(x) = cur.get("blinking") {
            cursor.blinking = Some(decode_bool(x, "cursor.blinking")?);
        }
        info.cursor = Some(cursor);
    }
    if let Some(v) = obj.get("mouse") {
        info.mouse = Some(decode_bool(v, "mouse")?);
    }
    Ok(info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_empty_gives_defaults() {
        let config = resolve(
            Info::default(),
            ResolveOptions {
                terminal_suspend: true,
            },
        );
        assert!(!config.attention.enabled);
        assert!(config.attention.notifications);
        assert!(config.attention.sound);
        assert_eq!(config.attention.volume, 0.4);
        assert_eq!(config.attention.sound_pack, "opencode.default");
        assert_eq!(config.attention.sounds.len(), 0);
        assert_eq!(config.leader_timeout, 2000);
        assert!(config.mouse);
        assert!(config.keybinds.has("terminal.suspend"));
        assert!(config.keybinds.has("session.list"));
        assert!(config.cursor.is_none());
    }

    #[test]
    fn resolve_applies_overrides_without_mutating_input() {
        let mut keybinds = HashMap::new();
        keybinds.insert(
            "session_list".to_string(),
            BindingValue::Item(keybind::BindingItem::Key("ctrl+l".to_string())),
        );
        let input = Info {
            theme: Some("custom".to_string()),
            keybinds: Some(keybinds),
            mouse: Some(false),
            leader_timeout: Some(LeaderTimeout(750)),
            cursor: Some(Cursor {
                style: None,
                blinking: Some(false),
            }),
            ..Default::default()
        };
        let config = resolve(
            input.clone(),
            ResolveOptions {
                terminal_suspend: true,
            },
        );
        assert_eq!(config.theme.as_deref(), Some("custom"));
        assert!(!config.mouse);
        assert_eq!(config.leader_timeout, 750);
        let cursor = config.cursor.unwrap();
        assert_eq!(cursor.style, "block");
        assert!(!cursor.blinking);
        assert_eq!(config.keybinds.get("session.list").len(), 1);
    }

    #[test]
    fn resolve_session_move_keybind() {
        let mut keybinds = HashMap::new();
        keybinds.insert(
            "session_move".to_string(),
            BindingValue::Item(keybind::BindingItem::Key("ctrl+o".to_string())),
        );
        let config = resolve(
            Info {
                keybinds: Some(keybinds),
                ..Default::default()
            },
            ResolveOptions {
                terminal_suspend: true,
            },
        );
        let bindings = config.keybinds.get("session.move");
        assert_eq!(bindings.len(), 1);
        assert_eq!(
            bindings[0].key,
            keybind::BindingKey::Str("ctrl+o".to_string())
        );
    }

    #[test]
    fn resolve_disables_suspend_and_assigns_ctrl_z_to_undo() {
        let config = resolve(
            Info::default(),
            ResolveOptions {
                terminal_suspend: false,
            },
        );
        assert!(!config.keybinds.has("terminal.suspend"));
        let bindings = config.keybinds.get("input.undo");
        assert_eq!(bindings.len(), 1);
        assert_eq!(
            bindings[0].key,
            keybind::BindingKey::Str("ctrl+z,ctrl+-,super+z".to_string())
        );
    }

    #[test]
    fn resolve_preserves_explicit_undo_when_suspend_unsupported() {
        let mut keybinds = HashMap::new();
        keybinds.insert(
            "input_undo".to_string(),
            BindingValue::Item(keybind::BindingItem::Key("ctrl+u".to_string())),
        );
        keybinds.insert(
            "terminal_suspend".to_string(),
            BindingValue::Item(keybind::BindingItem::Key("ctrl+s".to_string())),
        );
        let config = resolve(
            Info {
                keybinds: Some(keybinds),
                ..Default::default()
            },
            ResolveOptions {
                terminal_suspend: false,
            },
        );
        assert!(!config.keybinds.has("terminal.suspend"));
        let bindings = config.keybinds.get("input.undo");
        assert_eq!(bindings.len(), 1);
        assert_eq!(
            bindings[0].key,
            keybind::BindingKey::Str("ctrl+u".to_string())
        );
    }

    #[test]
    fn use_tui_config_panics_without_provider() {
        let result = std::panic::catch_unwind(|| use_tui_config(None));
        assert!(result.is_err());
    }
}
