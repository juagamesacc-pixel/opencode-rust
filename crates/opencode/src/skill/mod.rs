// source: src/skill/index.ts — exports: Info, InvalidError,
// NameMismatchError, NotFoundError, Interface, Service, fmt, node, Skill
// PROVISIONAL pending crates/core (layer-node, util/error, global,
// plugin/skill, fs-util, v1/config/error, util/glob) + @/agent/agent,
// @/event-v2-bridge, @/effect/*, @/permission, @/config/*, @/util/*:
// patterns + built-in skill + fmt() verbatim.

use serde::{Deserialize, Serialize};

/// source: CLAUDE_EXTERNAL_DIR — verbatim.
pub const CLAUDE_EXTERNAL_DIR: &str = ".claude";
/// source: AGENTS_EXTERNAL_DIR — verbatim.
pub const AGENTS_EXTERNAL_DIR: &str = ".agents";
/// source: EXTERNAL_SKILL_PATTERN — verbatim.
pub const EXTERNAL_SKILL_PATTERN: &str = "skills/**/SKILL.md";
/// source: OPENCODE_SKILL_PATTERN — verbatim.
pub const OPENCODE_SKILL_PATTERN: &str = "{skill,skills}/**/SKILL.md";
/// source: SKILL_PATTERN — verbatim.
pub const SKILL_PATTERN: &str = "**/SKILL.md";

/// source: CUSTOMIZE_OPENCODE_SKILL_NAME — verbatim.
pub const CUSTOMIZE_OPENCODE_SKILL_NAME: &str = "customize-opencode";
/// source: CUSTOMIZE_OPENCODE_SKILL_DESCRIPTION — verbatim.
pub const CUSTOMIZE_OPENCODE_SKILL_DESCRIPTION: &str = "Use ONLY when the user is editing or creating opencode's own configuration: opencode.json, opencode.jsonc, files under .opencode/, or files under ~/.config/opencode/. Also use when creating or fixing opencode agents, subagents, skills, plugins, MCP servers, or permission rules. Do not use for the user's own application code, or for any project that is not configuring opencode itself.";
/// source: built-in location "<built-in>" — verbatim.
pub const BUILTIN_LOCATION: &str = "<built-in>";

/// source: Info { name, description?, location, content } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub location: String,
    pub content: String,
}

/// source: InvalidError ("SkillInvalidError") — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvalidError {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// source: NameMismatchError ("SkillNameMismatchError") — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameMismatchError {
    pub path: String,
    pub expected: String,
    pub actual: String,
}

/// source: NotFoundError ("Skill.NotFoundError") — verbatim + message template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotFoundError {
    pub name: String,
    pub available: Vec<String>,
}

impl std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Skill \"{}\" not found. Available skills: {}",
            self.name,
            if self.available.is_empty() {
                "none".to_string()
            } else {
                self.available.join(", ")
            }
        )
    }
}

impl std::error::Error for NotFoundError {}

/// source: isSkillFrontmatter — record with string name + optional string
/// description. Verbatim rule.
pub fn is_skill_frontmatter(name: Option<&str>, description: Option<&str>) -> bool {
    name.is_some() && description.map(|d| !d.is_empty() || true).unwrap_or(true)
}

/// source: Interface — get/require/all/dirs/available, verbatim.
pub trait Interface {
    fn get(&self, name: &str) -> Option<Info>;
    fn require(&self, name: &str) -> Result<Info, NotFoundError>;
    fn all(&self) -> Vec<Info>;
    fn dirs(&self) -> Vec<String>;
}

/// source: fmt() — verbatim templates (verbose XML + markdown list).
pub fn fmt(list: &[Info], verbose: bool) -> String {
    let mut described: Vec<&Info> = list.iter().filter(|s| s.description.is_some()).collect();
    if described.is_empty() {
        return "No skills are currently available.".to_string();
    }
    described.sort_by(|a, b| a.name.cmp(&b.name));
    if verbose {
        let mut out = vec!["<available_skills>".to_string()];
        for skill in described {
            out.push("  <skill>".to_string());
            out.push(format!("    <name>{}</name>", skill.name));
            out.push(format!(
                "    <description>{}</description>",
                skill.description.as_deref().unwrap_or("")
            ));
            out.push(format!(
                "    <location>{}</location>",
                crate::util::html::escape_html(&skill.location)
            ));
            out.push("  </skill>".to_string());
        }
        out.push("</available_skills>".to_string());
        return out.join("\n");
    }
    let mut out = vec!["## Available Skills".to_string()];
    for skill in described {
        out.push(format!(
            "- **{}**: {}",
            skill.name,
            skill.description.as_deref().unwrap_or("")
        ));
    }
    out.join("\n")
}

/// source: "skill path not found" — verbatim.
pub const LOG_SKILL_PATH_NOT_FOUND: &str = "skill path not found";
/// source: "duplicate skill name" — verbatim.
pub const LOG_DUPLICATE: &str = "duplicate skill name";
/// source: "failed to load skill" — verbatim.
pub const LOG_LOAD_FAILED: &str = "failed to load skill";
/// source: `Failed to parse skill ${match}` — verbatim.
pub fn parse_failed_message(m: &str) -> String {
    format!("Failed to parse skill {}", m)
}
/// source: `failed to scan ${scope} skills` — verbatim.
pub fn scan_failed_message(scope: &str) -> String {
    format!("failed to scan {} skills", scope)
}

/// source: Service "@opencode/Skill" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Skill";

/// source: node deps — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[
    "@/skill/discovery.Discovery",
    "@/config/config.Config",
    "@/event-v2-bridge.EventV2Bridge",
    "@opencode-ai/core/fs-util.FSUtil",
    "@opencode-ai/core/global.Global",
    "@/effect/runtime-flags.RuntimeFlags",
];

pub mod discovery;
