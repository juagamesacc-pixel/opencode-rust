//! Port of packages/app/src/components/settings-v2/interface-transition.stories.tsx
//! ——— 1:1 exact clone, zero diversion ———
//! Rename log: `settings-v2` → `settings_v2` (Rust identifier snake_case).
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// PROVISIONAL: pending storybook — mirrors packages/app/src/components/settings-v2/interface-transition.stories.tsx
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryMeta {
    pub title: &'static str,
}
pub const META: StoryMeta = StoryMeta {
    title: "packages/app/src/components/settings-v2/interface-transition.stories.tsx",
};
