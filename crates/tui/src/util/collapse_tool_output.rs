// source: packages/tui/src/util/collapse-tool-output.ts (19 lines, v1.18.30)
// 1:1 port — line/char clamping with the verbatim `…` markers.

#![allow(dead_code)]

/// Result of `collapseToolOutput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollapsedOutput {
    pub output: String,
    pub overflow: bool,
}

/// Mirrors `collapseToolOutput`.
pub fn collapse_tool_output(output: &str, max_lines: usize, max_chars: usize) -> CollapsedOutput {
    let lines: Vec<&str> = output.split('\n').collect();
    let total_chars = output.chars().count();
    if lines.len() <= max_lines && total_chars <= max_chars {
        return CollapsedOutput {
            output: output.to_string(),
            overflow: false,
        };
    }
    let preview = lines
        .iter()
        .take(max_lines)
        .copied()
        .collect::<Vec<_>>()
        .join("\n");
    if preview.chars().count() > max_chars {
        let clamped: String = preview.chars().take(max_chars.saturating_sub(1)).collect();
        return CollapsedOutput {
            output: format!("{clamped}…"),
            overflow: true,
        };
    }
    let mut kept: Vec<&str> = lines.iter().take(max_lines).copied().collect();
    kept.push("…");
    CollapsedOutput {
        output: kept.join("\n"),
        overflow: true,
    }
}
