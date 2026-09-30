//! Rust port of `packages/core/src/tool/skill.ts`.

use serde::{Deserialize, Serialize};

pub const NAME: &str = "skill";
pub const FILE_LIMIT: usize = 10;

pub const DESCRIPTION: &str = "Load a specialized skill when the task at hand matches one of the available skills in the system context.\n\nUse this tool to inject the skill's instructions and resources into the current conversation. The output may contain detailed workflow guidance as well as references to scripts, files, etc. in the same directory as the skill.\n\nThe skill name must match one of the available skills in the system context.";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub name: String,
    pub directory: String,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInfo {
    pub name: String,
    pub location: String,
    pub content: String,
}

pub fn to_model_output(skill: &SkillInfo, files: &[String]) -> String {
    let directory = std::path::Path::new(&skill.location)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    let mut lines = vec![
        format!("<skill_content name=\"{}\">", skill.name),
        format!("# Skill: {}", skill.name),
        String::new(),
        skill.content.trim().to_string(),
        String::new(),
        format!("Base directory for this skill: {directory}"),
        "Relative paths in this skill (e.g., scripts/, reference/) are relative to this base directory.".to_string(),
        "Note: file list is sampled.".to_string(),
        String::new(),
        "<skill_files>".to_string(),
    ];
    for f in files {
        lines.push(format!("<file>{f}</file>"));
    }
    lines.push("</skill_files>".to_string());
    lines.push("</skill_content>".to_string());
    lines.join("\n")
}

// PROVISIONAL pending SkillV2 + FSUtil + PermissionV2 wiring.
