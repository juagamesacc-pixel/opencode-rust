use app::addons::serialize::{
    constrain, equal_bg, equal_fg, equal_flags, BufferCell, SerializeAddon,
};

#[test]
fn constrain_clamps() {
    assert_eq!(constrain(5, 0, 10), 5);
    assert_eq!(constrain(-1, 0, 10), 0);
    assert_eq!(constrain(20, 0, 10), 10);
}

#[test]
fn equal_fg_bg_flags() {
    let a = BufferCell {
        chars: "a".to_string(),
        code: 97,
        width: 1,
        fg_color_mode: 1,
        bg_color_mode: 1,
        fg_color: 0xFF0000,
        bg_color: 0x00FF00,
        bold: 1,
        italic: 0,
        underline: 0,
        strikethrough: 0,
        blink: 0,
        inverse: 0,
        invisible: 0,
        faint: 0,
        dim: false,
    };
    let b = a.clone();
    assert!(equal_fg(&a, &b));
    assert!(equal_bg(&a, &b));
    assert!(equal_flags(&a, &b));
    let mut c = a.clone();
    c.bold = 0;
    assert!(!equal_flags(&a, &c));
}

#[test]
fn serialize_addon_requires_load() {
    let addon = SerializeAddon::new();
    assert!(addon.serialize(None).is_err());
    assert_eq!(
        addon.serialize(None).unwrap_err(),
        "Cannot use addon until it has been loaded"
    );
}

#[test]
fn serialize_addon_after_activate_empty() {
    let mut addon = SerializeAddon::new();
    addon.activate();
    assert!(addon.serialize(None).is_ok());
    assert_eq!(addon.serialize(None).unwrap(), "");
}

#[test]
fn serialize_as_text_requires_load() {
    let addon = SerializeAddon::new();
    assert!(addon.serialize_as_text(None, None).is_err());
}

#[test]
fn addon_lifecycle() {
    let mut addon = SerializeAddon::new();
    assert!(!addon.loaded);
    addon.activate();
    assert!(addon.loaded);
    addon.dispose();
    assert!(!addon.loaded);
}
