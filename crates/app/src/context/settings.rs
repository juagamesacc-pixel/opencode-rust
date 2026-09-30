//! Rust port of `packages/app/src/context/settings.tsx` (opencode v1.18.30).
//!
//! Source 548 lines. Exports: `NotificationSettings`, `SoundSettings`, `Settings`, `monoDefault`, `sansDefault`, `terminalDefault`, `newLayoutDesignsDefault`, `oldInterfaceSunset`, `isAppUpgrade`, `shouldDisplayTabsToast`, `hasExistingWebState`, `initialAgentVisibility`, `shouldEnableNewLayout`, `layoutTransitionState`, `maximumSunsetTimeout`, `nextSunsetCheckDelay`, `resolveNewLayoutDesigns`, `monoInput`, `sansInput`, `monoFontFamily`, `sansFontFamily`, `terminalInput`, `terminalFontFamily`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/settings.tsx`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/settings.tsx

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `NotificationSettings`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationSettings {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `SoundSettings`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoundSettings {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `Settings`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    // PROVISIONAL: fields pending full port
}

pub const monoDefault: &str = "System Mono";

pub const sansDefault: &str = "System Sans";

pub const terminalDefault: &str = "JetBrainsMono Nerd Font Mono";

/// Mirrors `newLayoutDesignsDefault`.
pub fn newLayoutDesignsDefault_value() -> String {
    String::new()
}

/// Mirrors `oldInterfaceSunset`.
pub fn oldInterfaceSunset_value() -> String {
    String::new()
}

/// Mirrors `isAppUpgrade`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/settings.tsx
#[allow(non_snake_case)]
pub fn isAppUpgrade(/* previous: string | undefined, current: string | undefined */
) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `shouldDisplayTabsToast`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/settings.tsx
#[allow(non_snake_case)]
pub fn shouldDisplayTabsToast(/* 
  previous: string | undefined,
  current: string | undefined,
  existingInstall: boolean,
 */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `hasExistingWebState`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/settings.tsx
#[allow(non_snake_case)]
pub fn hasExistingWebState(/* settings: Promise<string> | string | null, previousVersion: string | undefined */
) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `initialAgentVisibility`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/settings.tsx
#[allow(non_snake_case)]
pub fn initialAgentVisibility(/* initialized: boolean | undefined, existing: boolean, previousVersion?: string */
) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `shouldEnableNewLayout`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/settings.tsx
#[allow(non_snake_case)]
pub fn shouldEnableNewLayout(/* previous: string | undefined, current: string | undefined */
) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `layoutTransitionState`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/settings.tsx
#[allow(non_snake_case)]
pub fn layoutTransitionState(/* scheduled: boolean, eligible: boolean, retired: boolean, dismissed: boolean */
) -> serde_json::Value {
    serde_json::json!({})
}

pub const maximumSunsetTimeout: i64 = 2_147_483_647;

/// Mirrors `nextSunsetCheckDelay`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/settings.tsx
#[allow(non_snake_case)]
pub fn nextSunsetCheckDelay(/* sunset: number, now: number */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `resolveNewLayoutDesigns`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/settings.tsx
#[allow(non_snake_case)]
pub fn resolveNewLayoutDesigns(/* retired: boolean, preference: boolean | undefined, fallback = true */
) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `monoInput`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/settings.tsx
#[allow(non_snake_case)]
pub fn monoInput(/* font: string | undefined */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `sansInput`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/settings.tsx
#[allow(non_snake_case)]
pub fn sansInput(/* font: string | undefined */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `monoFontFamily`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/settings.tsx
#[allow(non_snake_case)]
pub fn monoFontFamily(/* font: string | undefined */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `sansFontFamily`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/settings.tsx
#[allow(non_snake_case)]
pub fn sansFontFamily(/* font: string | undefined */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `terminalInput`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/settings.tsx
#[allow(non_snake_case)]
pub fn terminalInput(/* font: string | undefined */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `terminalFontFamily`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/settings.tsx
#[allow(non_snake_case)]
pub fn terminalFontFamily(/* font: string | undefined */) -> serde_json::Value {
    serde_json::json!({})
}
