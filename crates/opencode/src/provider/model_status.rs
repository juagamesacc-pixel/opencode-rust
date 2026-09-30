// source: src/provider/model-status.ts — exports: CatalogModelStatus,
// ModelStatus, ProviderModelStatus
// PROVISIONAL pending crates/core (models-dev): status literals verbatim.

/// source: ModelStatus = ["alpha", "beta", "deprecated", "active"] — verbatim order.
pub const MODEL_STATUSES: &[&str] = &["alpha", "beta", "deprecated", "active"];

/// source: default status "active" (fromModelsDevModel `model.status ?? "active"`) — verbatim.
pub const DEFAULT_STATUS: &str = "active";
