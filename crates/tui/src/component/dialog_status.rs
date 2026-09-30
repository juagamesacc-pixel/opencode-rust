// source: packages/tui/src/component/dialog-status.tsx (168 lines, v1.18.30)
// 1:1 port — MCP/LSP/formatter/plugin sections with verbatim copy and
// status colors; plugin spec parsing (`file://` names, `name@version`).

#![allow(dead_code)]

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget, Wrap};
use serde_json::Value;

use super::super::ui::link::rgba;
use crate::theme::{Rgba, Theme};

/// Parsed plugin display entry.
#[derive(Debug, Clone)]
pub struct PluginDisplay {
    pub name: String,
    pub version: Option<String>,
}

/// Mirrors the plugin-spec mapping (`file://` basenames, `name@version`).
pub fn parse_plugin_specs(specs: &[Value]) -> Vec<PluginDisplay> {
    let mut out: Vec<PluginDisplay> = specs
        .iter()
        .map(|item| {
            let value = match item {
                Value::String(s) => s.clone(),
                Value::Array(pair) => pair
                    .first()
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                _ => String::new(),
            };
            if let Some(path) = value.strip_prefix("file://") {
                let path = path.strip_prefix("file://").unwrap_or(path);
                let parts: Vec<&str> = path.split('/').collect();
                let filename = parts.last().copied().unwrap_or(path).to_string();
                if !filename.contains('.') {
                    return PluginDisplay {
                        name: filename,
                        version: None,
                    };
                }
                let basename = filename.split('.').next().unwrap_or(&filename).to_string();
                if basename == "index" {
                    let name = parts
                        .iter()
                        .rev()
                        .nth(1)
                        .copied()
                        .unwrap_or(&basename)
                        .to_string();
                    return PluginDisplay {
                        name,
                        version: None,
                    };
                }
                return PluginDisplay {
                    name: basename,
                    version: None,
                };
            }
            match value.rfind('@') {
                Some(index) if index > 0 => PluginDisplay {
                    name: value[..index].to_string(),
                    version: Some(value[index + 1..].to_string()),
                },
                _ => PluginDisplay {
                    name: value,
                    version: Some("latest".to_string()),
                },
            }
        })
        .collect();
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

/// MCP status dot color + label verbatim.
pub fn mcp_status_label(status: &str, error: Option<&str>, key: &str) -> (Rgba, String) {
    // Colors resolved by the caller; this maps status → label text.
    let _ = key;
    match status {
        "connected" => (
            Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            "Connected".to_string(),
        ),
        "failed" => (
            Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            error.unwrap_or(status).to_string(),
        ),
        "disabled" => (
            Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            "Disabled in configuration".to_string(),
        ),
        "needs_auth" => (
            Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            format!("Needs authentication (run: opencode mcp auth {key})"),
        ),
        "needs_client_registration" => (
            Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            error.unwrap_or(status).to_string(),
        ),
        _ => (
            Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            status.to_string(),
        ),
    }
}

/// Status dialog snapshot (pulled from sync by the app).
#[derive(Debug, Clone, Default)]
pub struct StatusSnapshot {
    pub mcp: Vec<(String, Value)>,
    pub lsp: Vec<Value>,
    pub formatters: Vec<Value>,
    pub plugins: Vec<PluginDisplay>,
}

/// Status dialog render (sections with counts + bullets, verbatim copy).
pub fn render_status(snapshot: &StatusSnapshot, theme: &Theme, area: Rect, buf: &mut Buffer) {
    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(vec![
        Span::styled(
            "Status",
            Style::default()
                .fg(rgba(theme.text))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  esc", Style::default().fg(rgba(theme.text_muted))),
    ]));
    lines.push(Line::from(""));
    if snapshot.mcp.is_empty() {
        lines.push(Line::styled(
            "No MCP Servers",
            Style::default().fg(rgba(theme.text)),
        ));
    } else {
        lines.push(Line::styled(
            format!("{} MCP Servers", snapshot.mcp.len()),
            Style::default().fg(rgba(theme.text)),
        ));
        for (key, item) in &snapshot.mcp {
            let status = item.get("status").and_then(|v| v.as_str()).unwrap_or("");
            let dot = match status {
                "connected" => theme.success,
                "failed" => theme.error,
                "disabled" => theme.text_muted,
                "needs_auth" => theme.warning,
                "needs_client_registration" => theme.error,
                _ => theme.text_muted,
            };
            let (_, label) =
                mcp_status_label(status, item.get("error").and_then(|v| v.as_str()), key);
            lines.push(Line::from(vec![
                Span::styled("• ", Style::default().fg(rgba(dot))),
                Span::styled(
                    key.clone(),
                    Style::default()
                        .fg(rgba(theme.text))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" {label}"),
                    Style::default().fg(rgba(theme.text_muted)),
                ),
            ]));
        }
    }
    if !snapshot.lsp.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::styled(
            format!("{} LSP Servers", snapshot.lsp.len()),
            Style::default().fg(rgba(theme.text)),
        ));
        for item in &snapshot.lsp {
            let dot = if item.get("status").and_then(|v| v.as_str()) == Some("connected") {
                theme.success
            } else {
                theme.error
            };
            lines.push(Line::from(vec![
                Span::styled("• ", Style::default().fg(rgba(dot))),
                Span::styled(
                    item.get("id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    Style::default()
                        .fg(rgba(theme.text))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(
                        " {}",
                        item.get("root").and_then(|v| v.as_str()).unwrap_or("")
                    ),
                    Style::default().fg(rgba(theme.text_muted)),
                ),
            ]));
        }
    }
    if snapshot.formatters.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::styled(
            "No Formatters",
            Style::default().fg(rgba(theme.text)),
        ));
    } else {
        lines.push(Line::from(""));
        lines.push(Line::styled(
            format!("{} Formatters", snapshot.formatters.len()),
            Style::default().fg(rgba(theme.text)),
        ));
        for item in &snapshot.formatters {
            lines.push(Line::from(vec![
                Span::styled("• ", Style::default().fg(rgba(theme.success))),
                Span::styled(
                    item.get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    Style::default()
                        .fg(rgba(theme.text))
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        }
    }
    if snapshot.plugins.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::styled(
            "No Plugins",
            Style::default().fg(rgba(theme.text)),
        ));
    } else {
        lines.push(Line::from(""));
        lines.push(Line::styled(
            format!("{} Plugins", snapshot.plugins.len()),
            Style::default().fg(rgba(theme.text)),
        ));
        for plugin in &snapshot.plugins {
            let mut spans = vec![
                Span::styled("• ", Style::default().fg(rgba(theme.success))),
                Span::styled(
                    plugin.name.clone(),
                    Style::default()
                        .fg(rgba(theme.text))
                        .add_modifier(Modifier::BOLD),
                ),
            ];
            if let Some(version) = plugin.version.as_ref() {
                spans.push(Span::styled(
                    format!(" @{version}"),
                    Style::default().fg(rgba(theme.text_muted)),
                ));
            }
            lines.push(Line::from(spans));
        }
    }
    Paragraph::new(lines).wrap(Wrap { trim: false }).render(
        Rect {
            x: area.x + 2,
            y: area.y,
            width: area.width.saturating_sub(4),
            height: area.height,
        },
        buf,
    );
}
