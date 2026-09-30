// source: packages/tui/src/component/dialog-move-session.tsx (353 lines, v1.18.30)
// 1:1 port — directory/root/subdirectory listing (current-first sort,
// strategy ordering, subdirectory grouping under longest root), title
// splitting with muted suffixes, two-step delete confirm, forced remove
// with the file-changes gate, and the refresh/error views.

#![allow(dead_code)]

use serde_json::Value;

use crate::runtime::abbreviate_home;
use crate::ui::dialog_select::{SelectOption, SelectState, TruncateTitle};
use crate::util::locale::truncate_left;

/// Mirrors `MoveSessionSelection`.
#[derive(Debug, Clone)]
pub enum MoveSessionSelection {
    Directory {
        directory: String,
        subdirectory: bool,
    },
    New,
}

/// One project directory root.
#[derive(Debug, Clone, Default)]
pub struct ProjectDirectory {
    pub directory: String,
    pub strategy: Option<String>,
}

/// Mirrors `contains` — root equality or a contained relative path.
pub fn contains_dir(root: &str, directory: &str) -> bool {
    if root == directory {
        return true;
    }
    let relative = crate::runtime::relative_path(root, directory);
    !relative.is_empty()
        && relative != ".."
        && !relative.starts_with("../")
        && !relative.starts_with('/')
}

/// Split title into visible head + muted suffix (mirrors the suffix math).
pub fn split_title(title: &str, suffix_len: usize) -> (String, String) {
    let visible_len = title.chars().count();
    let split = visible_len.saturating_sub(suffix_len);
    let head: String = title.chars().take(split).collect();
    let tail: String = title.chars().skip(split).collect();
    (head, tail)
}

/// Move-session dialog state.
pub struct MoveSessionState {
    pub project_id: String,
    pub directories: Option<Vec<ProjectDirectory>>,
    pub load_error: Option<String>,
    pub to_delete: Option<String>,
    pub removing: Option<String>,
    pub replacement_current: Option<String>,
    pub current: Option<MoveSessionSelection>,
    pub working: bool,
}

impl MoveSessionState {
    pub fn show_error(&self) -> bool {
        self.load_error.is_some() && self.directories.is_none()
    }

    /// Current root directory (longest containing root, verbatim).
    pub fn current_root(&self, current_directory: Option<&str>) -> Option<ProjectDirectory> {
        let directory = current_directory?;
        let mut roots: Vec<&ProjectDirectory> = self
            .directories
            .as_ref()?
            .iter()
            .filter(|root| contains_dir(&root.directory, directory))
            .collect();
        roots.sort_by_key(|root| std::cmp::Reverse(root.directory.len()));
        roots.first().cloned().cloned().or(Some(ProjectDirectory {
            directory: directory.to_string(),
            strategy: None,
        }))
    }

    /// Sorted roots (current first, strategy roots before plain, verbatim).
    pub fn sorted_roots(&self, current: Option<&str>) -> Vec<ProjectDirectory> {
        let mut roots = self.directories.clone().unwrap_or_default();
        if let Some(current) = current {
            if !roots.iter().any(|item| item.directory == current) {
                roots.insert(
                    0,
                    ProjectDirectory {
                        directory: current.to_string(),
                        strategy: None,
                    },
                );
            }
        }
        roots.sort_by(|a, b| {
            if Some(a.directory.as_str()) == current {
                return std::cmp::Ordering::Less;
            }
            if Some(b.directory.as_str()) == current {
                return std::cmp::Ordering::Greater;
            }
            match (a.strategy.is_some(), b.strategy.is_some()) {
                (false, true) => std::cmp::Ordering::Less,
                (true, false) => std::cmp::Ordering::Greater,
                _ if a.strategy.is_none() => a.directory.len().cmp(&b.directory.len()),
                _ => std::cmp::Ordering::Equal,
            }
        });
        roots
    }

    /// Subdirectory entries grouped under their longest root.
    pub fn subdirectory_entries(
        &self,
        sessions: &[Value],
        project_id: &str,
        roots: &[ProjectDirectory],
    ) -> Vec<(String, ProjectDirectory)> {
        let mut seen: Vec<String> = Vec::new();
        let mut out = Vec::new();
        for session in sessions {
            if session.get("projectID").and_then(|v| v.as_str()) != Some(project_id) {
                continue;
            }
            let path = session.get("path").and_then(|v| v.as_str()).unwrap_or("");
            if path.is_empty() || path == "." || path == "/" {
                continue;
            }
            let directory = session
                .get("directory")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            if roots.iter().any(|root| root.directory == directory) || seen.contains(&directory) {
                continue;
            }
            seen.push(directory.clone());
            let mut candidates: Vec<&ProjectDirectory> = roots
                .iter()
                .filter(|root| {
                    let relative = crate::runtime::relative_path(&root.directory, &directory);
                    !relative.is_empty()
                        && relative != ".."
                        && !relative.starts_with("../")
                        && !relative.starts_with('/')
                })
                .collect();
            candidates.sort_by_key(|root| std::cmp::Reverse(root.directory.len()));
            if let Some(root) = candidates.first() {
                out.push((directory, (*root).clone()));
            }
        }
        out
    }

    /// Full option list (roots + subdirectories, root-major sort).
    pub fn build_options(
        &self,
        sessions: &[Value],
        home: &str,
        term_width: u16,
        to_delete_hint: Option<&str>,
    ) -> Vec<SelectOption> {
        if self.show_error() {
            return Vec::new();
        }
        let current = self.current_root(self.current_directory());
        let current_dir = current.as_ref().map(|root| root.directory.clone());
        let roots = self.sorted_roots(current_dir.as_deref());
        if roots.is_empty() && self.directories.is_some() {
            return vec![SelectOption {
                title: "No project directories found".to_string(),
                ..SelectOption::default()
            }];
        }
        let mut listed: Vec<(String, ProjectDirectory)> = roots
            .iter()
            .map(|root| (root.directory.clone(), root.clone()))
            .collect();
        listed.extend(self.subdirectory_entries(sessions, &self.project_id, &roots));
        listed.sort_by(|a, b| {
            let root_order = roots
                .iter()
                .position(|r| r.directory == a.1.directory)
                .unwrap_or(usize::MAX)
                .cmp(
                    &roots
                        .iter()
                        .position(|r| r.directory == b.1.directory)
                        .unwrap_or(usize::MAX),
                );
            if root_order != std::cmp::Ordering::Equal {
                return root_order;
            }
            if a.0 == a.1.directory {
                return std::cmp::Ordering::Less;
            }
            if b.0 == b.1.directory {
                return std::cmp::Ordering::Greater;
            }
            a.0.cmp(&b.0)
        });
        let title_width = (116u16.min(term_width.saturating_sub(2)) as usize)
            .saturating_sub(12)
            .max(1);
        listed
            .into_iter()
            .map(|(location, root)| {
                let title = abbreviate_home(&location, home);
                let suffix = if location == root.directory {
                    None
                } else {
                    Some(format!(
                        "/{}",
                        crate::runtime::relative_path(&root.directory, &location)
                    ))
                };
                let visible = truncate_left(&title, title_width);
                let deleting = self.to_delete.as_deref() == Some(location.as_str());
                let is_removing = self.removing.as_deref() == Some(location.as_str());
                let (title_text, suffix_text) = match (is_removing, deleting, &suffix) {
                    (true, _, _) => (format!("Deleting {location}"), None),
                    (false, true, _) => (
                        format!(
                            "Press {} again to confirm",
                            to_delete_hint.unwrap_or("delete")
                        ),
                        None,
                    ),
                    (false, false, Some(suffix)) => {
                        let (head, tail) = split_title(&visible, suffix.chars().count());
                        (head, Some(tail))
                    }
                    _ => (visible, None),
                };
                SelectOption {
                    title: title_text,
                    title_view: suffix_text,
                    bg: if deleting {
                        Some(crate::theme::Rgba::from_hex("#e06c75"))
                    } else {
                        None
                    },
                    value: serde_json::json!({
                        "type": "directory",
                        "directory": location,
                        "subdirectory": location != root.directory,
                    }),
                    category: if Some(root.directory.as_str()) == current_dir.as_deref() {
                        Some("Current".to_string())
                    } else {
                        Some("Other".to_string())
                    },
                    title_width: Some(title_width),
                    truncate_title: TruncateTitle::Left,
                    ..SelectOption::default()
                }
            })
            .collect()
    }

    fn current_directory(&self) -> Option<&str> {
        match self.current.as_ref() {
            Some(MoveSessionSelection::Directory { directory, .. }) => Some(directory),
            _ => None,
        }
    }

    /// Full dialog height (mirrors the `fullHeight` memo).
    pub fn full_height(term_height: u16) -> u16 {
        (8u16).max(
            (16u16).min(
                term_height
                    .saturating_sub(term_height / 4)
                    .saturating_sub(2),
            ),
        )
    }

    /// Build the select state (title `Move session`, verbatim).
    pub fn select_state(
        &self,
        sessions: &[Value],
        home: &str,
        term_width: u16,
        to_delete_hint: Option<&str>,
    ) -> SelectState {
        let mut state = SelectState::new(
            "Move session",
            self.build_options(sessions, home, term_width, to_delete_hint),
        );
        state.render_filter = !self.show_error();
        state
    }
}

/// Delete-button enablement (mirrors the action `disabled` fn).
pub fn delete_disabled(
    value: Option<&MoveSessionSelection>,
    directories: &[ProjectDirectory],
) -> bool {
    let Some(MoveSessionSelection::Directory {
        directory,
        subdirectory,
    }) = value
    else {
        return true;
    };
    if *subdirectory {
        return true;
    }
    !directories
        .iter()
        .any(|item| &item.directory == directory && item.strategy.is_some())
}
