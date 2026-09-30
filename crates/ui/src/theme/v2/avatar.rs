//! Rust port of `packages/ui/src/theme/v2/avatar.ts` (opencode v1.18.30).
//!
//! 1:1 data tables — fixed project avatar colors (OC-2), theme-independent.

#![allow(dead_code)]

use super::super::types::V2ColorValue;

/// Fixed project avatar foreground (1:1 with TS `V2_AVATAR_FG`).
pub const V2_AVATAR_FG: &str = "#ffffffff";

/// 1:1 with TS `V2_AVATAR_LIGHT` (source order preserved).
pub const V2_AVATAR_LIGHT: &[(&str, &str)] = &[
    ("v2-avatar-fg", V2_AVATAR_FG),
    ("v2-avatar-bg-orange", "#ee7330ff"),
    ("v2-avatar-border-orange", "#d16427ff"),
    ("v2-avatar-bg-yellow", "#e7af36ff"),
    ("v2-avatar-border-yellow", "#cb9f34ff"),
    ("v2-avatar-bg-cyan", "#0096b8ff"),
    ("v2-avatar-border-cyan", "#007d9bff"),
    ("v2-avatar-bg-green", "#2eaf5aff"),
    ("v2-avatar-border-green", "#198b43ff"),
    ("v2-avatar-bg-red", "#d92e3cff"),
    ("v2-avatar-border-red", "#b82d35ff"),
    ("v2-avatar-bg-pink", "#e4429eff"),
    ("v2-avatar-border-pink", "#c83d8bff"),
    ("v2-avatar-bg-blue", "#3250dfff"),
    ("v2-avatar-border-blue", "#2c47c8ff"),
    ("v2-avatar-bg-purple", "#623be2ff"),
    ("v2-avatar-border-purple", "#5230c2ff"),
    ("v2-avatar-bg-gray", "#5c5c5cff"),
    ("v2-avatar-border-gray", "#3a3a3aff"),
];

/// 1:1 with TS `V2_AVATAR_DARK` (source order preserved).
pub const V2_AVATAR_DARK: &[(&str, &str)] = &[
    ("v2-avatar-fg", V2_AVATAR_FG),
    ("v2-avatar-bg-orange", "#723d22ff"),
    ("v2-avatar-border-orange", "#ff8648ff"),
    ("v2-avatar-bg-yellow", "#68552bff"),
    ("v2-avatar-border-yellow", "#e7af36ff"),
    ("v2-avatar-bg-cyan", "#005a6eff"),
    ("v2-avatar-border-cyan", "#0096b8ff"),
    ("v2-avatar-bg-green", "#196130ff"),
    ("v2-avatar-border-green", "#49c970ff"),
    ("v2-avatar-bg-red", "#7a1f23ff"),
    ("v2-avatar-border-red", "#d92e3cff"),
    ("v2-avatar-bg-pink", "#8c2d61ff"),
    ("v2-avatar-border-pink", "#e4429eff"),
    ("v2-avatar-bg-blue", "#263fa9ff"),
    ("v2-avatar-border-blue", "#7698fdff"),
    ("v2-avatar-bg-purple", "#361f83ff"),
    ("v2-avatar-border-purple", "#7152f4ff"),
    ("v2-avatar-bg-gray", "#5c5c5cff"),
    ("v2-avatar-border-gray", "#aeaeaeff"),
];

/// Collect the light avatar table into an owned map.
pub fn avatar_light() -> std::collections::BTreeMap<String, V2ColorValue> {
    V2_AVATAR_LIGHT
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

/// Collect the dark avatar table into an owned map.
pub fn avatar_dark() -> std::collections::BTreeMap<String, V2ColorValue> {
    V2_AVATAR_DARK
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}
