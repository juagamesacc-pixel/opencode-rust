//! Port of packages/app/src/components/dialog-connect-provider.stories.tsx
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// PROVISIONAL: pending storybook — mirrors packages/app/src/components/dialog-connect-provider.stories.tsx
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryMeta {
    pub title: &'static str,
}
pub const META: StoryMeta = StoryMeta {
    title: "packages/app/src/components/dialog-connect-provider.stories.tsx",
};
