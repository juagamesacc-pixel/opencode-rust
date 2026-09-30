//! Port of `test/theme.test.ts` (opencode v1.18.30).
//!
//! Theme store, circular-ref rejection, and terminal-mode derivation.
//! (`discoverThemes` lives in `context/theme` and ports with batch 4.)
use serde_json::json;
use tui::theme::index::{
    add_theme, all_themes, default_theme, has_theme, resolve_theme, terminal_mode, ColorValue,
};

fn theme_value(name: &str) -> serde_json::Value {
    serde_json::to_value(default_theme(name).unwrap()).unwrap()
}

#[test]
fn add_theme_writes_into_module_theme_store() {
    let name = "plugin-theme-batch2-a";
    assert!(!has_theme(name));
    assert!(add_theme(name, &theme_value("opencode")));
    assert!(has_theme(name));
}

#[test]
fn add_theme_keeps_first_theme_for_duplicate_names() {
    let name = "plugin-theme-batch2-keep";
    let mut one = theme_value("opencode");
    let mut two = theme_value("opencode");
    one["theme"]["primary"] = json!("#101010");
    two["theme"]["primary"] = json!("#fefefe");

    assert!(add_theme(name, &one));
    assert!(!add_theme(name, &two));
    assert_eq!(
        all_themes().get(name).unwrap().theme.get("primary"),
        Some(&ColorValue::Hex("#101010".to_string()))
    );
}

#[test]
fn add_theme_ignores_entries_without_a_theme_object() {
    let name = "plugin-theme-batch2-invalid";
    assert!(!add_theme(name, &json!({ "defs": { "a": "#ffffff" } })));
    assert!(!has_theme(name));
}

#[test]
fn has_theme_checks_theme_presence() {
    let name = "plugin-theme-batch2-has";
    assert!(!has_theme(name));
    assert!(add_theme(name, &theme_value("opencode")));
    assert!(has_theme(name));
}

#[test]
fn resolve_theme_rejects_circular_color_refs() {
    let mut item = default_theme("opencode").unwrap();
    let mut defs = item.defs.clone().unwrap_or_default();
    defs.insert("one".to_string(), "two".to_string());
    defs.insert("two".to_string(), "one".to_string());
    item.defs = Some(defs);
    item.theme
        .insert("primary".to_string(), ColorValue::Ref("one".to_string()));
    let err = resolve_theme(&item, "dark").unwrap_err();
    assert!(err.contains("Circular color reference"));
}

#[test]
fn terminal_mode_derives_mode_from_refreshed_background() {
    assert_eq!(terminal_mode(Some("#fbf1c7")), Some("light"));
    assert_eq!(terminal_mode(Some("#1a1b26")), Some("dark"));
}

#[test]
fn terminal_mode_does_not_derive_mode_from_ansi_slot_zero() {
    assert_eq!(terminal_mode(None), None);
}
