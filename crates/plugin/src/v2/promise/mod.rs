// source: packages/plugin/src/v2/promise/index.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/promise/index.ts` (opencode v1.18.30).
//!
//! Source 12 lines: re-exports `PluginContext`, `PluginOptions`, `define`, `Plugin`, `PluginDomain`, `Registration`, `Reload`, `AgentDraft`, `AgentHooks`, `AISDKHooks`, `Catalog*`, `Command*`, `Integration*`, `Reference*`, `Skill*`.

pub mod agent;
pub mod aisdk;
pub mod catalog;
pub mod command;
pub mod context;
pub mod integration;
pub mod plugin;
pub mod reference;
pub mod registration;
pub mod skill;

pub use agent::{AgentDraft, AgentHooks};
pub use aisdk::AISDKHooks;
pub use catalog::{CatalogDraft, CatalogHooks, CatalogProviderRecord};
pub use command::{CommandDraft, CommandHooks};
pub use context::PluginContext;
pub use integration::{IntegrationDraft, IntegrationHooks, IntegrationMethodRegistration};
pub use plugin::{define, Plugin, PluginDomain};
pub use reference::{ReferenceDraft, ReferenceHooks};
pub use registration::{Registration, Reload};
pub use skill::{SkillDraft, SkillHooks};
