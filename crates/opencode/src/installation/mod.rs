// source: src/installation/index.ts — exports: Method, ReleaseType, Event,
// getReleaseType, Info, userAgent, USER_AGENT, isPreview, isLocal,
// UpgradeFailedError, Interface, Service, use, node, latest, method, upgrade,
// Installation
// PROVISIONAL pending crates/core (layer-node, app-node-builder,
// app-node-platform, process, service-use, runtime, installation/version,
// npm-config, schema) + @opencode-ai/schema/installation-event + semver:
// pure logic (getReleaseType, userAgent, upgradeFailure, method sort,
// command tables, API URLs) ported verbatim; process/http bodies as trait.

use serde::{Deserialize, Serialize};

/// source: Method — verbatim union order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Method {
    Curl,
    Npm,
    Yarn,
    Pnpm,
    Bun,
    Brew,
    Scoop,
    Choco,
    Unknown,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::Curl => "curl",
            Method::Npm => "npm",
            Method::Yarn => "yarn",
            Method::Pnpm => "pnpm",
            Method::Bun => "bun",
            Method::Brew => "brew",
            Method::Scoop => "scoop",
            Method::Choco => "choco",
            Method::Unknown => "unknown",
        }
    }
}

/// source: ReleaseType — verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseType {
    Patch,
    Minor,
    Major,
}

/// source: getReleaseType — verbatim semver major/minor compare.
pub fn get_release_type(current: (u64, u64), latest: (u64, u64)) -> ReleaseType {
    if latest.0 > current.0 {
        return ReleaseType::Major;
    }
    if latest.1 > current.1 {
        return ReleaseType::Minor;
    }
    ReleaseType::Patch
}

/// source: Info { version, latest } ("InstallationInfo") — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub version: String,
    pub latest: String,
}

/// source: userAgent(client = "cli") — `opencode/${channel}/${version}/${client}`, verbatim.
pub fn user_agent(channel: &str, version: &str, client: &str) -> String {
    format!("opencode/{}/{}/{}", channel, version, client)
}

/// source: isPreview() — channel !== "latest", verbatim.
pub fn is_preview(channel: &str) -> bool {
    channel != "latest"
}

/// source: isLocal() — channel === "local", verbatim.
pub fn is_local(channel: &str) -> bool {
    channel == "local"
}

/// source: UpgradeFailedError — message = stderr, verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeFailedError {
    pub stderr: String,
}

impl std::fmt::Display for UpgradeFailedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stderr)
    }
}

impl std::error::Error for UpgradeFailedError {}

/// source: upgradeFailure(method, result?) — verbatim branches.
pub fn upgrade_failure(method: &Method, code: Option<i32>) -> String {
    if *method == Method::Choco {
        return "not running from an elevated command shell".to_string();
    }
    match code {
        Some(c) => format!("Upgrade failed for {} (exit code {}).", method.as_str(), c),
        None => format!("Upgrade failed for {}.", method.as_str()),
    }
}

/// source: `Unknown installation method: ${m}` — verbatim.
pub fn unknown_method_message(m: &str) -> String {
    format!("Unknown installation method: {}", m)
}

/// source: method() exec-path markers — verbatim substrings.
pub const EXEC_CURL_MARKERS: &[&str] = &[".opencode/bin", ".local/bin"];

/// source: method() checks table — verbatim (name, probe argv, installedName).
pub struct MethodCheck {
    pub name: Method,
    pub probe: &'static [&'static str],
    pub installed_name: &'static str,
}

pub const METHOD_CHECKS: &[MethodCheck] = &[
    MethodCheck {
        name: Method::Npm,
        probe: &["npm", "list", "-g", "--depth=0"],
        installed_name: "opencode-ai",
    },
    MethodCheck {
        name: Method::Yarn,
        probe: &["yarn", "global", "list"],
        installed_name: "opencode-ai",
    },
    MethodCheck {
        name: Method::Pnpm,
        probe: &["pnpm", "list", "-g", "--depth=0"],
        installed_name: "opencode-ai",
    },
    MethodCheck {
        name: Method::Bun,
        probe: &["bun", "pm", "ls", "-g"],
        installed_name: "opencode-ai",
    },
    MethodCheck {
        name: Method::Brew,
        probe: &["brew", "list", "--formula", "opencode"],
        installed_name: "opencode",
    },
    MethodCheck {
        name: Method::Scoop,
        probe: &["scoop", "list", "opencode"],
        installed_name: "opencode",
    },
    MethodCheck {
        name: Method::Choco,
        probe: &["choco", "list", "--limit-output", "opencode"],
        installed_name: "opencode",
    },
];

/// source: method() exec-priority sort — exec.includes(name) first, verbatim.
pub fn sort_checks_by_exec<'a>(exec: &str, checks: &[&'a MethodCheck]) -> Vec<&'a MethodCheck> {
    let mut out: Vec<&MethodCheck> = checks.to_vec();
    out.sort_by(|a, b| {
        let am = exec.contains(a.name.as_str());
        let bm = exec.contains(b.name.as_str());
        match (am, bm) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        }
    });
    out
}

/// source: latest() endpoint URLs — verbatim.
pub const URL_GITHUB_LATEST: &str =
    "https://api.github.com/repos/anomalyco/opencode/releases/latest";
pub const URL_BREW_FORMULA: &str = "https://formulae.brew.sh/api/formula/opencode.json";
pub const URL_SCOOP_MANIFEST: &str =
    "https://raw.githubusercontent.com/ScoopInstaller/Main/master/bucket/opencode.json";
pub const URL_CHOCO: &str = "https://community.chocolatey.org/api/v2/Packages?$filter=Id%20eq%20%27opencode%27%20and%20IsLatestVersion&$select=Version";
pub const URL_INSTALL_SCRIPT: &str = "https://opencode.ai/install";

/// source: tap formula + brew env — verbatim.
pub const BREW_TAP_FORMULA: &str = "anomalyco/tap/opencode";
pub const BREW_NO_AUTO_UPDATE: (&str, &str) = ("HOMEBREW_NO_AUTO_UPDATE", "1");

/// source: upgrade() per-method argv — verbatim.
pub fn upgrade_argv(method: &Method, target: &str) -> Option<Vec<String>> {
    match method {
        Method::Npm => Some(vec![
            "npm".into(),
            "install".into(),
            "-g".into(),
            format!("opencode-ai@{}", target),
        ]),
        Method::Pnpm => Some(vec![
            "pnpm".into(),
            "install".into(),
            "-g".into(),
            format!("opencode-ai@{}", target),
        ]),
        Method::Bun => Some(vec![
            "bun".into(),
            "install".into(),
            "-g".into(),
            format!("opencode-ai@{}", target),
        ]),
        Method::Choco => Some(vec![
            "choco".into(),
            "upgrade".into(),
            "opencode".into(),
            format!("--version={}", target),
            "-y".into(),
        ]),
        Method::Scoop => Some(vec![
            "scoop".into(),
            "install".into(),
            format!("opencode@{}", target),
        ]),
        _ => None,
    }
}

/// source: Interface — info/method/latest/upgrade, verbatim.
pub trait Interface {
    fn info(&self) -> Info;
    fn method(&self) -> Method;
    fn latest(&self, method: Option<Method>) -> Result<String, UpgradeFailedError>;
    fn upgrade(&self, method: Method, target: &str) -> Result<(), UpgradeFailedError>;
}

/// source: Service "@opencode/Installation" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Installation";

/// source: node deps [httpClient, AppProcess.node] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[
    "@opencode-ai/core/effect/app-node-platform.httpClient",
    "@opencode-ai/core/process.AppProcess",
];
