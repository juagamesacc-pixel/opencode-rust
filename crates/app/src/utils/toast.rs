//! Rust port of `packages/app/src/utils/toast.tsx` (opencode v1.18.30).
//!
//! Source 47 lines: `setV2Toast`, `ToastRegion`, `showToast`,
//! `dismissToast` (V1/V2 registry note preserved). UI rendering is
//! PROVISIONAL; registry selection is verbatim.
//! Original file: `packages/app/src/utils/toast.tsx`

#![allow(dead_code)]

// PROVISIONAL: pending ui toast crates — mirrors `packages/app/src/utils/toast.tsx`.
/// Mirrors the `v2` registry flag.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ToastRegistry {
    pub v2: bool,
}

impl ToastRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Mirrors `setV2Toast(value)`.
    pub fn update_set_v2(&mut self, value: bool) {
        self.v2 = value;
    }

    /// Mirrors the `ToastRegion({ v2 })` selection.
    pub fn transition_region(&self, v2: bool) -> &'static str {
        if v2 {
            "ToastV2.Region"
        } else {
            "Toast.Region"
        }
    }

    /// Mirrors the `showToast` dispatch registry (`legacy` vs `v2`).
    pub fn transition_show_registry(&self) -> &'static str {
        if self.v2 {
            "showToastV2"
        } else {
            "showLegacyToast"
        }
    }

    /// Mirrors the `dismissToast` registry note (ids are registry-scoped).
    pub fn transition_dismiss_registry(&self) -> &'static str {
        if self.v2 {
            "toasterV2.dismiss"
        } else {
            "legacyToaster.dismiss"
        }
    }
}
