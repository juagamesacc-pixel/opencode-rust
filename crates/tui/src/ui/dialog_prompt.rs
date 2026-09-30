// source: packages/tui/src/ui/dialog-prompt.tsx (127 lines, v1.18.30)
// 1:1 port — busy suspend (confirm blocked, muted colors), `BUSY` traits
// equivalent, footer hints, `show()` promise (`String | null`).

#![allow(dead_code)]

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Widget, Wrap};
use tokio::sync::oneshot;

use super::dialog::{DialogBinding, DialogContent, DialogControl, DialogStack, TextAreaState};
use super::link::rgba;
use crate::theme::Theme;
use std::sync::{Arc, Mutex};

/// Mirrors `DialogPromptProps` (description arrives pre-rendered as lines).
#[derive(Debug, Clone)]
pub struct PromptProps {
    pub title: String,
    pub description: Vec<String>,
    pub placeholder: Option<String>,
    pub value: Option<String>,
    pub busy: bool,
    pub busy_text: Option<String>,
    pub submit_hint: Option<String>,
}

/// Mirrors the `show()` result (`string | null`).
pub type PromptResult = Option<String>;

type SharedDone = Arc<Mutex<Option<oneshot::Sender<PromptResult>>>>;

fn resolve_shared(shared: &SharedDone, result: PromptResult) {
    if let Ok(mut guard) = shared.lock() {
        if let Some(tx) = guard.take() {
            let _ = tx.send(result);
        }
    }
}

/// Prompt dialog state.
pub struct PromptState {
    props: PromptProps,
    textarea: TextAreaState,
    done: SharedDone,
}

impl PromptState {
    fn confirm(&mut self) {
        // Mirrors `confirm` — blocked while busy.
        if self.props.busy {
            return;
        }
        let value = self.textarea.plain_text();
        resolve_shared(&self.done, Some(value));
    }

    /// Intrinsic text editing keys (the submit command itself is dispatched
    /// by the app keymap, mirroring `useBindings` priority semantics).
    pub fn edit_key(&mut self, key: &KeyEvent) -> bool {
        if self.props.busy {
            return false;
        }
        match key.code {
            KeyCode::Char(ch) => {
                self.textarea.insert(ch);
                true
            }
            KeyCode::Backspace => {
                self.textarea.backspace();
                true
            }
            KeyCode::Left => {
                self.textarea.move_left();
                true
            }
            KeyCode::Right => {
                self.textarea.move_right();
                true
            }
            KeyCode::Enter => {
                self.textarea.insert('\n');
                true
            }
            _ => false,
        }
    }
}

impl DialogContent for PromptState {
    fn render(&self, theme: &Theme, area: Rect, buf: &mut Buffer) {
        let mut y = area.y;
        Line::styled(
            self.props.title.clone(),
            Style::default()
                .fg(rgba(theme.text))
                .add_modifier(Modifier::BOLD),
        )
        .render(
            Rect {
                x: area.x + 2,
                y,
                width: area.width.saturating_sub(9),
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
        for line in &self.props.description {
            Paragraph::new(line.clone())
                .style(Style::default().fg(rgba(theme.text)))
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
        }
        let text_color = if self.props.busy {
            theme.text_muted
        } else {
            theme.text
        };
        let (viewport, cursor_row, cursor_col) = self.textarea.viewport(3);
        let body: Vec<Line> = if self.textarea.plain_text().is_empty() {
            vec![Line::styled(
                self.props
                    .placeholder
                    .clone()
                    .unwrap_or_else(|| "Enter text".to_string()),
                Style::default().fg(rgba(theme.text_muted)),
            )]
        } else {
            viewport
                .into_iter()
                .map(|line| Line::styled(line, Style::default().fg(rgba(text_color))))
                .collect()
        };
        Paragraph::new(body).render(
            Rect {
                x: area.x + 2,
                y,
                width: area.width.saturating_sub(4),
                height: 3.min(area.height.saturating_sub(y - area.y)),
            },
            buf,
        );
        let _ = (cursor_row, cursor_col);
        y += 3;
        if self.props.busy {
            let spinner = format!(
                "BUSY {}",
                self.props
                    .busy_text
                    .clone()
                    .unwrap_or_else(|| "Working…".to_string())
            );
            Paragraph::new(Line::styled(
                spinner,
                Style::default().fg(rgba(theme.text_muted)),
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
            Paragraph::new(Line::styled(
                "processing…",
                Style::default().fg(rgba(theme.text_muted)),
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
        } else if let Some(hint) = self.props.submit_hint.as_ref() {
            let footer = Line::from(vec![
                ratatui::text::Span::styled(hint.clone(), Style::default().fg(rgba(theme.text))),
                ratatui::text::Span::styled(" submit", Style::default().fg(rgba(theme.text_muted))),
            ]);
            Paragraph::new(footer).render(
                Rect {
                    x: area.x + 2,
                    y,
                    width: area.width.saturating_sub(4),
                    height: 1,
                },
                buf,
            );
        }
    }

    fn handle_key(&mut self, key: &KeyEvent, _control: &mut dyn DialogControl) -> bool {
        self.edit_key(key)
    }

    fn bindings(&self) -> Vec<DialogBinding> {
        vec![DialogBinding {
            key: "dialog.prompt.submit".to_string(),
            desc: "Submit dialog prompt".to_string(),
            group: "Dialog".to_string(),
        }]
    }
}

/// Mirrors `DialogPrompt.show`.
pub fn show_prompt(
    stack: &mut DialogStack,
    title: &str,
    props: PromptProps,
) -> oneshot::Receiver<PromptResult> {
    let (tx, rx) = oneshot::channel();
    let shared: SharedDone = Arc::new(Mutex::new(Some(tx)));
    let on_close = shared.clone();
    let mut full = props;
    full.title = title.to_string();
    stack.replace(
        Box::new(PromptState {
            textarea: TextAreaState::new(
                full.value.as_deref(),
                full.placeholder.as_deref().unwrap_or("Enter text"),
            ),
            props: full,
            done: shared,
        }),
        Some(Box::new(move || resolve_shared(&on_close, None))),
    );
    // Dialog opens at medium size with the cursor at line end (mirrors onMount).
    stack.set_size(crate::ui::dialog::DialogSize::Medium);
    rx
}
