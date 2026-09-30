// source: src/command/index.ts — exports: Event, Info, hints, Default,
// Interface, Service, node, Command
// PROVISIONAL pending crates/core (layer-node) + @/config/config + @/mcp +
// @/skill + @/effect/* + @opencode-ai/schema/legacy-event: Info shape, hints()
// ($N sorted-dedupe + $ARGUMENTS), Default INIT/REVIEW + descriptions,
// template placeholders "${path}", skill suffix lines verbatim.

use serde::{Deserialize, Serialize};

/// source: Default = { INIT: "init", REVIEW: "review" } — verbatim.
pub const DEFAULT_INIT: &str = "init";
pub const DEFAULT_REVIEW: &str = "review";

/// source: init description "guided AGENTS.md setup" — verbatim.
pub const INIT_DESCRIPTION: &str = "guided AGENTS.md setup";
/// source: review description — verbatim.
pub const REVIEW_DESCRIPTION: &str = "review changes [commit|branch|pr], defaults to uncommitted";

/// source: template placeholder "${path}" — verbatim.
pub const PATH_PLACEHOLDER: &str = "${path}";

/// source: include_str! passthroughs for template assets (verbatim bytes).
pub const PROMPT_INITIALIZE: &str = include_str!("template/initialize.txt");
pub const PROMPT_REVIEW: &str = include_str!("template/review.txt");

/// source: skill suffix lines — verbatim.
pub fn skill_suffix(dir: &str) -> String {
    format!(
        "\n\nBase directory for this skill: {}\nRelative paths in this skill (e.g., scripts/, references/) are relative to this base directory.",
        dir
    )
}

/// source: Info { name, description?, agent?, model?, source?, template, subtask?, hints } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub template: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtask: Option<bool>,
    pub hints: Vec<String>,
}

/// source: hints() — $N sorted-dedupe + $ARGUMENTS append. Verbatim.
pub fn hints(template: &str) -> Vec<String> {
    let mut numbered: Vec<String> = Vec::new();
    let bytes = template.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'$' {
            let mut j = i + 1;
            while j < bytes.len() && bytes[j].is_ascii_digit() {
                j += 1;
            }
            if j > i + 1 {
                let m = template[i..j].to_string();
                if !numbered.contains(&m) {
                    numbered.push(m);
                }
                i = j;
                continue;
            }
        }
        i += 1;
    }
    numbered.sort();
    let mut result = numbered;
    if template.contains("$ARGUMENTS") {
        result.push("$ARGUMENTS".to_string());
    }
    result
}

/// source: Interface — get/list, verbatim.
pub trait Interface {
    fn get(&self, name: &str) -> Option<Info>;
    fn list(&self) -> Vec<Info>;
}

/// source: Service "@opencode/Command" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Command";

/// source: node deps [Config.node, MCP.node, Skill.node] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &["@/config/config.Config", "@/mcp.MCP", "@/skill.Skill"];
