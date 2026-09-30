//! Rust port of `packages/core/src/tool/question.ts`.

use serde::{Deserialize, Serialize};

pub const NAME: &str = "question";
pub const DESCRIPTION: &str = "Use this tool when you need to ask the user questions during execution. This allows you to:\n1. Gather user preferences or requirements\n2. Clarify ambiguous instructions\n3. Get decisions on implementation choices as you work\n4. Offer choices to the user about what direction to take.\n\nUsage notes:\n- When `custom` is enabled (default), a \"Type your own answer\" option is added automatically; don't include \"Other\" or catch-all options\n- Answers are returned as arrays of labels; set `multiple: true` to allow selecting more than one\n- If you recommend a specific option, make that the first option in the list and add \"(Recommended)\" at the end of the label";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptOption {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub question: String,
    pub header: String,
    pub options: Vec<PromptOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub questions: Vec<Prompt>,
}

pub type Answer = Vec<String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub answers: Vec<Answer>,
}

pub fn to_model_output(questions: &[Prompt], answers: &[Answer]) -> String {
    let formatted = questions
        .iter()
        .enumerate()
        .map(|(i, q)| {
            let ans = answers
                .get(i)
                .map(|a| {
                    if a.is_empty() {
                        "Unanswered".to_string()
                    } else {
                        a.join(", ")
                    }
                })
                .unwrap_or_else(|| "Unanswered".to_string());
            format!("\"{}\"=\"{}\"", q.question, ans)
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("User has answered your questions: {formatted}. You can now continue with the user's answers in mind.")
}

// PROVISIONAL pending QuestionV2 + PermissionV2 wiring.
