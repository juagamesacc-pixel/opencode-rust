//! Rust port of `packages/script/src/index.ts` (opencode v1.18.30).
//!
//! Source 77 lines. Exports: `Script` (channel/version/preview/release/team).
//!
//! 1:1 notes:
//! - `rootPkgPath = path.resolve(import.meta.dir, "../../../package.json")` + `Bun.file(...).json()` → `packageManager` field.
//! - `expectedBunVersion = rootPkg.packageManager?.split("@")[1]` + throw `"packageManager field not found in root package.json"` if missing.
//! - `expectedBunVersionRange = "^" + expectedBunVersion` + `semver.satisfies(process.versions.bun, range)` else throw
//!   `"This script requires bun@{range}, but you are using bun@{bunVersion}"`.
//! - `env = { OPENCODE_CHANNEL, OPENCODE_BUMP, OPENCODE_VERSION, OPENCODE_RELEASE }` keys verbatim.
//! - `CHANNEL = env.OPENCODE_CHANNEL ?? (env.OPENCODE_BUMP ? "latest" : env.OPENCODE_VERSION && !startsWith("0.0.0-") ? "latest" : git branch --show-current)`.
//! - `IS_PREVIEW = CHANNEL !== "latest"`.
//! - `VERSION = env.OPENCODE_VERSION ?? (IS_PREVIEW ? "0.0.0-{CHANNEL}-{YYYYMMDDHHmm}" : npm registry latest + bump)` with
//!   `major+1.0.0`/`major.minor+1.0`/`major.minor.patch+1` branches.
//! - `bot = ["actions-user","opencode","opencode-agent[bot]"]`; `TEAM_MEMBERS` file split `/\r?\n/`, trim, filter empty/`#`, + bot.
//! - `Script` getters: `channel→CHANNEL`, `version→VERSION`, `preview→IS_PREVIEW`, `release→!!OPENCODE_RELEASE`, `team→team`.
//! - `console.log("opencode script", JSON.stringify(Script,null,2))`.
//!
//! PROVISIONAL: `Bun.file`, `semver`, `process.*`, `fetch npm registry`, `$` git are runtime surfaces — descriptor only.

// ---------------------------------------------------------------------------
// Env keys verbatim
// ---------------------------------------------------------------------------

pub const ENV_CHANNEL: &str = "OPENCODE_CHANNEL";
pub const ENV_BUMP: &str = "OPENCODE_BUMP";
pub const ENV_VERSION: &str = "OPENCODE_VERSION";
pub const ENV_RELEASE: &str = "OPENCODE_RELEASE";

/// Error string verbatim: `"packageManager field not found in root package.json"`.
pub const ERR_NO_PACKAGE_MANAGER: &str = "packageManager field not found in root package.json";

/// Port of `semver.satisfies` error prefix verbatim.
pub fn err_bun_version_mismatch(range: &str, bun_version: &str) -> String {
    format!(
        "This script requires bun@{}, but you are using bun@{}",
        range, bun_version
    )
}

/// Bot list verbatim.
pub const BOT_MEMBERS: &[&str] = &["actions-user", "opencode", "opencode-agent[bot]"];
/// Team file path verbatim: `"../../../.github/TEAM_MEMBERS"` relative to `src/index.ts`.
pub const TEAM_MEMBERS_PATH: &str = "../../../.github/TEAM_MEMBERS";

/// Latest channel verbatim.
pub const CHANNEL_LATEST: &str = "latest";

/// Log prefix verbatim: `"opencode script"`.
pub const LOG_PREFIX: &str = "opencode script";

/// NPM registry URL verbatim: `"https://registry.npmjs.org/opencode-ai/latest"`.
pub const NPM_LATEST_URL: &str = "https://registry.npmjs.org/opencode-ai/latest";

/// Preview version prefix: `"0.0.0-"`.
pub const PREVIEW_PREFIX: &str = "0.0.0-";

/// Port of `expectedBunVersionRange = "^" + expectedBunVersion`.
pub fn bun_version_range(expected: &str) -> String {
    format!("^{}", expected)
}

/// Port of `expectedBunVersion = packageManager?.split("@")[1]` — returns Err if missing.
pub fn parse_package_manager(package_manager: Option<&str>) -> Result<String, String> {
    match package_manager {
        Some(pm) => match pm.split('@').nth(1) {
            Some(v) if !v.is_empty() => Ok(v.to_string()),
            _ => Err(ERR_NO_PACKAGE_MANAGER.to_string()),
        },
        None => Err(ERR_NO_PACKAGE_MANAGER.to_string()),
    }
}

/// Port of `CHANNEL` resolution.
///
/// Pseudocode:
/// ```ts
/// if (env.OPENCODE_CHANNEL) return env.OPENCODE_CHANNEL
/// if (env.OPENCODE_BUMP) return "latest"
/// if (env.OPENCODE_VERSION && !startsWith("0.0.0-")) return "latest"
/// return await $`git branch --show-current`.text().trim()
/// ```
pub fn resolve_channel(
    open_code_channel: Option<&str>,
    open_code_bump: Option<&str>,
    open_code_version: Option<&str>,
    git_branch: &str,
) -> String {
    if let Some(ch) = open_code_channel {
        if !ch.is_empty() {
            return ch.to_string();
        }
    }
    if let Some(b) = open_code_bump {
        if !b.is_empty() {
            return CHANNEL_LATEST.to_string();
        }
    }
    if let Some(v) = open_code_version {
        if !v.is_empty() && !v.starts_with(PREVIEW_PREFIX) {
            return CHANNEL_LATEST.to_string();
        }
    }
    git_branch.trim().to_string()
}

/// Port of `IS_PREVIEW = CHANNEL !== "latest"`.
pub fn is_preview(channel: &str) -> bool {
    channel != CHANNEL_LATEST
}

/// Port of preview version `0.0.0-{CHANNEL}-{YYYYMMDDHHmm}` timestamp slice `toISOString().slice(0,16).replace(/[-:T]/g,"")`.
pub fn preview_version(channel: &str, iso_timestamp: &str) -> String {
    // iso_timestamp expected like "2026-09-12T09:12:00.000Z" → slice(0,16) → "2026-09-12T09:12"
    let slice = if iso_timestamp.len() >= 16 {
        &iso_timestamp[..16]
    } else {
        iso_timestamp
    };
    let compact: String = slice
        .chars()
        .filter(|c| *c != '-' && *c != ':' && *c != 'T')
        .collect();
    format!("{}{}-{}", PREVIEW_PREFIX, channel, compact)
}

/// Port of non-preview version bump from npm latest.
///
/// `t = OPENCODE_BUMP?.toLowerCase()`; if `major` → `major+1.0.0`, if `minor` → `major.minor+1.0` else `major.minor.patch+1`.
pub fn bump_version(latest: &str, open_code_bump: Option<&str>) -> String {
    let parts: Vec<u32> = latest
        .split('.')
        .map(|x| x.parse::<u32>().unwrap_or(0))
        .collect();
    let major = parts.first().copied().unwrap_or(0);
    let minor = parts.get(1).copied().unwrap_or(0);
    let patch = parts.get(2).copied().unwrap_or(0);
    let t = open_code_bump.map(|s| s.to_lowercase()).unwrap_or_default();
    if t == "major" {
        return format!("{}.0.0", major + 1);
    }
    if t == "minor" {
        return format!("{}.{}.0", major, minor + 1);
    }
    format!("{}.{}.{}", major, minor, patch + 1)
}

/// Port of `VERSION` resolution dispatch.
///
/// Returns `env.OPENCODE_VERSION` if present else preview or npm+bump.
pub fn resolve_version(
    open_code_version: Option<&str>,
    channel: &str,
    iso_timestamp: &str,
    npm_latest: &str,
    open_code_bump: Option<&str>,
) -> String {
    if let Some(v) = open_code_version {
        if !v.is_empty() {
            return v.to_string();
        }
    }
    if is_preview(channel) {
        return preview_version(channel, iso_timestamp);
    }
    bump_version(npm_latest, open_code_bump)
}

/// Port of team-member parsing: `split(/\r?\n/).map(trim).filter(x=>x && !startsWith("#"))`.
pub fn parse_team_members(content: &str) -> Vec<String> {
    content
        .split(['\n', '\r'])
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty() && !x.starts_with('#'))
        .collect()
}

/// Full team list including bots verbatim.
pub fn team_with_bots(members: Vec<String>) -> Vec<String> {
    let mut out = members;
    for b in BOT_MEMBERS {
        out.push(b.to_string());
    }
    out
}

/// Mirrors `export const Script = { get channel, get version, get preview, get release, get team }`.
#[derive(Clone, Debug, PartialEq)]
pub struct Script {
    pub channel: String,
    pub version: String,
    pub preview: bool,
    pub release: bool,
    pub team: Vec<String>,
}

impl Script {
    pub fn new(
        channel: String,
        version: String,
        preview: bool,
        release: bool,
        team: Vec<String>,
    ) -> Self {
        Self {
            channel,
            version,
            preview,
            release,
            team,
        }
    }
}
