// source: src/plugin/github-copilot/models.ts — exports: schema, get,
// CopilotModels (endpoint select /v1/messages, usd-per-million pricing,
// max_thinking_budget variants, usable filter; PROVISIONAL: fetch as trait).
// Key rules verbatim.

/// source: usd-per-million — 10_000 / batch_size. Verbatim.
pub fn usd_per_million(batch_size: f64) -> f64 {
    if batch_size > 0.0 {
        10_000.0 / batch_size
    } else {
        0.0
    }
}

/// source: messages endpoint marker — verbatim.
pub const MESSAGES_ENDPOINT: &str = "/v1/messages";
