//! Rust port of `packages/core/src/tool/grep.ts`.

use serde::{Deserialize, Serialize};

pub const NAME: &str = "grep";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub pattern: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchEntry {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub entry: MatchEntry,
    pub line: u64,
    pub text: String,
}

pub fn to_model_output(output: &[Match]) -> String {
    let mut lines: Vec<String> = if output.is_empty() {
        vec!["No files found".to_string()]
    } else {
        vec![format!("Found {} matches", output.len())]
    };
    let mut current = String::new();
    for m in output {
        if current != m.entry.path {
            if !current.is_empty() {
                lines.push(String::new());
            }
            current = m.entry.path.clone();
            lines.push(format!("{}:", m.entry.path));
        }
        lines.push(format!("  Line {}: {}", m.line, m.text));
    }
    lines.join("\n")
}

// PROVISIONAL pending effect/runtime — Layer wiring requires FSUtil + Ripgrep + Location + PermissionV2.
