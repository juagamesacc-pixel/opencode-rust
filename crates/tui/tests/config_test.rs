//! Port of `test/config.test.tsx` (opencode v1.18.30).
//!
//! Schema constraints, resolve() defaults/overrides, and the provider
//! contract. The Solid-context render test maps to `use_tui_config`.
use serde_json::json;
use tui::config::index::{
    decode_info, decode_plugin_spec, resolve, use_tui_config, Info, ResolveOptions,
    ATTENTION_SOUND_NAMES, LEADER_TIMEOUT_DEFAULT,
};

#[test]
fn defines_package_owned_plugin_specs_and_attention_sound_names() {
    assert_eq!(
        decode_plugin_spec(&json!("example-plugin")).unwrap(),
        tui::config::index::PluginSpec::Name("example-plugin".to_string())
    );
    assert_eq!(
        decode_plugin_spec(&json!(["example-plugin", { "enabled": true }])).unwrap(),
        tui::config::index::PluginSpec::WithOptions(
            "example-plugin".to_string(),
            [("enabled".to_string(), json!(true))].into_iter().collect()
        )
    );
    assert!(decode_plugin_spec(&json!(["example-plugin"])).is_err());
    assert_eq!(
        ATTENTION_SOUND_NAMES,
        &[
            "default",
            "question",
            "permission",
            "error",
            "done",
            "subagent_done"
        ]
    );
}

#[test]
fn validates_config_constraints() {
    let info = decode_info(&json!({
        "leader_timeout": 250,
        "attention": { "volume": 1, "sounds": { "done": "done.wav" } },
        "prompt": { "max_height": 10, "max_width": "auto" },
        "scroll_speed": 0.001,
        "diff_style": "stacked",
        "cursor": { "blinking": false },
        "plugin": ["example-plugin"],
    }))
    .unwrap();
    assert_eq!(info.leader_timeout.unwrap().0, 250);
    assert_eq!(info.attention.unwrap().volume, Some(1.0));
    assert_eq!(
        info.diff_style,
        Some(tui::config::index::DiffStyle::Stacked)
    );
    assert_eq!(info.cursor.unwrap().blinking, Some(false));

    assert!(decode_info(&json!({ "leader_timeout": 0 })).is_err());
    assert!(decode_info(&json!({ "attention": { "volume": 1.1 } })).is_err());
    assert!(decode_info(&json!({ "prompt": { "max_width": 0 } })).is_err());
    assert!(decode_info(&json!({ "scroll_speed": 0 })).is_err());
    assert!(decode_info(&json!({ "cursor": { "style": "beam" } })).is_err());
    let dropped =
        decode_info(&json!({ "attention": { "sounds": { "unknown": "sound.wav" } } })).unwrap();
    assert_eq!(dropped.attention.unwrap().sounds.unwrap().len(), 0);
}

#[test]
fn resolves_host_neutral_defaults() {
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
    assert_eq!(config.leader_timeout, LEADER_TIMEOUT_DEFAULT);
    assert!(config.mouse);
    assert!(config.keybinds.has("terminal.suspend"));
    assert!(config.keybinds.has("session.list"));
    assert!(config.cursor.is_none());
}

#[test]
fn resolves_overrides_without_mutating_input() {
    use tui::config::keybind::{BindingItem, BindingValue};
    let mut keybinds = std::collections::HashMap::new();
    keybinds.insert(
        "session_list".to_string(),
        BindingValue::Item(BindingItem::Key("ctrl+l".to_string())),
    );
    let input = Info {
        theme: Some("custom".to_string()),
        mouse: Some(false),
        leader_timeout: Some(tui::config::index::LeaderTimeout(750)),
        keybinds: Some(keybinds),
        cursor: Some(tui::config::index::Cursor {
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
    assert_eq!(input.keybinds.unwrap().len(), 1);
}

#[test]
fn resolves_a_session_move_keybind() {
    use tui::config::keybind::{BindingItem, BindingValue};
    let mut keybinds = std::collections::HashMap::new();
    keybinds.insert(
        "session_move".to_string(),
        BindingValue::Item(BindingItem::Key("ctrl+o".to_string())),
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
        tui::config::keybind::BindingKey::Str("ctrl+o".to_string())
    );
}

#[test]
fn disables_suspend_and_assigns_ctrl_z_to_undo_when_unsupported() {
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
        tui::config::keybind::BindingKey::Str("ctrl+z,ctrl+-,super+z".to_string())
    );
}

#[test]
fn preserves_an_explicit_undo_binding_when_suspend_is_unsupported() {
    use tui::config::keybind::{BindingItem, BindingValue};
    let mut keybinds = std::collections::HashMap::new();
    keybinds.insert(
        "input_undo".to_string(),
        BindingValue::Item(BindingItem::Key("ctrl+u".to_string())),
    );
    keybinds.insert(
        "terminal_suspend".to_string(),
        BindingValue::Item(BindingItem::Key("ctrl+s".to_string())),
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
        tui::config::keybind::BindingKey::Str("ctrl+u".to_string())
    );
}

#[test]
fn provides_resolved_config_through_provider() {
    let config = resolve(
        Info {
            theme: Some("custom".to_string()),
            ..Default::default()
        },
        ResolveOptions {
            terminal_suspend: true,
        },
    );
    let provided = tui::config::index::tui_config_provider(config);
    let value = use_tui_config(Some(&provided));
    assert_eq!(value.theme.as_deref(), Some("custom"));
    assert!(value.mouse);
    assert_eq!(value.leader_timeout, LEADER_TIMEOUT_DEFAULT);
}

#[test]
fn requires_the_config_provider() {
    let result = std::panic::catch_unwind(|| use_tui_config(None));
    assert!(result.is_err());
}
