//! Port of packages/app/src/components/prompt-input/placeholder.ts
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]

pub struct PromptPlaceholderInput {
    pub mode: String,
    pub comment_count: usize,
    pub example: String,
    pub suggest: bool,
}

pub fn prompt_placeholder<F>(input: &PromptPlaceholderInput, t: F) -> String
where
    F: Fn(&str, Option<&str>) -> String,
{
    if input.mode == "shell" {
        return t("prompt.placeholder.shell", Some(&input.example));
    }
    if input.comment_count > 1 {
        return t("prompt.placeholder.summarizeComments", None);
    }
    if input.comment_count == 1 {
        return t("prompt.placeholder.summarizeComment", None);
    }
    if !input.suggest {
        return t("prompt.placeholder.simple", None);
    }
    t("prompt.placeholder.normal", Some(&input.example))
}

pub fn prompt_design_placeholder<F>(mode: &str, placeholder: &str, t: F) -> String
where
    F: Fn(&str, Option<&str>) -> String,
{
    if mode == "shell" {
        return placeholder.to_string();
    }
    t("ui.promptInput.placeholder.normal", None)
}
