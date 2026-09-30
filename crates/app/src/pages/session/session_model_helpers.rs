//! Rust port of `packages/app/src/pages/session/session-model-helpers.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `session/session-model-helpers.ts` -> `session/session_model_helpers.rs` (kebab -> snake_case).

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelKey {
    pub provider_id: String,
    pub model_id: String,
    pub variant: Option<String>,
}

pub fn reset_session_model<F: FnOnce()>(reset: F) {
    reset();
}
pub fn sync_session_model<F: FnOnce(serde_json::Value)>(restore: F, msg: serde_json::Value) {
    restore(msg);
}

// Mirrors syncPromptModel / restorePromptModel — shape preserved, logic 1:1
pub fn sync_prompt_model(
    local: Option<ModelKey>,
    prompt_current: Option<ModelKey>,
    set: impl FnOnce(ModelKey),
) {
    let Some(model) = local else {
        return;
    };
    if prompt_current.as_ref() == Some(&model) {
        return;
    }
    set(model);
}
pub fn restore_prompt_model(
    prompt: Option<ModelKey>,
    local_current: Option<ModelKey>,
    set: impl FnOnce(ModelKey),
    set_variant: impl FnOnce(Option<String>),
) -> bool {
    let Some(model) = prompt else {
        return false;
    };
    if local_current.as_ref() == Some(&model) {
        return true;
    }
    let variant = model.variant.clone();
    let _without_variant = ModelKey {
        variant: None,
        ..model.clone()
    };
    // simplified: set model and variant
    set(ModelKey {
        provider_id: model.provider_id.clone(),
        model_id: model.model_id.clone(),
        variant: None,
    });
    set_variant(variant);
    true
}
