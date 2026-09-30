// source: src/plugin/modal/models.ts — exports: get, ModalModels
// (`${baseURL trailing-slash-strip}/models`, effort default "none";
// PROVISIONAL: fetch as trait). Key rules verbatim.

/// source: models path — verbatim suffix after trailing-slash strip.
pub const MODELS_SUFFIX: &str = "/models";
/// source: effort default "none" — verbatim.
pub const EFFORT_DEFAULT: &str = "none";

/// source: price() — Number(value) else fallback. Verbatim rule.
pub fn price(value: Option<f64>, fallback: f64) -> f64 {
    value.unwrap_or(fallback)
}
