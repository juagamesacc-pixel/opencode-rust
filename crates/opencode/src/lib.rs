//! opencode v1.18.30 @3104c14, Bun 1.3.14/TS 5.8.2 → Rust 1.98.1
//! 1:1 port of `packages/opencode`. Source is spec; no new/removed items.
//! Barrel mirrors src/index.ts order (CLI entry: no library exports; modules
//! below follow src/ subdir lexical order = `find src -maxdepth 1 -type d | sort`).
//!
//! NOTED-NOT-PORTED (build-tool shims, no Rust equivalent — cf. server
//! sst-env.d.ts precedent): src/audio.d.ts, src/markdown.d.ts, src/sql.d.ts.
#![allow(dead_code)]
#![allow(non_snake_case)]
// 1:1 fidelity: source names like RUN_ENTRY_NONE, formatAccountLabel, node/use/select stay verbatim
#![allow(non_upper_case_globals, non_camel_case_types)]
// doc comments mirror source text verbatim
#![allow(clippy::doc_lazy_continuation)]
// source dirs mirror 1:1 (e.g., account/mod.rs, agent/mod.rs, cli/cmd/mod.rs, etc.)
#![allow(clippy::module_inception)]
// doc bytes preserved verbatim (cli/effect/prompt.rs:5)
#![allow(clippy::tabs_in_doc_comments)]
pub mod account; // src/account
pub mod acp; // src/acp
pub mod agent; // src/agent
pub mod auth; // src/auth
pub mod background; // src/background
pub mod bus; // src/bus
pub mod cli; // src/cli
pub mod command; // src/command
pub mod config; // src/config
pub mod control_plane; // src/control-plane
pub mod core_provisional; // PROVISIONAL stubs for @opencode-ai/core/* (unported); see inventory
pub mod effect; // src/effect
pub mod env; // src/env
pub mod event_manifest; // src/event-manifest.ts root file
pub mod event_v2_bridge; // src/event-v2-bridge.ts root file
pub mod format; // src/format
pub mod git; // src/git
pub mod id; // src/id
pub mod ide; // src/ide
pub mod image; // src/image
pub mod installation; // src/installation
pub mod lsp; // src/lsp
pub mod mcp; // src/mcp
pub mod node; // src/node.ts root file
pub mod permission; // src/permission
pub mod plugin; // src/plugin
pub mod project; // src/project
pub mod provider; // src/provider
pub mod question; // src/question
pub mod server; // src/server
pub mod session; // src/session
pub mod share; // src/share
pub mod skill; // src/skill
pub mod snapshot; // src/snapshot
pub mod storage; // src/storage
pub mod sync; // src/sync
pub mod temporary; // src/temporary.ts root file
pub mod tool; // src/tool
pub mod util; // src/util
pub mod worktree; // src/worktree
