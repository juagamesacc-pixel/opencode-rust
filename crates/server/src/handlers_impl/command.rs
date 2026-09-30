//! Rust port of `packages/server/src/handlers/command.ts` (opencode v1.18.30).
//!
//! Source 8 lines: `CommandHandler = group(Api, "server.command", h=>h.handle("command.list", ()=>response(CommandV2.Service.use(cmd=>cmd.list()))))`
//!
//! PROVISIONAL: `CommandV2.Service` pending `crates/core`.

pub const GROUP: &str = "server.command";
pub const OPERATION: &str = "command.list";
pub const USES_LOCATION_RESPONSE: bool = true;
pub const SERVICE_ID: &str = "@opencode/CommandV2";
