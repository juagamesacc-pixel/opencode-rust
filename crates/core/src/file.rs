//! Rust port of `packages/core/src/file.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteInput {
    pub path: String,
    pub content: String,
}

pub fn preserve_bom(original: &[u8], text: &str) -> Vec<u8> {
    let has_bom =
        original.len() >= 3 && original[0] == 0xef && original[1] == 0xbb && original[2] == 0xbf;
    let mut out = Vec::new();
    if has_bom {
        out.extend_from_slice(&[0xef, 0xbb, 0xbf]);
    }
    out.extend_from_slice(text.as_bytes());
    out
}

// PROVISIONAL pending FileMutation service wiring.
