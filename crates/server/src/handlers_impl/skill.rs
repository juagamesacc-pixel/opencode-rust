//! Rust port of `packages/server/src/handlers/skill.ts` (opencode v1.18.30).
//!
//! Source 8 lines: `SkillHandler.handle("skill.list", ()=>response(SkillV2.Service.use(skill=>skill.list())))`
//!
//! PROVISIONAL: `SkillV2.Service` pending `crates/core`.

pub const GROUP: &str = "server.skill";
pub const OPERATION: &str = "skill.list";
pub const USES_LOCATION_RESPONSE: bool = true;
pub const SERVICE_ID: &str = "@opencode/SkillV2";
