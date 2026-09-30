// source: packages/tui/src/ui/dialog-export-options.tsx (220 lines, v1.18.30)
// 1:1 port — tab order, space toggle (gated off filename), per-field
// footers, `FILENAME` traits equivalent, `show()` result verbatim.

#![allow(dead_code)]

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph, Widget, Wrap};
use tokio::sync::oneshot;

use super::dialog::{DialogBinding, DialogContent, DialogControl, DialogStack, TextAreaState};
use super::link::rgba;
use crate::theme::Theme;
use std::sync::{Arc, Mutex};

/// Mirrors the `onConfirm` options object.
#[derive(Debug, Clone)]
pub struct ExportResult {
    pub filename: String,
    pub thinking: bool,
    pub tool_details: bool,
    pub assistant_metadata: bool,
    pub open_without_saving: bool,
}

/// Mirrors the `show()` result (`options | null`).
pub type ExportDialogResult = Option<ExportResult>;

/// Focus order verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportField {
    Filename,
    Thinking,
    ToolDetails,
    AssistantMetadata,
    OpenWithoutSaving,
}

const FIELD_ORDER: [ExportField; 5] = [
    ExportField::Filename,
    ExportField::Thinking,
    ExportField::ToolDetails,
    ExportField::AssistantMetadata,
    ExportField::OpenWithoutSaving,
];

type SharedDone = Arc<Mutex<Option<oneshot::Sender<ExportDialogResult>>>>;

fn resolve_shared(shared: &SharedDone, result: ExportDialogResult) {
    if let Ok(mut guard) = shared.lock() {
        if let Some(tx) = guard.take() {
            let _ = tx.send(result);
        }
    }
}

/// Export-options dialog state (defaults seed the store verbatim).
pub struct ExportState {
    pub filename: TextAreaState,
    pub thinking: bool,
    pub tool_details: bool,
    pub assistant_metadata: bool,
    pub open_without_saving: bool,
    pub active: ExportField,
    done: SharedDone,
}

impl ExportState {
    fn confirm(&mut self) {
        let result = ExportResult {
            filename: self.filename.plain_text(),
            thinking: self.thinking,
            tool_details: self.tool_details,
            assistant_metadata: self.assistant_metadata,
            open_without_saving: self.open_without_saving,
        };
        resolve_shared(&self.done, Some(result));
    }

    fn cycle_field(&mut self) {
        let current = FIELD_ORDER
            .iter()
            .position(|f| *f == self.active)
            .unwrap_or(0);
        self.active = FIELD_ORDER[(current + 1) % FIELD_ORDER.len()];
    }

    fn toggle_active(&mut self) {
        // Mirrors the space binding (enabled only off filename).
        if self.active == ExportField::Filename {
            return;
        }
        match self.active {
            ExportField::Thinking => self.thinking = !self.thinking,
            ExportField::ToolDetails => self.tool_details = !self.tool_details,
            ExportField::AssistantMetadata => self.assistant_metadata = !self.assistant_metadata,
            ExportField::OpenWithoutSaving => self.open_without_saving = !self.open_without_saving,
            ExportField::Filename => {}
        }
    }

    fn check(&self, on: bool) -> &'static str {
        if on {
            "[x]"
        } else {
            "[ ]"
        }
    }
}

impl DialogContent for ExportState {
    fn render(&self, theme: &Theme, area: Rect, buf: &mut Buffer) {
        let mut y = area.y;
        Line::styled(
            "Export Options",
            Style::default()
                .fg(rgba(theme.text))
                .add_modifier(Modifier::BOLD),
        )
        .render(
            Rect {
                x: area.x + 2,
                y,
                width: area.width.saturating_sub(7),
                height: 1,
            },
            buf,
        );
        Line::styled("esc", Style::default().fg(rgba(theme.text_muted))).render(
            Rect {
                x: area.x + area.width.saturating_sub(5),
                y,
                width: 3,
                height: 1,
            },
            buf,
        );
        y += 1;
        Paragraph::new(Line::styled(
            "Filename:",
            Style::default().fg(rgba(theme.text)),
        ))
        .render(
            Rect {
                x: area.x + 2,
                y,
                width: area.width.saturating_sub(4),
                height: 1,
            },
            buf,
        );
        y += 1;
        let (viewport, _, _) = self.filename.viewport(3);
        Paragraph::new(
            viewport
                .into_iter()
                .map(|line| Line::styled(line, Style::default().fg(rgba(theme.text))))
                .collect::<Vec<_>>(),
        )
        .render(
            Rect {
                x: area.x + 2,
                y,
                width: area.width.saturating_sub(4),
                height: 3,
            },
            buf,
        );
        y += 3;
        for (field, label, on) in [
            (ExportField::Thinking, "Include thinking", self.thinking),
            (
                ExportField::ToolDetails,
                "Include tool details",
                self.tool_details,
            ),
            (
                ExportField::AssistantMetadata,
                "Include assistant metadata",
                self.assistant_metadata,
            ),
            (
                ExportField::OpenWithoutSaving,
                "Open without saving",
                self.open_without_saving,
            ),
        ] {
            let active = self.active == field;
            let row = Line::from(vec![
                Span::styled(
                    format!("{} ", self.check(on)),
                    Style::default().fg(rgba(if active {
                        theme.primary
                    } else {
                        theme.text_muted
                    })),
                ),
                Span::styled(
                    label,
                    Style::default().fg(rgba(if active { theme.primary } else { theme.text })),
                ),
            ]);
            let row_area = Rect {
                x: area.x + 2,
                y,
                width: area.width.saturating_sub(4),
                height: 1,
            };
            if active {
                Block::default()
                    .style(Style::default().bg(rgba(theme.background_element)))
                    .render(row_area, buf);
            }
            Paragraph::new(row).render(row_area, buf);
            y += 1;
        }
        let footer = if self.active == ExportField::Filename {
            vec![
                Span::styled("return", Style::default().fg(rgba(theme.text))),
                Span::styled(" to confirm, ", Style::default().fg(rgba(theme.text_muted))),
                Span::styled("tab", Style::default().fg(rgba(theme.text))),
                Span::styled(" for options", Style::default().fg(rgba(theme.text_muted))),
            ]
        } else {
            vec![
                Span::styled("space", Style::default().fg(rgba(theme.text))),
                Span::styled(" to toggle, ", Style::default().fg(rgba(theme.text_muted))),
                Span::styled("return", Style::default().fg(rgba(theme.text))),
                Span::styled(" to confirm", Style::default().fg(rgba(theme.text_muted))),
            ]
        };
        Paragraph::new(Line::from(footer)).render(
            Rect {
                x: area.x + 2,
                y,
                width: area.width.saturating_sub(4),
                height: 1,
            },
            buf,
        );
    }

    fn handle_key(&mut self, key: &KeyEvent, _control: &mut dyn DialogControl) -> bool {
        match key.code {
            KeyCode::Tab => {
                self.cycle_field();
                true
            }
            KeyCode::Char(' ') if self.active != ExportField::Filename => {
                self.toggle_active();
                true
            }
            KeyCode::Enter if self.active == ExportField::Filename => {
                // Filename submit (mirrors the textarea onSubmit).
                self.confirm();
                true
            }
            KeyCode::Enter => {
                self.confirm();
                true
            }
            _ => {
                if self.active == ExportField::Filename {
                    return match key.code {
                        KeyCode::Char(ch) => {
                            self.filename.insert(ch);
                            true
                        }
                        KeyCode::Backspace => {
                            self.filename.backspace();
                            true
                        }
                        KeyCode::Left => {
                            self.filename.move_left();
                            true
                        }
                        KeyCode::Right => {
                            self.filename.move_right();
                            true
                        }
                        _ => false,
                    };
                }
                false
            }
        }
    }

    fn bindings(&self) -> Vec<DialogBinding> {
        vec![
            DialogBinding {
                key: "tab".to_string(),
                desc: "Next export option".to_string(),
                group: "Dialog".to_string(),
            },
            DialogBinding {
                key: "space".to_string(),
                desc: "Toggle export option".to_string(),
                group: "Dialog".to_string(),
            },
        ]
    }
}

/// Mirrors `DialogExportOptions.show`.
pub fn show_export_options(
    stack: &mut DialogStack,
    default_filename: &str,
    default_thinking: bool,
    default_tool_details: bool,
    default_assistant_metadata: bool,
    default_open_without_saving: bool,
) -> oneshot::Receiver<ExportDialogResult> {
    let (tx, rx) = oneshot::channel();
    let shared: SharedDone = Arc::new(Mutex::new(Some(tx)));
    let on_close = shared.clone();
    stack.replace(
        Box::new(ExportState {
            filename: TextAreaState::new(Some(default_filename), "Enter filename"),
            thinking: default_thinking,
            tool_details: default_tool_details,
            assistant_metadata: default_assistant_metadata,
            open_without_saving: default_open_without_saving,
            active: ExportField::Filename,
            done: shared,
        }),
        Some(Box::new(move || resolve_shared(&on_close, None))),
    );
    stack.set_size(crate::ui::dialog::DialogSize::Medium);
    rx
}
