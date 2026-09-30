// source: packages/tui/src/component/error-component.tsx (240 lines, v1.18.30)
// 1:1 port — crash screen with the verbatim fallback palettes, issue-URL
// builder (6000-char budget + binary-search truncation + marker), action
// row (copy/restart/quit) and the full keyboard map. Version and clipboard
// arrive via injection (core constants are not TUI dependencies).

#![allow(dead_code)]

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap};

use super::super::ui::link::rgba;
use crate::context::clipboard::ClipboardService;
use crate::theme::Rgba;
use crate::util::system::{describe_os, describe_terminal};

/// Issue template URL verbatim.
pub const ISSUE_URL_BASE: &str =
    "https://github.com/anomalyco/opencode/issues/new?template=bug-report.yml";
/// URL length budget verbatim.
pub const MAX_URL_LENGTH: usize = 6000;
/// Truncation marker verbatim.
pub const TRUNCATED_MARKER: &str = "\n… (truncated)";
/// Reproduce text verbatim.
pub const REPRODUCE_TEXT: &str =
    "Reported automatically from the opencode crash screen. If you can, describe what you were doing when it crashed.";

/// Fallback palette (mirrors the theme-asset values, theme-independent so
/// the screen works when theming itself crashed).
#[derive(Debug, Clone)]
pub struct CrashColors {
    pub bg: Rgba,
    pub element: Rgba,
    pub border_subtle: Rgba,
    pub text: Rgba,
    pub muted: Rgba,
    pub primary: Rgba,
    pub on_primary: Rgba,
    pub error: Rgba,
    pub success: Rgba,
}

pub fn crash_colors(light: bool) -> CrashColors {
    if light {
        CrashColors {
            bg: Rgba::from_hex("#ffffff"),
            element: Rgba::from_hex("#f5f5f5"),
            border_subtle: Rgba::from_hex("#d4d4d4"),
            text: Rgba::from_hex("#1a1a1a"),
            muted: Rgba::from_hex("#8a8a8a"),
            primary: Rgba::from_hex("#3b7dd8"),
            on_primary: Rgba::from_hex("#ffffff"),
            error: Rgba::from_hex("#d1383d"),
            success: Rgba::from_hex("#3d9a57"),
        }
    } else {
        CrashColors {
            bg: Rgba::from_hex("#0a0a0a"),
            element: Rgba::from_hex("#1e1e1e"),
            border_subtle: Rgba::from_hex("#3c3c3c"),
            text: Rgba::from_hex("#eeeeee"),
            muted: Rgba::from_hex("#808080"),
            primary: Rgba::from_hex("#fab283"),
            on_primary: Rgba::from_hex("#0a0a0a"),
            error: Rgba::from_hex("#e06c75"),
            success: Rgba::from_hex("#7fd88f"),
        }
    }
}

/// Mirrors `buildIssueURL` — field keys match the bug-report template.
pub fn build_issue_url(message: &str, stack: &str, version: &str) -> String {
    let mut params: Vec<(String, String)> = vec![
        ("title".to_string(), format!("TUI crash: {message}")),
        ("opencode-version".to_string(), version.to_string()),
        ("os".to_string(), describe_os()),
        ("terminal".to_string(), describe_terminal()),
        ("reproduce".to_string(), REPRODUCE_TEXT.to_string()),
    ];
    let head = format!("The opencode TUI crashed with an unexpected error.\n\n**Error:** {message}\n\n**Stack trace:**\n");
    let body = |stack_body: &str| format!("{head}```\n{stack_body}\n```");
    let render = |params: &[(String, String)]| {
        let query: Vec<String> = params
            .iter()
            .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
            .collect();
        format!("{ISSUE_URL_BASE}&{}", query.join("&"))
    };
    let set_body = |params: &mut Vec<(String, String)>, stack_body: &str| {
        if let Some(slot) = params.iter_mut().find(|(k, _)| k == "description") {
            slot.1 = body(stack_body);
        } else {
            params.push(("description".to_string(), body(stack_body)));
        }
    };
    set_body(&mut params, stack);
    if render(&params).len() <= MAX_URL_LENGTH {
        return render(&params);
    }
    // Largest raw stack prefix whose encoded URL (with marker) still fits.
    let mut lo = 0usize;
    let mut hi = stack.len();
    while lo < hi {
        let mid = (lo + hi).div_ceil(2);
        set_body(
            &mut params,
            &format!("{}{TRUNCATED_MARKER}", &stack[..mid.min(stack.len())]),
        );
        if render(&params).len() <= MAX_URL_LENGTH {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    set_body(
        &mut params,
        &format!("{}{TRUNCATED_MARKER}", &stack[..lo.min(stack.len())]),
    );
    render(&params)
}

fn url_encode(input: &str) -> String {
    let mut out = String::new();
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char)
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

/// Crash-screen actions (keys verbatim: c/r/q).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrashAction {
    Copy,
    Restart,
    Quit,
}

/// Crash screen state.
pub struct ErrorScreen {
    pub message: String,
    pub stack: String,
    pub light: bool,
    pub version: String,
    pub copied: bool,
    pub selected: usize,
    pub scroll_offset: usize,
    pub exit_requested: bool,
    pub reset_requested: bool,
}

impl ErrorScreen {
    pub fn new(message: &str, stack: &str, light: bool, version: &str) -> Self {
        Self {
            message: if message.is_empty() {
                "An unknown error occurred.".to_string()
            } else {
                message.to_string()
            },
            stack: if stack.is_empty() {
                "No stack trace available.".to_string()
            } else {
                stack.to_string()
            },
            light,
            version: version.to_string(),
            copied: false,
            selected: 0,
            scroll_offset: 0,
            exit_requested: false,
            reset_requested: false,
        }
    }

    pub fn actions() -> [CrashAction; 3] {
        [CrashAction::Copy, CrashAction::Restart, CrashAction::Quit]
    }

    fn move_selection(&mut self, delta: i64) {
        let len = Self::actions().len() as i64;
        self.selected = (self.selected as i64 + delta).rem_euclid(len) as usize;
    }

    pub fn action_label(&self, action: CrashAction) -> String {
        match action {
            CrashAction::Copy => {
                if self.copied {
                    "✓ Copied".to_string()
                } else {
                    "Copy report".to_string()
                }
            }
            CrashAction::Restart => "Restart".to_string(),
            CrashAction::Quit => "Quit".to_string(),
        }
    }

    fn use_action(&mut self, action: CrashAction, clipboard: &ClipboardService) {
        match action {
            CrashAction::Copy => self.copy_report(clipboard),
            CrashAction::Restart => self.reset_requested = true,
            CrashAction::Quit => self.exit_requested = true,
        }
    }

    fn copy_report(&mut self, clipboard: &ClipboardService) {
        clipboard.write(&build_issue_url(&self.message, &self.stack, &self.version));
        self.copied = true;
    }

    /// Keyboard map verbatim (vertical keys scroll, the rest act).
    pub fn handle_key(
        &mut self,
        key: &KeyEvent,
        clipboard: &ClipboardService,
        stack_lines: usize,
        view_height: usize,
    ) -> bool {
        let name = key_name(key);
        if key.modifiers.contains(KeyModifiers::CONTROL) && name == "c" {
            self.exit_requested = true;
            return true;
        }
        match name.as_str() {
            "return" => {
                self.use_action(Self::actions()[self.selected], clipboard);
                true
            }
            "left" => {
                self.move_selection(-1);
                true
            }
            "right" => {
                self.move_selection(1);
                true
            }
            "tab" => {
                self.move_selection(if key.modifiers.contains(KeyModifiers::SHIFT) {
                    -1
                } else {
                    1
                });
                true
            }
            "up" => {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
                true
            }
            "down" => {
                self.scroll_offset =
                    (self.scroll_offset + 1).min(stack_lines.saturating_sub(view_height));
                true
            }
            "pageup" => {
                self.scroll_offset = self.scroll_offset.saturating_sub(view_height);
                true
            }
            "pagedown" => {
                self.scroll_offset =
                    (self.scroll_offset + view_height).min(stack_lines.saturating_sub(view_height));
                true
            }
            "home" => {
                self.scroll_offset = 0;
                true
            }
            "end" => {
                self.scroll_offset = stack_lines.saturating_sub(view_height);
                true
            }
            "q" => {
                self.exit_requested = true;
                true
            }
            "c" => {
                self.copy_report(clipboard);
                true
            }
            "r" => {
                self.reset_requested = true;
                true
            }
            _ => false,
        }
    }

    /// Responsive thresholds verbatim (84/24 width clamp, 18/20 heights).
    pub fn render(&self, area: Rect, buf: &mut Buffer, term_width: u16, term_height: u16) {
        let colors = crash_colors(self.light);
        Block::default()
            .style(ratatui::style::Style::default().bg(rgba(colors.bg)))
            .render(area, buf);
        let content_width = (84u16).min((24u16).max(term_width.saturating_sub(4)));
        let x = area.x + area.width.saturating_sub(content_width) / 2;
        let show_subtext = term_height >= 18;
        let show_footer = term_height >= 20;
        let mut y = area.y + 1;
        let title = |fg: Rgba, bold: bool| {
            let mut style = ratatui::style::Style::default().fg(rgba(fg));
            if bold {
                style = style.add_modifier(Modifier::BOLD);
            }
            style
        };
        Paragraph::new(Line::styled("opencode crashed", title(colors.text, true))).render(
            Rect {
                x,
                y,
                width: content_width,
                height: 1,
            },
            buf,
        );
        y += 1;
        if show_subtext {
            Paragraph::new(Line::styled(
                "An unexpected error stopped the session.",
                ratatui::style::Style::default().fg(rgba(colors.muted)),
            ))
            .render(
                Rect {
                    x,
                    y,
                    width: content_width,
                    height: 1,
                },
                buf,
            );
            y += 1;
        }
        // Error panel (rounded red border titled ` Error `).
        let panel = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(ratatui::style::Style::default().fg(rgba(colors.error)))
            .title(" Error ")
            .title_style(ratatui::style::Style::default().fg(rgba(colors.error)))
            .style(ratatui::style::Style::default().bg(rgba(colors.bg)));
        let panel_area = Rect {
            x,
            y,
            width: content_width,
            height: 3,
        };
        panel.render(panel_area, buf);
        Paragraph::new(Line::styled(
            self.message.clone(),
            ratatui::style::Style::default().fg(rgba(colors.text)),
        ))
        .render(
            Rect {
                x: x + 2,
                y: y + 1,
                width: content_width.saturating_sub(4),
                height: 1,
            },
            buf,
        );
        y += 4;
        // Actions row (min width 15, key hint below).
        let mut ax = x;
        for (index, action) in Self::actions().iter().enumerate() {
            let is_selected = self.selected == index;
            let is_copied = *action == CrashAction::Copy && self.copied;
            let bg = if is_copied {
                colors.success
            } else if is_selected {
                colors.primary
            } else {
                colors.element
            };
            let fg = if is_copied || is_selected {
                colors.on_primary
            } else {
                colors.text
            };
            let label = self.action_label(*action);
            let width = (label.chars().count() + 4).max(15) as u16;
            Block::default()
                .style(ratatui::style::Style::default().bg(rgba(bg)))
                .render(
                    Rect {
                        x: ax,
                        y,
                        width: width.min(content_width.saturating_sub(ax - x)),
                        height: 1,
                    },
                    buf,
                );
            Paragraph::new(Line::styled(label, title(fg, true))).render(
                Rect {
                    x: ax + 2,
                    y,
                    width: width
                        .saturating_sub(4)
                        .min(content_width.saturating_sub(ax - x)),
                    height: 1,
                },
                buf,
            );
            Paragraph::new(Line::styled(
                action_key(*action),
                ratatui::style::Style::default().fg(rgba(if is_selected {
                    colors.primary
                } else {
                    colors.muted
                })),
            ))
            .render(
                Rect {
                    x: ax + 2,
                    y: y + 1,
                    width: 12,
                    height: 1,
                },
                buf,
            );
            ax += width + 2;
        }
        y += 2;
        // Stack trace panel (rounded subtle border, ` ↑↓ scroll ` footer).
        let remaining = area.y
            + area
                .height
                .saturating_sub(y)
                .saturating_sub(if show_footer { 3 } else { 1 });
        let trace = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(ratatui::style::Style::default().fg(rgba(colors.border_subtle)))
            .title(" Stack trace ")
            .title_style(ratatui::style::Style::default().fg(rgba(colors.muted)));
        let trace_area = Rect {
            x,
            y,
            width: content_width,
            height: remaining.max(3),
        };
        trace.render(trace_area, buf);
        let stack_lines: Vec<&str> = self.stack.lines().collect();
        let visible = (trace_area.height as usize).saturating_sub(2);
        for (row, line) in stack_lines
            .iter()
            .skip(self.scroll_offset)
            .take(visible)
            .enumerate()
        {
            Paragraph::new(Line::styled(
                (*line).to_string(),
                ratatui::style::Style::default().fg(rgba(colors.muted)),
            ))
            .render(
                Rect {
                    x: x + 1,
                    y: y + 1 + row as u16,
                    width: content_width.saturating_sub(2),
                    height: 1,
                },
                buf,
            );
        }
        Paragraph::new(Line::styled(
            " ↑↓ scroll ",
            ratatui::style::Style::default().fg(rgba(colors.muted)),
        ))
        .render(
            Rect {
                x: x + content_width.saturating_sub(13),
                y: y + trace_area.height.saturating_sub(1),
                width: 12,
                height: 1,
            },
            buf,
        );
        if show_footer {
            let footer = if self.copied {
                "Report copied — paste it into a new GitHub issue."
            } else {
                "Copy the report and open a GitHub issue to help us fix this."
            };
            Paragraph::new(Line::styled(
                footer,
                ratatui::style::Style::default().fg(rgba(colors.muted)),
            ))
            .render(
                Rect {
                    x,
                    y: y + trace_area.height + 1,
                    width: content_width,
                    height: 1,
                },
                buf,
            );
            Paragraph::new(Line::styled(
                format!("opencode {}", self.version),
                ratatui::style::Style::default().fg(rgba(colors.muted)),
            ))
            .render(
                Rect {
                    x,
                    y: y + trace_area.height + 2,
                    width: content_width,
                    height: 1,
                },
                buf,
            );
        }
    }
}

fn action_key(action: CrashAction) -> &'static str {
    match action {
        CrashAction::Copy => "c",
        CrashAction::Restart => "r",
        CrashAction::Quit => "q",
    }
}

fn key_name(key: &KeyEvent) -> String {
    match key.code {
        KeyCode::Enter => "return".to_string(),
        KeyCode::Left => "left".to_string(),
        KeyCode::Right => "right".to_string(),
        KeyCode::Up => "up".to_string(),
        KeyCode::Down => "down".to_string(),
        KeyCode::Tab => "tab".to_string(),
        KeyCode::BackTab => "shift+tab".to_string(),
        KeyCode::Home => "home".to_string(),
        KeyCode::End => "end".to_string(),
        KeyCode::PageUp => "pageup".to_string(),
        KeyCode::PageDown => "pagedown".to_string(),
        KeyCode::Esc => "escape".to_string(),
        KeyCode::Char(ch) => ch.to_string(),
        _ => String::new(),
    }
}
