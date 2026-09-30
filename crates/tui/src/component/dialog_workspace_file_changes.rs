// source: packages/tui/src/component/dialog-workspace-file-changes.tsx (144 lines, v1.18.30)
// 1:1 port — yes/no toggle (left/right, clamped), return confirms,
// `show()` promise (`"yes"`/`"no"`/`None` on close); file rows with
// A/D/M labels and +/- counts verbatim.

#![allow(dead_code)]

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget, Wrap};
use serde_json::Value;
use tokio::sync::oneshot;

use super::super::ui::dialog::{DialogBinding, DialogContent, DialogControl, DialogStack};
use super::super::ui::link::rgba;
use crate::theme::Theme;
use crate::util::locale::truncate_left;
use std::sync::{Arc, Mutex};

/// Mirrors `WorkspaceFileChangesChoice`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileChangesChoice {
    No,
    Yes,
}

impl FileChangesChoice {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileChangesChoice::No => "no",
            FileChangesChoice::Yes => "yes",
        }
    }
}

const CHOICES: [FileChangesChoice; 2] = [FileChangesChoice::No, FileChangesChoice::Yes];

fn status_label(status: &str) -> &'static str {
    match status {
        "added" => "A",
        "deleted" => "D",
        _ => "M",
    }
}

fn change_count_width(file: &Value) -> usize {
    let additions = file.get("additions").and_then(|v| v.as_u64()).unwrap_or(0);
    let deletions = file.get("deletions").and_then(|v| v.as_u64()).unwrap_or(0);
    let mut text = String::new();
    if additions > 0 {
        text += &format!("+{additions}");
    }
    if deletions > 0 {
        text += &format!(" -{deletions}");
    }
    text.len() + 2
}

type SharedDone = Arc<Mutex<Option<oneshot::Sender<Option<FileChangesChoice>>>>>;

fn resolve_shared(shared: &SharedDone, result: Option<FileChangesChoice>) {
    if let Ok(mut guard) = shared.lock() {
        if let Some(tx) = guard.take() {
            let _ = tx.send(result);
        }
    }
}

/// File-changes dialog state (starts on `yes`, verbatim).
pub struct FileChangesState {
    pub files: Vec<Value>,
    pub title: Option<String>,
    pub message: Option<String>,
    pub active: FileChangesChoice,
    done: SharedDone,
}

impl FileChangesState {
    fn confirm(&mut self, control: &mut dyn DialogControl) {
        let active = self.active;
        resolve_shared(&self.done, Some(active));
        control.clear();
    }

    fn step(&mut self, direction: i64) {
        let index = CHOICES.iter().position(|c| *c == self.active).unwrap_or(1) as i64;
        let next = (index + direction).clamp(0, CHOICES.len() as i64 - 1);
        self.active = CHOICES[next as usize];
    }

    fn file_name_width(&self) -> usize {
        let widest = self
            .files
            .iter()
            .map(change_count_width)
            .max()
            .unwrap_or(7)
            .max(7);
        48usize.saturating_sub(widest.saturating_sub(7))
    }

    fn list_height(&self) -> usize {
        self.files.len().min(8)
    }
}

impl DialogContent for FileChangesState {
    fn render(&self, theme: &Theme, area: Rect, buf: &mut Buffer) {
        let mut y = area.y;
        Line::styled(
            self.title
                .clone()
                .unwrap_or_else(|| "File Changes Found".to_string()),
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
        Paragraph::new(
            self.message.clone().unwrap_or_else(|| {
                "Do you want to move these changes with the session?".to_string()
            }),
        )
        .style(Style::default().fg(rgba(theme.text_muted)))
        .wrap(Wrap { trim: false })
        .render(
            Rect {
                x: area.x + 2,
                y,
                width: area.width.saturating_sub(4),
                height: 2,
            },
            buf,
        );
        y += 2;
        let name_width = self.file_name_width();
        let height = self.list_height() as u16;
        let list_area = Rect {
            x: area.x,
            y,
            width: area.width,
            height,
        };
        crate::ui::link::rgba(theme.background_element);
        ratatui::widgets::Block::default()
            .style(Style::default().bg(rgba(theme.background_element)))
            .render(list_area, buf);
        for (row, file) in self.files.iter().take(height as usize).enumerate() {
            let status = file.get("status").and_then(|v| v.as_str()).unwrap_or("");
            let name = file.get("file").and_then(|v| v.as_str()).unwrap_or("");
            let additions = file.get("additions").and_then(|v| v.as_u64()).unwrap_or(0);
            let deletions = file.get("deletions").and_then(|v| v.as_u64()).unwrap_or(0);
            let row_y = y + row as u16;
            Paragraph::new(Line::styled(
                status_label(status),
                Style::default().fg(rgba(theme.text_muted)),
            ))
            .render(
                Rect {
                    x: area.x + 2,
                    y: row_y,
                    width: 2,
                    height: 1,
                },
                buf,
            );
            Paragraph::new(Line::styled(
                truncate_left(name, name_width),
                Style::default().fg(rgba(theme.text_muted)),
            ))
            .render(
                Rect {
                    x: area.x + 4,
                    y: row_y,
                    width: area.width.saturating_sub(12),
                    height: 1,
                },
                buf,
            );
            let mut counts = String::from(" ");
            if additions > 0 {
                counts += &format!("+{additions}");
            }
            if deletions > 0 {
                counts += &format!(" -{deletions}");
            }
            let styled = if additions > 0 && deletions > 0 {
                Line::from(vec![
                    Span::styled(
                        format!(" +{additions}"),
                        Style::default().fg(rgba(theme.diff_added)),
                    ),
                    Span::styled(
                        format!(" -{deletions}"),
                        Style::default().fg(rgba(theme.diff_removed)),
                    ),
                ])
            } else if additions > 0 {
                Line::styled(
                    format!(" +{additions}"),
                    Style::default().fg(rgba(theme.diff_added)),
                )
            } else if deletions > 0 {
                Line::styled(
                    format!(" -{deletions}"),
                    Style::default().fg(rgba(theme.diff_removed)),
                )
            } else {
                Line::from(" ")
            };
            let _ = counts;
            Paragraph::new(styled).render(
                Rect {
                    x: area.x + area.width.saturating_sub(9),
                    y: row_y,
                    width: 7,
                    height: 1,
                },
                buf,
            );
        }
        y += height;
        let mut x = area.x + 2;
        for choice in CHOICES {
            let label = choice.as_str();
            let width = (label.chars().count() + 4) as u16;
            let active = self.active == choice;
            let style = if active {
                Style::default()
                    .fg(rgba(theme.selected_list_item_text))
                    .bg(rgba(theme.primary))
            } else {
                Style::default().fg(rgba(theme.text_muted))
            };
            Paragraph::new(Line::styled(format!("  {label}  "), style)).render(
                Rect {
                    x,
                    y: y + 1,
                    width,
                    height: 1,
                },
                buf,
            );
            x += width + 1;
        }
    }

    fn handle_key(&mut self, key: &KeyEvent, control: &mut dyn DialogControl) -> bool {
        match key.code {
            KeyCode::Enter => {
                self.confirm(control);
                true
            }
            KeyCode::Left => {
                self.step(-1);
                true
            }
            KeyCode::Right => {
                self.step(1);
                true
            }
            _ => false,
        }
    }

    fn bindings(&self) -> Vec<DialogBinding> {
        Vec::new()
    }
}

/// Mirrors `DialogWorkspaceFileChanges.show`.
pub fn show_file_changes(
    stack: &mut DialogStack,
    files: Vec<Value>,
    title: Option<String>,
    message: Option<String>,
) -> oneshot::Receiver<Option<FileChangesChoice>> {
    let (tx, rx) = oneshot::channel();
    let shared: SharedDone = Arc::new(Mutex::new(Some(tx)));
    let on_close = shared.clone();
    stack.replace(
        Box::new(FileChangesState {
            files,
            title,
            message,
            active: FileChangesChoice::Yes,
            done: shared,
        }),
        Some(Box::new(move || resolve_shared(&on_close, None))),
    );
    rx
}
