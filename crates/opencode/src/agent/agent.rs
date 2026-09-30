// source: src/agent/agent.ts — exports: Info, Interface, Service, use,
// node, Agent
// PROVISIONAL pending crates/core (layer-node, v1/permission, schema,
// provider, model, location-services, location, reference, plugin, global)
// + ai sdk + @/*: Info shape, built-in agent table (build/plan/general/
// explore/compaction/title/summary with descriptions/modes/prompts/temps),
// defaults ruleset, user-merge rules, Truncate.GLOB rule, list sort,
// defaultInfo error strings verbatim; generate as trait.

use serde::{Deserialize, Serialize};

/// source: include_str! passthroughs for prompt assets (verbatim bytes).
pub const PROMPT_GENERATE: &str = include_str!("generate.txt");
pub const PROMPT_COMPACTION: &str = include_str!("prompt/compaction.txt");
pub const PROMPT_EXPLORE: &str = include_str!("prompt/explore.txt");
pub const PROMPT_SUMMARY: &str = include_str!("prompt/summary.txt");
pub const PROMPT_TITLE: &str = include_str!("prompt/title.txt");

/// source: Info { name, description?, mode, native?, hidden?, topP?,
/// temperature?, color?, permission, model?, variant?, prompt?, options, steps? } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    pub permission: Vec<crate::permission::Rule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<ModelRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    pub options: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<f64>,
}

/// source: model { modelID, providerID } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelRef {
    pub model_id: String,
    pub provider_id: String,
}

/// source: mode literals — verbatim.
pub const MODE_SUBAGENT: &str = "subagent";
pub const MODE_PRIMARY: &str = "primary";
pub const MODE_ALL: &str = "all";

/// source: built-in agent names — verbatim.
pub const AGENT_BUILD: &str = "build";
pub const AGENT_PLAN: &str = "plan";
pub const AGENT_GENERAL: &str = "general";
pub const AGENT_EXPLORE: &str = "explore";
pub const AGENT_COMPACTION: &str = "compaction";
pub const AGENT_TITLE: &str = "title";
pub const AGENT_SUMMARY: &str = "summary";

/// source: build description — verbatim.
pub const BUILD_DESCRIPTION: &str =
    "The default agent. Executes tools based on configured permissions.";
/// source: plan description — verbatim.
pub const PLAN_DESCRIPTION: &str = "Plan mode. Disallows all edit tools.";
/// source: general description — verbatim.
pub const GENERAL_DESCRIPTION: &str = "General-purpose agent for researching complex questions and executing multi-step tasks. Use this agent to execute multiple units of work in parallel.";
/// source: explore description — verbatim.
pub const EXPLORE_DESCRIPTION: &str = "Fast agent specialized for exploring codebases. Use this when you need to quickly find files by patterns (eg. \"src/components/**/*.tsx\"), search code for keywords (eg. \"API endpoints\"), or answer questions about the codebase (eg. \"how do API endpoints work?\"). When calling this agent, specify the desired thoroughness level: \"quick\" for basic searches, \"medium\" for moderate exploration, or \"very thorough\" for comprehensive analysis across multiple locations and naming conventions.";
/// source: title temperature 0.5 — verbatim.
pub const TITLE_TEMPERATURE: f64 = 0.5;
/// source: generate temperature 0.3 — verbatim.
pub const GENERATE_TEMPERATURE: f64 = 0.3;

/// source: default permission ruleset entries — verbatim.
pub const DEFAULT_RULES: &[(&str, &str, &str)] = &[
    ("*", "*", "allow"),
    ("doom_loop", "*", "ask"),
    ("external_directory", "*", "ask"),
    ("question", "*", "deny"),
    ("plan_enter", "*", "deny"),
    ("plan_exit", "*", "deny"),
    ("read", "*", "allow"),
    ("read", "*.env", "ask"),
    ("read", "*.env.*", "ask"),
    ("read", "*.env.example", "allow"),
];

/// source: defaultInfo errors — verbatim templates.
pub fn default_not_found_message(name: &str) -> String {
    format!("default agent \"{}\" not found", name)
}
pub fn default_is_subagent_message(name: &str) -> String {
    format!("default agent \"{}\" is a subagent", name)
}
pub fn default_is_hidden_message(name: &str) -> String {
    format!("default agent \"{}\" is hidden", name)
}
/// source: "no primary visible agent found" — verbatim.
pub const NO_VISIBLE_AGENT_MESSAGE: &str = "no primary visible agent found";

/// source: list() sort — default_agent (or "build") first desc, then name asc. Verbatim.
pub fn sort_agents(names: &mut [String], default_agent: Option<&str>) {
    names.sort_by(|a, b| {
        let fa = Some(a.as_str()) == default_agent.or(Some("build"));
        let fb = Some(b.as_str()) == default_agent.or(Some("build"));
        fb.cmp(&fa).then_with(|| a.cmp(b))
    });
}

/// source: generate prompt template — verbatim.
pub fn generate_user_prompt(description: &str, existing: &[String]) -> String {
    format!(
        "Create an agent configuration based on this request: \"{}\".\n\nIMPORTANT: The following identifiers already exist and must NOT be used: {}\n  Return ONLY the JSON object, no other text, do not wrap in backticks",
        description,
        existing.join(", ")
    )
}

/// source: Interface — get/list/defaultInfo/defaultAgent/generate, verbatim.
pub trait Interface {
    fn get(&self, agent: &str) -> Option<Info>;
    fn list(&self) -> Vec<Info>;
    fn default_info(&self, default_agent: Option<&str>) -> Result<Info, String>;
    fn default_agent(&self, default_agent: Option<&str>) -> Result<String, String>;
}

/// source: Service "@opencode/Agent" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Agent";

/// source: node deps — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[
    "@/config/config.Config",
    "@/auth.Auth",
    "@/plugin.Plugin",
    "@/skill.Skill",
    "@/provider/provider.Provider",
    "locationServiceMapNode",
];
