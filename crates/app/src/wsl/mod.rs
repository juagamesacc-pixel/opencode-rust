//! WSL modules — mirrors `packages/app/src/wsl/*` (opencode v1.18.30).
//!
//! Rename log (hyphen → underscore): `add-server-probes.ts` → `add_server_probes`,
//! `dialog-add-server.tsx` → `dialog_add_server`,
//! `dialog-add-wsl-server.css` → `dialog_add_wsl_server_css` (passthrough),
//! `settings-model.ts` → `settings_model`.

#![allow(dead_code)]

pub mod add_server_probes;
pub mod context;
pub mod dialog_add_server;
pub mod dialog_add_wsl_server_css;
pub mod settings;
pub mod settings_model;
pub mod types;
