//! Rust port of `@opencode-ai/ui` (opencode v1.18.30).
//!
//! 1:1 module tree mirroring `packages/ui/src/**`:
//! `assets`, `components`, `context`, `hooks`, `i18n`, `storybook`, `styles`, `theme`, `v2`, `script`.
#![allow(non_snake_case, dead_code)] // 1:1 source names/fields preserved, unused mirrors source exports
#![allow(non_upper_case_globals, non_camel_case_types)] // 1:1 source names like oc2Theme, V2_PRIMITIVES_DEFAULT stay verbatim
#![allow(clippy::invisible_characters)] // i18n strings byte-identical to source translations
#![allow(clippy::doc_lazy_continuation)] // doc comments mirror source text
#![allow(clippy::module_inception)] // mod.rs structure mirrors source dirs 1:1
#![allow(clippy::tabs_in_doc_comments)] // doc bytes preserved
pub mod assets;
pub mod components;
pub mod context;
pub mod hooks;
pub mod i18n;
pub mod script;
pub mod storybook;
pub mod styles;
pub mod theme;
pub mod v2;
