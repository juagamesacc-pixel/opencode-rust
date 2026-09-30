//! Rust port of `packages/server/src/handlers/provider.ts` (opencode v1.18.30).
//!
//! Source 29 lines: `ProviderHandler` with `provider.list` (catalog.provider.available wrapped) and
//! `provider.get` (lookup -> ProviderNotFoundError "Provider not found: {id}").
//!
//! PROVISIONAL: `Catalog.Service` pending `crates/core`.

pub const GROUP: &str = "server.provider";
pub const OPERATIONS: &[&str] = &["provider.list", "provider.get"];
pub const SERVICE_ID: &str = "@opencode/Catalog";

pub fn not_found_message(provider_id: &str) -> String {
    format!("Provider not found: {provider_id}")
}
