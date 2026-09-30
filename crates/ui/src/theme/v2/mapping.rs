//! Rust port of `packages/ui/src/theme/v2/mapping.ts` (opencode v1.18.30).
//!
//! 1:1 data tables + `mapV2Semantics`/`mergeV2Tokens` merge order preserved.

#![allow(dead_code)]

use super::avatar::{avatar_dark, avatar_light};

/// 1:1 with TS `lightAgentTokens`.
pub const LIGHT_AGENT_TOKENS: &[(&str, &str)] = &[
    ("v2-agent-plan-solid", "var(--v2-pink-800)"),
    ("v2-agent-plan-border", "rgba(200, 61, 139, 0.20)"),
    ("v2-agent-plan-background", "rgba(253, 236, 243, 0.10)"),
    ("v2-agent-build-solid", "var(--v2-blue-800)"),
    ("v2-agent-build-border", "rgba(44, 71, 200, 0.20)"),
    ("v2-agent-build-background", "rgba(236, 241, 254, 0.10)"),
    ("v2-agent-explore-solid", "var(--v2-yellow-900)"),
    ("v2-agent-explore-border", "rgba(203, 159, 52, 0.20)"),
    ("v2-agent-explore-background", "rgba(254, 250, 236, 0.1)"),
    ("v2-agent-review-solid", "var(--v2-green-800)"),
    ("v2-agent-writer-solid", "var(--v2-purple-700)"),
];

/// 1:1 with TS `darkAgentTokens`.
pub const DARK_AGENT_TOKENS: &[(&str, &str)] = &[
    ("v2-agent-plan-solid", "var(--v2-pink-400)"),
    ("v2-agent-plan-border", "rgba(247, 153, 198, 0.20)"),
    ("v2-agent-plan-background", "rgba(170, 53, 118, 0.05)"),
    ("v2-agent-build-solid", "var(--v2-blue-300)"),
    ("v2-agent-build-border", "rgba(162, 188, 255, 0.20)"),
    ("v2-agent-build-background", "rgba(38, 63, 169, 0.05)"),
    ("v2-agent-explore-solid", "var(--v2-yellow-300)"),
    ("v2-agent-explore-border", "rgba(243, 218, 155, 0.20)"),
    ("v2-agent-explore-background", "rgba(172, 136, 51, 0.05)"),
    ("v2-agent-review-solid", "var(--v2-green-300)"),
    ("v2-agent-writer-solid", "var(--v2-purple-400)"),
];

/// 1:1 with TS `light` semantics head (entries before the avatar spread).
pub const LIGHT_TOKENS_HEAD: &[(&str, &str)] = &[
    ("v2-background-bg-base", "var(--v2-grey-100)"),
    ("v2-background-bg-deep", "var(--v2-grey-200)"),
    ("v2-background-bg-layer-01", "var(--v2-grey-300)"),
    ("v2-background-bg-layer-02", "var(--v2-grey-400)"),
    ("v2-background-bg-layer-03", "var(--v2-grey-500)"),
    ("v2-background-bg-layer-04", "var(--v2-grey-600)"),
    ("v2-background-bg-inverse", "var(--v2-grey-1000)"),
    ("v2-background-bg-contrast", "var(--v2-grey-900)"),
    ("v2-background-bg-button-neutral", "var(--v2-grey-100)"),
    ("v2-background-bg-accent", "var(--v2-blue-600)"),
    ("v2-text-text-inverse", "var(--v2-grey-100)"),
    ("v2-text-text-contrast", "var(--v2-grey-100)"),
    ("v2-text-text-accent", "var(--v2-blue-600)"),
    ("v2-text-text-accent-hover", "var(--v2-blue-700)"),
    ("v2-text-text-code-accent", "var(--v2-blue-900)"),
    ("v2-border-border-muted", "var(--v2-alpha-dark-8)"),
    ("v2-border-border-base", "var(--v2-alpha-dark-10)"),
    ("v2-border-border-strong", "var(--v2-alpha-dark-20)"),
    ("v2-border-border-inverse", "var(--v2-grey-1000)"),
    ("v2-border-border-focus", "var(--v2-blue-500)"),
    ("v2-overlay-simple-overlay-hover", "var(--v2-alpha-dark-4)"),
    (
        "v2-overlay-simple-overlay-pressed",
        "var(--v2-alpha-dark-8)",
    ),
    (
        "v2-overlay-simple-overlay-contrast-hover",
        "var(--v2-alpha-light-12)",
    ),
    (
        "v2-overlay-simple-overlay-contrast-pressed",
        "var(--v2-alpha-light-24)",
    ),
    ("v2-overlay-simple-overlay-scrim", "var(--v2-alpha-dark-40)"),
    (
        "v2-overlay-gradient-depth-overlay-depth-top",
        "var(--v2-alpha-light-100)",
    ),
    (
        "v2-overlay-gradient-depth-overlay-depth-bot",
        "var(--v2-alpha-light-0)",
    ),
    ("v2-overlay-simple-tab-active-scrim", "#fafafa00"),
    ("v2-overlay-simple-tab-hover-scrim", "#eeeeee00"),
    ("v2-overlay-simple-tab-scrim", "#fafafa00"),
    ("v2-state-bg-success", "var(--v2-green-100)"),
    ("v2-state-fg-success", "var(--v2-green-800)"),
    ("v2-state-border-success", "var(--v2-green-300)"),
    ("v2-state-bg-warning", "var(--v2-yellow-100)"),
    ("v2-state-fg-warning", "var(--v2-yellow-800)"),
    ("v2-state-border-warning", "var(--v2-yellow-300)"),
    ("v2-state-bg-danger", "var(--v2-red-100)"),
    ("v2-state-fg-danger", "var(--v2-red-800)"),
    ("v2-state-border-danger", "var(--v2-red-300)"),
    ("v2-state-bg-info", "var(--v2-blue-100)"),
    ("v2-state-fg-info", "var(--v2-blue-800)"),
    ("v2-state-border-info", "var(--v2-blue-300)"),
];

/// 1:1 with TS `dark` semantics head (entries before the avatar spread).
pub const DARK_TOKENS_HEAD: &[(&str, &str)] = &[
    ("v2-background-bg-base", "var(--v2-grey-1000)"),
    ("v2-background-bg-deep", "var(--v2-grey-1100)"),
    ("v2-background-bg-layer-01", "var(--v2-grey-800)"),
    ("v2-background-bg-layer-02", "var(--v2-grey-600)"),
    ("v2-background-bg-layer-03", "var(--v2-grey-500)"),
    ("v2-background-bg-layer-04", "var(--v2-grey-400)"),
    ("v2-background-bg-inverse", "var(--v2-grey-100)"),
    ("v2-background-bg-contrast", "var(--v2-grey-700)"),
    ("v2-background-bg-button-neutral", "var(--v2-alpha-light-6)"),
    ("v2-background-bg-accent", "var(--v2-blue-600)"),
    ("v2-text-text-inverse", "var(--v2-grey-1000)"),
    ("v2-text-text-contrast", "var(--v2-grey-100)"),
    ("v2-text-text-accent", "var(--v2-blue-400)"),
    ("v2-text-text-accent-hover", "var(--v2-blue-300)"),
    ("v2-text-text-code-accent", "var(--v2-blue-400)"),
    ("v2-border-border-muted", "var(--v2-alpha-light-8)"),
    ("v2-border-border-base", "var(--v2-alpha-light-10)"),
    ("v2-border-border-strong", "var(--v2-alpha-light-20)"),
    ("v2-border-border-inverse", "var(--v2-grey-100)"),
    ("v2-border-border-focus", "var(--v2-blue-500)"),
    ("v2-overlay-simple-overlay-hover", "var(--v2-alpha-light-6)"),
    (
        "v2-overlay-simple-overlay-pressed",
        "var(--v2-alpha-light-10)",
    ),
    (
        "v2-overlay-simple-overlay-contrast-hover",
        "var(--v2-alpha-dark-24)",
    ),
    (
        "v2-overlay-simple-overlay-contrast-pressed",
        "var(--v2-alpha-dark-40)",
    ),
    ("v2-overlay-simple-overlay-scrim", "var(--v2-alpha-dark-60)"),
    (
        "v2-overlay-gradient-depth-overlay-depth-top",
        "var(--v2-alpha-light-100)",
    ),
    (
        "v2-overlay-gradient-depth-overlay-depth-bot",
        "var(--v2-alpha-light-0)",
    ),
    ("v2-overlay-simple-tab-active-scrim", "#24242400"),
    ("v2-overlay-simple-tab-hover-scrim", "#3a3a3a00"),
    ("v2-overlay-simple-tab-scrim", "#08080800"),
    ("v2-state-bg-success", "var(--v2-green-1200)"),
    ("v2-state-fg-success", "var(--v2-green-500)"),
    ("v2-state-border-success", "var(--v2-green-900)"),
    ("v2-state-bg-warning", "var(--v2-yellow-1200)"),
    ("v2-state-fg-warning", "var(--v2-yellow-500)"),
    ("v2-state-border-warning", "var(--v2-yellow-900)"),
    ("v2-state-bg-danger", "var(--v2-red-1200)"),
    ("v2-state-fg-danger", "var(--v2-red-500)"),
    ("v2-state-border-danger", "var(--v2-red-900)"),
    ("v2-state-bg-info", "var(--v2-blue-1200)"),
    ("v2-state-fg-info", "var(--v2-blue-500)"),
    ("v2-state-border-info", "var(--v2-blue-900)"),
];

/// 1:1 with TS `light` elevation + illustration tail (entries after the avatar spread).
pub const LIGHT_TOKENS_TAIL: &[(&str, &str)] = &[
    ("v2-elevation-raised", "0px 2px 4px 0px var(--v2-alpha-dark-4), 0px 1px 2px -1px var(--v2-alpha-dark-8), 0px 0px 0px 0.5px var(--v2-alpha-dark-12), 0px 0px 0px 0px var(--v2-alpha-dark-0)"),
    ("v2-elevation-floating", "0px 8px 16px 0px var(--v2-alpha-dark-4), 0px 4px 8px 0px var(--v2-alpha-dark-8), 0px 0px 0px 0.5px var(--v2-alpha-dark-12), 0px 0px 0px 0px var(--v2-alpha-dark-0)"),
    ("v2-elevation-overlay", "0px 16px 32px 0px var(--v2-alpha-dark-4), 0px 8px 16px 0px var(--v2-alpha-dark-8), 0px 0px 0px 0.5px var(--v2-alpha-dark-12), 0px 0px 0px 0px var(--v2-alpha-dark-0)"),
    ("v2-elevation-button-neutral", "0px 1px 1.5px 0px var(--v2-alpha-dark-10), 0px 0px 0px 0.5px var(--v2-alpha-dark-14), 0px 0px 0px 0px var(--v2-alpha-dark-0)"),
    ("v2-elevation-button-contrast", "0px 1px 1.5px 0px var(--v2-alpha-dark-20), 0px 0px 0px 0.5px var(--v2-grey-800), inset 0px 1px 2px 0px var(--v2-alpha-light-14), inset 0px -1px 2px 0px var(--v2-alpha-dark-6), 0px 0px 0px 0px var(--v2-alpha-dark-0)"),
    ("v2-elevation-elements", "0px 0.5px 0.5px 0px var(--v2-alpha-dark-40)"),
    ("v2-elevation-switch-off", "inset 0px 1px 1px 0px var(--v2-alpha-dark-8), inset 0px 0.5px 0.5px 0px var(--v2-alpha-dark-8), inset 0px 0px 0px 0.5px var(--v2-alpha-dark-10)"),
    ("v2-elevation-switch-on", "inset 0px 2px 2px 0px var(--v2-alpha-dark-10), inset 0px 1px 1px 0px var(--v2-alpha-dark-10), inset 0px 0px 0px 0.5px var(--v2-alpha-dark-20)"),
    ("v2-illustration-illustration-layer-01", "var(--v2-grey-300)"),
    ("v2-illustration-illustration-layer-02", "var(--v2-grey-400)"),
    ("v2-illustration-illustration-layer-03", "var(--v2-grey-500)"),
];

/// 1:1 with TS `dark` elevation + illustration tail (entries after the avatar spread).
pub const DARK_TOKENS_TAIL: &[(&str, &str)] = &[
    ("v2-elevation-raised", "0px 2px 4px 0px var(--v2-alpha-dark-30), 0px 1px 2px 0px var(--v2-alpha-dark-30), 0px 0px 0px 0.5px var(--v2-alpha-light-16), 0px -0.5px 0px 0px var(--v2-alpha-light-6)"),
    ("v2-elevation-floating", "0px 8px 16px 0px var(--v2-alpha-dark-30), 0px 4px 8px 0px var(--v2-alpha-dark-30), 0px 0px 0px 0.5px var(--v2-alpha-light-16), 0px -0.5px 0px 0px var(--v2-alpha-light-6)"),
    ("v2-elevation-overlay", "0px 16px 32px 0px var(--v2-alpha-dark-30), 0px 8px 16px 0px var(--v2-alpha-dark-30), 0px 0px 0px 0.5px var(--v2-alpha-light-16), 0px -0.5px 0px 0px var(--v2-alpha-light-6)"),
    ("v2-elevation-button-neutral", "0px 1px 2px 0px var(--v2-alpha-dark-40), 0px 0px 0px 0.5px var(--v2-alpha-light-20), 0px -0.5px 0px 0px var(--v2-alpha-light-10)"),
    ("v2-elevation-button-contrast", "0px 1px 2px 0px var(--v2-alpha-dark-40), 0px 0px 0px 0.5px var(--v2-alpha-light-40), inset 0px 0px 0px 0px var(--v2-alpha-light-0), inset 0px 0px 0px 0px var(--v2-alpha-light-0), 0px -0.5px 0px 0px var(--v2-alpha-light-30)"),
    ("v2-elevation-elements", "0px 0.5px 0.5px 0px var(--v2-alpha-dark-40)"),
    ("v2-elevation-switch-off", "inset 0px -0.5px 0px 0px var(--v2-alpha-light-10), inset 0px 0px 0px 0px var(--v2-alpha-light-0), inset 0px 0px 0px 0.5px var(--v2-alpha-light-16)"),
    ("v2-elevation-switch-on", "inset 0px -0.5px 0px 0px var(--v2-alpha-light-10), inset 0px 0px 0px 0px var(--v2-alpha-light-0), inset 0px 0px 0px 0.5px var(--v2-alpha-light-16)"),
    ("v2-illustration-illustration-layer-01", "var(--v2-grey-900)"),
    ("v2-illustration-illustration-layer-02", "var(--v2-grey-800)"),
    ("v2-illustration-illustration-layer-03", "var(--v2-grey-700)"),
];

/// Entry count of the light semantics tables (head + agents + avatar + tail).
pub const LIGHT_SEMANTICS_COUNT: usize =
    LIGHT_TOKENS_HEAD.len() + LIGHT_AGENT_TOKENS.len() + 19 + LIGHT_TOKENS_TAIL.len();

/// Entry count of the dark semantics tables (head + agents + avatar + tail).
pub const DARK_SEMANTICS_COUNT: usize =
    DARK_TOKENS_HEAD.len() + DARK_AGENT_TOKENS.len() + 19 + DARK_TOKENS_TAIL.len();

/// 1:1 with TS `ref` — build a `var(--name)` color ref.
pub fn var_ref(name: &str) -> String {
    format!("var(--{name})")
}

/// 1:1 with TS `mapV2Semantics` (spread order: base, agent tokens, avatar, elevation tail).
pub fn map_v2_semantics(is_dark: bool) -> std::collections::BTreeMap<String, String> {
    let mut out: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    let extend = |out: &mut std::collections::BTreeMap<String, String>, rows: &[(&str, &str)]| {
        for (k, v) in rows {
            out.insert(k.to_string(), v.to_string());
        }
    };
    if is_dark {
        extend(&mut out, DARK_TOKENS_HEAD);
        extend(&mut out, DARK_AGENT_TOKENS);
        out.extend(avatar_dark());
        extend(&mut out, DARK_TOKENS_TAIL);
    } else {
        extend(&mut out, LIGHT_TOKENS_HEAD);
        extend(&mut out, LIGHT_AGENT_TOKENS);
        out.extend(avatar_light());
        extend(&mut out, LIGHT_TOKENS_TAIL);
    }
    out
}

/// 1:1 with TS `mergeV2Tokens` (`Object.assign({}, ...layers)` — later layers win).
pub fn merge_v2_tokens(
    layers: &[std::collections::BTreeMap<String, String>],
) -> std::collections::BTreeMap<String, String> {
    let mut out = std::collections::BTreeMap::new();
    for layer in layers {
        out.extend(layer.iter().map(|(k, v)| (k.clone(), v.clone())));
    }
    out
}
