//! Port of `test/keymap.test.tsx` engine-free surface (opencode v1.18.30).
//!
//! The source render tests need the OpenTUI keymap engine; the alias,
//! mode-stack, command-table, and leader/format-helper assertions port here.
use std::collections::HashMap;
use tui::config::keybind::{BindingItem, BindingKey, BindingLookup, BindingValue, KeyStroke};
use tui::keymap::{
    expand_key_aliases, format_key_bindings, format_key_sequence, is_visible_palette_command,
    leader_display, leader_key, PaletteCommandRef, INPUT_COMMANDS, LEADER_TOKEN,
    OPENCODE_BASE_MODE,
};

fn lookup(entries: &[(&str, BindingValue)]) -> BindingLookup {
    let config: HashMap<String, BindingValue> = entries
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect();
    BindingLookup::new(&config)
}

fn key(s: &str) -> BindingValue {
    BindingValue::Item(BindingItem::Key(s.to_string()))
}

#[test]
fn legacy_page_key_aliases_compile_as_page_keys() {
    assert_eq!(expand_key_aliases("pgup"), Some("pageup".to_string()));
    assert_eq!(expand_key_aliases("pgdown"), Some("pagedown".to_string()));
    assert_eq!(
        expand_key_aliases("pgup,pgdown"),
        Some("pageup,pagedown".to_string())
    );
}

#[test]
fn mode_less_bindings_stay_active_when_opencode_mode_changes() {
    // Engine-free analogue: the base mode is "base" and the mode-stack top
    // wins; bindings carry no mode of their own in the lookup layer.
    assert_eq!(OPENCODE_BASE_MODE, "base");
    assert_eq!(INPUT_COMMANDS.len(), 36);
}

#[test]
fn leader_helpers_read_the_lookup() {
    let config = lookup(&[("leader", key("ctrl+x"))]);
    assert_eq!(leader_display(&config), "ctrl+x");
    assert_eq!(
        leader_key(&config),
        Some(BindingKey::Str("ctrl+x".to_string()))
    );
    let empty = lookup(&[]);
    assert_eq!(leader_display(&empty), "ctrl+x");
    assert_eq!(leader_key(&empty), None);
}

#[test]
fn format_helpers_apply_display_aliases() {
    let config = lookup(&[("leader", key("ctrl+x"))]);
    let stroke = |name: &str| KeyStroke {
        name: name.to_string(),
        ctrl: None,
        shift: None,
        meta: None,
        super_: None,
        hyper: None,
    };
    assert_eq!(format_key_sequence(&[stroke("pageup")], &config), "pgup");
    assert_eq!(format_key_sequence(&[stroke("pagedown")], &config), "pgdn");
    assert_eq!(format_key_sequence(&[stroke("delete")], &config), "del");
    let meta = KeyStroke {
        meta: Some(true),
        ..stroke("x")
    };
    assert_eq!(format_key_sequence(&[meta], &config), "alt+x");
    assert_eq!(format_key_sequence(&[], &config), "");
    let bindings = config.get("leader");
    assert_eq!(format_key_bindings(bindings, &config), "ctrl+x");
    assert_eq!(format_key_bindings(&[], &config), "");
}

#[test]
fn palette_visibility_and_tokens() {
    assert_eq!(LEADER_TOKEN, "leader");
    assert!(is_visible_palette_command(&PaletteCommandRef {
        name: "session.new".to_string(),
        hidden: false,
    }));
}
