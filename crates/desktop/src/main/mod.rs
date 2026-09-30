//! Rust port of `packages/desktop/src/main/` (opencode v1.18.30).
//!
//! Ambient declarations from `src/main/env.d.ts` (no runtime code in source):
//! - `ImportMetaEnv.OPENCODE_CHANNEL: string` → [`crate::main::constants::parse_channel`]
//!   input; read from the `OPENCODE_CHANNEL` environment variable.
//! - `declare module "virtual:opencode-server"` (`Server.listen` / `Server.Listener`,
//!   `Config.get` / `Config.Info`, `bootstrap`) →
//!   // PROVISIONAL(packages/desktop/src/main/env.d.ts): virtual Vite module
//!   resolved to `../../../opencode/dist/types/src/node` at bundle time; no
//!   in-workspace Rust binding exists. Mirrored as stubs in [`sidecar`] / [`server`].

pub mod apps;
pub mod attachment_picker;
pub mod background_cli;
pub mod constants;
pub mod debug;
pub mod desktop_menu_actions;
pub mod draft_store;
pub mod external_url;
pub mod index;
pub mod initialization;
pub mod install_state;
pub mod ipc;
pub mod logging;
pub mod menu;
pub mod migrate;
pub mod native_translations;
pub mod onboarding;
pub mod server;
pub mod shell_env;
pub mod sidecar;
pub mod store;
pub mod store_cleanup;
pub mod store_keys;
pub mod unresponsive;
pub mod updater;
pub mod updater_controller;
pub mod updater_subscriptions;
pub mod window_registry;
pub mod window_state;
pub mod windows;
pub mod wsl;
