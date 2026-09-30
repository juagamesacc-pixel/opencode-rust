// source: packages/tui/src/routes/session/question.tsx (515 lines, v1.18.30)
// 1:1 port — question prompt state machine: tabs (questions + confirm,
// none for single-select), single vs multi semantics, custom answers
// with the edit flow, 1-9 shortcuts, and the reply/reject payloads.

#![allow(dead_code)]

use serde_json::Value;

/// Mode id verbatim.
pub const QUESTION_MODE: &str = "question";

/// One question option.
#[derive(Debug, Clone)]
pub struct QuestionOption {
    pub label: String,
    pub description: Option<String>,
}

/// One question (subset of fields read here).
#[derive(Debug, Clone)]
pub struct Question {
    pub header: String,
    pub question: String,
    pub options: Vec<QuestionOption>,
    pub multiple: bool,
    pub custom_enabled: bool,
}

impl Question {
    pub fn from_value(value: &Value) -> Self {
        Self {
            header: value
                .get("header")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            question: value
                .get("question")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            options: value
                .get("options")
                .and_then(|v| v.as_array())
                .map(|options| {
                    options
                        .iter()
                        .map(|opt| QuestionOption {
                            label: opt
                                .get("label")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            description: opt
                                .get("description")
                                .and_then(|v| v.as_str())
                                .map(str::to_string),
                        })
                        .collect()
                })
                .unwrap_or_default(),
            multiple: value
                .get("multiple")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            custom_enabled: value
                .get("custom")
                .map(|v| v != &Value::Bool(false))
                .unwrap_or(true),
        }
    }
}

/// Question prompt state.
pub struct QuestionState {
    pub questions: Vec<Question>,
    pub tab: usize,
    pub answers: Vec<Vec<String>>,
    pub custom: Vec<String>,
    pub selected: usize,
    pub editing: bool,
}

impl QuestionState {
    pub fn new(request: &Value) -> Self {
        let questions: Vec<Question> = request
            .get("questions")
            .and_then(|v| v.as_array())
            .map(|list| list.iter().map(Question::from_value).collect())
            .unwrap_or_default();
        let tabs = questions.len();
        Self {
            questions,
            tab: 0,
            answers: vec![Vec::new(); tabs],
            custom: vec![String::new(); tabs],
            selected: 0,
            editing: false,
        }
    }

    /// Single-select when exactly one question without `multiple`.
    pub fn single(&self) -> bool {
        self.questions.len() == 1 && !self.questions.first().map(|q| q.multiple).unwrap_or(false)
    }

    /// Tab count (questions + confirm, none for single).
    pub fn tabs(&self) -> usize {
        if self.single() {
            1
        } else {
            self.questions.len() + 1
        }
    }

    pub fn confirm_tab(&self) -> bool {
        !self.single() && self.tab == self.questions.len()
    }

    pub fn question(&self) -> Option<&Question> {
        self.questions.get(self.tab)
    }

    pub fn multi(&self) -> bool {
        self.question().map(|q| q.multiple).unwrap_or(false)
    }

    /// `other` (custom row focused) — custom enabled + selected past options.
    pub fn other(&self) -> bool {
        let options = self.question().map(|q| q.options.len()).unwrap_or(0);
        self.question().map(|q| q.custom_enabled).unwrap_or(false) && self.selected == options
    }

    pub fn input(&self) -> String {
        self.custom.get(self.tab).cloned().unwrap_or_default()
    }

    pub fn custom_picked(&self) -> bool {
        let value = self.input();
        if value.is_empty() {
            return false;
        }
        self.answers
            .get(self.tab)
            .map(|answers| answers.contains(&value))
            .unwrap_or(false)
    }

    /// Mirrors `pick` — single questions reply immediately.
    pub fn pick(&mut self, answer: &str, custom: bool) -> Option<Vec<Vec<String>>> {
        while self.answers.len() <= self.tab {
            self.answers.push(Vec::new());
        }
        self.answers[self.tab] = vec![answer.to_string()];
        if custom {
            while self.custom.len() <= self.tab {
                self.custom.push(String::new());
            }
            self.custom[self.tab] = answer.to_string();
        }
        if self.single() {
            return Some(vec![vec![answer.to_string()]]);
        }
        self.tab += 1;
        self.selected = 0;
        None
    }

    /// Mirrors `toggle` (multi-select).
    pub fn toggle(&mut self, answer: &str) {
        while self.answers.len() <= self.tab {
            self.answers.push(Vec::new());
        }
        let entry = &mut self.answers[self.tab];
        if let Some(index) = entry.iter().position(|item| item == answer) {
            entry.remove(index);
        } else {
            entry.push(answer.to_string());
        }
    }

    pub fn move_to(&mut self, index: usize) {
        self.selected = index;
    }

    pub fn select_tab(&mut self, index: usize) {
        self.tab = index;
        self.selected = 0;
    }

    pub fn step_tab(&mut self, direction: i64) {
        let tabs = self.tabs().max(1) as i64;
        self.select_tab((self.tab as i64 + direction).rem_euclid(tabs) as usize);
    }

    /// Option count + custom row (mirrors the `total`/`max` memo).
    pub fn total_rows(&self) -> usize {
        self.question().map(|q| q.options.len()).unwrap_or(0)
            + if self.question().map(|q| q.custom_enabled).unwrap_or(false) {
                1
            } else {
                0
            }
    }

    /// Mirrors `selectOption`.
    pub fn select_option(&mut self) -> QuestionSelectNext {
        if self.other() {
            if !self.multi() {
                self.editing = true;
                return QuestionSelectNext::Edit;
            }
            let value = self.input();
            if !value.is_empty() && self.custom_picked() {
                self.toggle(&value.clone());
                return QuestionSelectNext::Toggled;
            }
            self.editing = true;
            return QuestionSelectNext::Edit;
        }
        let Some(option) = self
            .question()
            .and_then(|q| q.options.get(self.selected))
            .map(|o| o.label.clone())
        else {
            return QuestionSelectNext::None;
        };
        if self.multi() {
            self.toggle(&option);
            return QuestionSelectNext::Toggled;
        }
        if let Some(reply) = self.pick(&option, false) {
            return QuestionSelectNext::Reply(reply);
        }
        QuestionSelectNext::Advanced
    }

    /// Mirrors the edit-submit command (empty clears the custom answer).
    pub fn submit_edit(&mut self, text: &str) -> QuestionSelectNext {
        let text = text.trim().to_string();
        if text.is_empty() {
            if let Some(previous) = self
                .custom
                .get(self.tab)
                .cloned()
                .filter(|previous| !previous.is_empty())
            {
                if let Some(answers) = self.answers.get_mut(self.tab) {
                    answers.retain(|item| item != &previous);
                }
                if let Some(slot) = self.custom.get_mut(self.tab) {
                    *slot = String::new();
                }
            }
            self.editing = false;
            return QuestionSelectNext::Edited;
        }
        if self.multi() {
            while self.custom.len() <= self.tab {
                self.custom.push(String::new());
            }
            let previous = std::mem::replace(&mut self.custom[self.tab], text.clone());
            while self.answers.len() <= self.tab {
                self.answers.push(Vec::new());
            }
            let entry = &mut self.answers[self.tab];
            if !previous.is_empty() {
                entry.retain(|item| item != &previous);
            }
            if !entry.contains(&text) {
                entry.push(text);
            }
            self.editing = false;
            return QuestionSelectNext::Edited;
        }
        self.editing = false;
        if let Some(reply) = self.pick(&text, true) {
            return QuestionSelectNext::Reply(reply);
        }
        QuestionSelectNext::Edited
    }

    /// Mirrors `submit` (all tabs, empty arrays defaulted).
    pub fn submit_answers(&self) -> Vec<Vec<String>> {
        (0..self.questions.len())
            .map(|index| self.answers.get(index).cloned().unwrap_or_default())
            .collect()
    }

    /// Review rows (mirrors the confirm tab).
    pub fn review(&self) -> Vec<(String, String, bool)> {
        self.questions
            .iter()
            .enumerate()
            .map(|(index, question)| {
                let value = self
                    .answers
                    .get(index)
                    .map(|answers| answers.join(", "))
                    .unwrap_or_default();
                (question.header.clone(), value.clone(), !value.is_empty())
            })
            .collect()
    }
}

pub enum QuestionSelectNext {
    None,
    Edit,
    Edited,
    Toggled,
    Advanced,
    Reply(Vec<Vec<String>>),
}

/// Reply/reject payloads (mirrors the SDK calls).
pub fn question_reply_params(
    request_id: &str,
    directory: Option<&str>,
    answers: Vec<Vec<String>>,
) -> Value {
    let mut params = serde_json::json!({ "requestID": request_id, "answers": answers });
    if let Some(directory) = directory {
        params["directory"] = Value::String(directory.to_string());
    }
    params
}

pub fn question_reject_params(request_id: &str, directory: Option<&str>) -> Value {
    let mut params = serde_json::json!({ "requestID": request_id });
    if let Some(directory) = directory {
        params["directory"] = Value::String(directory.to_string());
    }
    params
}

/// Footer hint verbs (mirrors the footer row).
pub fn question_footer(
    single: bool,
    confirm: bool,
    multi: bool,
) -> Vec<(&'static str, &'static str)> {
    let mut hints = Vec::new();
    if !single {
        hints.push(("⇆", "tab"));
    }
    if !confirm {
        hints.push(("↑↓", "select"));
    }
    hints.push((
        "enter",
        if confirm {
            "submit"
        } else if multi {
            "toggle"
        } else if single {
            "submit"
        } else {
            "confirm"
        },
    ));
    hints.push(("esc", "dismiss"));
    hints
}
