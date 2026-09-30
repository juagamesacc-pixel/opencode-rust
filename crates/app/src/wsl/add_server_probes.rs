//! Rust port of `packages/app/src/wsl/add-server-probes.ts` (opencode v1.18.30).
//!
//! Source 57 lines: `useWslAddServerProbes` (probe mutation + gate +
//! `addServerProbePlan` effect). Tanstack-query/SolidJS wiring is
//! PROVISIONAL; plan-key/gate semantics preserved.
//! Original file: `packages/app/src/wsl/add-server-probes.ts`

#![allow(dead_code)]

use crate::wsl::settings_model::AddServerProbePlan;

/// Mirrors the `useWslAddServerProbes` plan input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddServerProbeInput {
    pub view_is_catalog: bool,
    pub adding: bool,
    pub busy: bool,
    pub selected_distro: Option<String>,
    pub probing_pending: bool,
}

// PROVISIONAL: pending tanstack-query/solid-js — mirrors `packages/app/src/wsl/add-server-probes.ts`.
#[derive(Debug, Default)]
pub struct AddServerProbes {
    pub probing_addable: bool,
}

impl AddServerProbes {
    pub fn new() -> Self {
        Self::default()
    }

    /// Mirrors the `probingAddable` selector.
    pub fn update_probing(&mut self, pending: bool, kind_is_addable: bool) {
        self.probing_addable = pending && kind_is_addable;
    }

    pub fn transition_reset(&mut self) {
        self.probing_addable = false;
    }

    pub fn describe_plan(plan: &AddServerProbePlan) -> &'static str {
        match plan {
            AddServerProbePlan::Auto { .. } => "auto",
            AddServerProbePlan::Addable { .. } => "addable",
        }
    }

    pub fn describe_input(input: &AddServerProbeInput) -> String {
        format!(
            "view_catalog={} adding={} busy={} selected={:?}",
            input.view_is_catalog, input.adding, input.busy, input.selected_distro
        )
    }
}
