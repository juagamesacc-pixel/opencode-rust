//! Port of packages/app/src/components/prompt-input.stories.tsx
//! ——— 1:1 exact clone, zero diversion ———
//! Rename log: `prompt-input.stories.tsx` → `prompt_input.stories.tsx` (Rust identifier snake_case).
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// PROVISIONAL: pending storybook — mirrors packages/app/src/components/prompt-input.stories.tsx
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryMeta {
    pub title: &'static str,
}
pub const META: StoryMeta = StoryMeta {
    title: "packages/app/src/components/prompt-input.stories.tsx",
};
