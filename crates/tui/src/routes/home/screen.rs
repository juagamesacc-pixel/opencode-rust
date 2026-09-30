// source: packages/tui/src/routes/home.tsx (95 lines, v1.18.30)
// Home route descriptor: placeholder sets, max-width rule, mount/cleanup
// semantics (editor selection clear, one-shot prompt seeding, ready-gated
// auto-submit), and the layout slot order.

#![allow(dead_code)]

/// Placeholder sets verbatim.
pub const HOME_PLACEHOLDERS_NORMAL: &[&str] = &[
    "Fix a TODO in the codebase",
    "What is the tech stack of this project?",
    "Fix broken tests",
];
pub const HOME_PLACEHOLDERS_SHELL: &[&str] = &["ls -la", "git status", "pwd"];

/// Prompt max width (mirrors `promptMaxWidth`: `auto` → 70% floor 75).
pub fn home_prompt_max_width(configured: Option<u16>, auto: bool, term_width: u16) -> u16 {
    if auto {
        return ((term_width as usize * 7 / 10).max(75) as u16).min(term_width.max(1));
    }
    configured.unwrap_or(75)
}

/// One-shot prompt seeding (mirrors the `once` bind logic).
#[derive(Debug, Clone, Default)]
pub struct HomeSeed {
    done: bool,
}

impl HomeSeed {
    /// Returns the seed prompt when the one-shot fires (route prompt wins
    /// over `--prompt`, verbatim order).
    pub fn seed(
        &mut self,
        route_prompt: Option<crate::prompt::history::PromptInfo>,
        args_prompt: Option<&str>,
    ) -> Option<crate::prompt::history::PromptInfo> {
        if self.done {
            return None;
        }
        if let Some(prompt) = route_prompt {
            self.done = true;
            return Some(prompt);
        }
        if let Some(input) = args_prompt {
            self.done = true;
            return Some(crate::prompt::history::PromptInfo {
                input: input.to_string(),
                mode: None,
                parts: Vec::new(),
            });
        }
        None
    }

    /// Auto-submit gate (mirrors the ready effect: sync + model ready,
    /// args prompt echoed in the input, fires once).
    pub fn should_auto_submit(
        &self,
        sent: bool,
        has_ref: bool,
        sync_ready: bool,
        model_ready: bool,
        args_prompt: Option<&str>,
        current_input: &str,
    ) -> bool {
        if sent || !has_ref || !sync_ready || !model_ready {
            return false;
        }
        match args_prompt {
            Some(prompt) => current_input == prompt,
            None => false,
        }
    }
}

/// Layout slot order (mirrors the JSX tree for the app renderer).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HomeSlot {
    Logo,
    Prompt,
    PromptRight,
    Bottom,
    Footer,
}

pub const HOME_SLOTS: &[HomeSlot] = &[
    HomeSlot::Logo,
    HomeSlot::Prompt,
    HomeSlot::PromptRight,
    HomeSlot::Bottom,
    HomeSlot::Footer,
];
