//! Rust port of `packages/app/src/pages/session/timeline/summary-diffs.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `session/timeline/summary-diffs.ts` -> `session/timeline/summary_diffs.rs` (kebab -> snake_case).

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SummaryDiff {
    pub file: String,
}

pub fn unique_summary_diffs(diffs: Option<&[SummaryDiff]>) -> Vec<SummaryDiff> {
    let input = match diffs {
        Some(v) => v,
        None => return vec![],
    };
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for diff in input.iter().rev() {
        if diff.file.is_empty() {
            continue;
        }
        if seen.contains(&diff.file) {
            continue;
        }
        seen.insert(diff.file.clone());
        result.push(diff.clone());
    }
    result.reverse();
    result
}
