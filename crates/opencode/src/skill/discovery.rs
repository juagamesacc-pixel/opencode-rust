// source: src/skill/discovery.ts — exports: Interface, Service, node, Discovery
// PROVISIONAL pending crates/core (layer-node, app-node-platform, fs-util,
// global) + @/util/effect-http-client: consts + URL/version/staging rules verbatim.

/// source: skillConcurrency = 4 — verbatim.
pub const SKILL_CONCURRENCY: usize = 4;
/// source: fileConcurrency = 8 — verbatim.
pub const FILE_CONCURRENCY: usize = 8;

/// source: IndexSkill { name, files, version? } — verbatim.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSkill {
    pub name: String,
    pub files: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// source: Index { skills } — verbatim.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Index {
    pub skills: Vec<IndexSkill>,
}

/// source: cache subdir "skills" — verbatim.
pub const CACHE_SUBDIR: &str = "skills";
/// source: version file ".opencode-version" — verbatim.
pub const VERSION_FILE: &str = ".opencode-version";
/// source: index file "index.json" — verbatim.
pub const INDEX_FILE: &str = "index.json";
/// source: "SKILL.md" required file — verbatim.
pub const SKILL_MD: &str = "SKILL.md";

/// source: base URL trailing-slash rule — verbatim.
pub fn base_url(url: &str) -> String {
    if url.ends_with('/') {
        url.to_string()
    } else {
        format!("{}/", url)
    }
}

/// source: log strings — verbatim.
pub const LOG_FETCHING_INDEX: &str = "fetching index";
pub const LOG_FETCH_INDEX_FAILED: &str = "failed to fetch index";
pub const LOG_MISSING_SKILL_MD: &str = "skill entry missing SKILL.md";
pub const LOG_DOWNLOAD_FAILED: &str = "failed to download";
pub const LOG_REFRESH_FAILED: &str = "failed to refresh skill";

/// source: Interface — pull, verbatim.
pub trait Interface {
    fn pull(&self, url: &str) -> Vec<String>;
}

/// source: Service "@opencode/SkillDiscovery" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/SkillDiscovery";

/// source: node deps [FSUtil.node, path, httpClient] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[
    "@opencode-ai/core/fs-util.FSUtil",
    "@opencode-ai/core/effect/app-node-platform.path",
    "@opencode-ai/core/effect/app-node-platform.httpClient",
];
