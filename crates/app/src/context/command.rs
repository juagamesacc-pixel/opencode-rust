//! Rust port of `packages/app/src/context/command.tsx` (opencode v1.18.30).
//!
//! Source 477 lines. Exports: `DEFAULT_PALETTE_KEYBIND`, `KeybindConfig`, `Keybind`, `CommandOption`, `commandPaletteOptions`, `resolveKeybindOption`, `CommandCatalogItem`, `CommandRegistration`, `addCommandRegistration`, `activeCommandRegistrations`, `parseKeybind`, `matchKeybind`, `formatKeybindParts`, `formatKeybind`, `formatKeybindKeys`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/command.tsx`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/command.tsx

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

pub const DEFAULT_PALETTE_KEYBIND: &str = "mod+k,mod+shift+p";

/// Mirrors `KeybindConfig`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeybindConfig {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `Keybind`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Keybind {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `CommandOption`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandOption {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `commandPaletteOptions`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/command.tsx
#[allow(non_snake_case)]
pub fn commandPaletteOptions(/* options: CommandOption[] */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `resolveKeybindOption`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/command.tsx
#[allow(non_snake_case)]
pub fn resolveKeybindOption(/* candidates: CommandOption[] | undefined, event: KeyboardEvent */
) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `CommandCatalogItem`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandCatalogItem {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `CommandRegistration`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandRegistration {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `addCommandRegistration`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/command.tsx
#[allow(non_snake_case)]
pub fn addCommandRegistration(/* registrations: CommandRegistration[], entry: CommandRegistration */
) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `activeCommandRegistrations`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/command.tsx
#[allow(non_snake_case)]
pub fn activeCommandRegistrations(/* registrations: CommandRegistration[] */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `parseKeybind`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/command.tsx
#[allow(non_snake_case)]
pub fn parseKeybind(/* config: string */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `matchKeybind`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/command.tsx
#[allow(non_snake_case)]
pub fn matchKeybind(/* keybinds: Keybind[], event: KeyboardEvent */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `formatKeybindParts`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/command.tsx
#[allow(non_snake_case)]
pub fn formatKeybindParts(/* config: string, t?: (key: KeyLabel */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `formatKeybind`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/command.tsx
#[allow(non_snake_case)]
pub fn formatKeybind(/* config: string, t?: (key: KeyLabel */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `formatKeybindKeys`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/command.tsx
#[allow(non_snake_case)]
pub fn formatKeybindKeys(/* config: string, t?: (key: KeyLabel */) -> serde_json::Value {
    serde_json::json!({})
}
