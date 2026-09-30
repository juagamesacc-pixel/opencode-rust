// source: src/util/repository.ts — exports: RemoteReference, FileReference,
// Reference, InvalidRepositoryReferenceError, UnsupportedLocalRepositoryError,
// InvalidRepositoryBranchError, RepositoryError, isRepositoryError,
// parseRepositoryReference, isFileRepositoryReference,
// isRemoteRepositoryReference, parseRemoteRepositoryReference,
// validateRepositoryBranch, parseGitHubRemote, repositoryCachePath,
// repositoryCacheIdentity, sameRepositoryReference
// PROVISIONAL pending crates/core (global Path.repos): pure parse logic verbatim.

use serde::{Deserialize, Serialize};

/// source: BaseReference — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseReference {
    pub host: String,
    pub path: String,
    pub segments: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    pub repo: String,
    pub remote: String,
    pub label: String,
}

/// source: RemoteReference — verbatim (+ optional protocol).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteReference {
    #[serde(flatten)]
    pub base: BaseReference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// source: FileReference — verbatim (host "file", protocol "file:").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileReference {
    #[serde(flatten)]
    pub base: BaseReference,
}

/// source: Reference — verbatim union.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Reference {
    Remote(RemoteReference),
    File(FileReference),
}

impl Reference {
    pub fn host(&self) -> &str {
        match self {
            Reference::Remote(r) => &r.base.host,
            Reference::File(f) => &f.base.host,
        }
    }
    pub fn path(&self) -> &str {
        match self {
            Reference::Remote(r) => &r.base.path,
            Reference::File(f) => &f.base.path,
        }
    }
    pub fn segments(&self) -> &[String] {
        match self {
            Reference::Remote(r) => &r.base.segments,
            Reference::File(f) => &f.base.segments,
        }
    }
    pub fn protocol(&self) -> Option<&str> {
        match self {
            Reference::Remote(r) => r.protocol.as_deref(),
            Reference::File(_) => Some("file:"),
        }
    }
}

/// source: InvalidRepositoryReferenceError ("RepositoryInvalidReferenceError") — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvalidRepositoryReferenceError {
    pub repository: String,
    pub message: String,
}

/// source: UnsupportedLocalRepositoryError — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsupportedLocalRepositoryError {
    pub repository: String,
    pub message: String,
}

/// source: InvalidRepositoryBranchError — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvalidRepositoryBranchError {
    pub branch: String,
    pub message: String,
}

/// source: RepositoryError union — verbatim members.
#[derive(Debug, Clone)]
pub enum RepositoryError {
    InvalidReference(InvalidRepositoryReferenceError),
    UnsupportedLocal(UnsupportedLocalRepositoryError),
    InvalidBranch(InvalidRepositoryBranchError),
}

/// source: "Repository must be a git URL, host/path reference, or GitHub owner/repo shorthand" — verbatim.
pub const INVALID_REF_MESSAGE: &str =
    "Repository must be a git URL, host/path reference, or GitHub owner/repo shorthand";
/// source: "Local file repositories are not supported" — verbatim.
pub const UNSUPPORTED_LOCAL_MESSAGE: &str = "Local file repositories are not supported";
/// source: branch message — verbatim.
pub const INVALID_BRANCH_MESSAGE: &str =
    "Branch must contain only alphanumeric characters, /, _, ., and -, and cannot start with - or contain ..";

fn normalize_repository_input(input: &str) -> String {
    // source: trim, strip ^git\+, strip #fragment, strip trailing slashes. Verbatim.
    let mut s = input.trim().to_string();
    if let Some(stripped) = s.strip_prefix("git+") {
        s = stripped.to_string();
    }
    if let Some(pos) = s.find('#') {
        s.truncate(pos);
    }
    while s.ends_with('/') && !s.is_empty() {
        s.pop();
    }
    s
}

fn trim_git_suffix(input: &str) -> String {
    // source: /\.git$/ → "". Verbatim.
    match input.strip_suffix(".git") {
        Some(s) => s.to_string(),
        None => input.to_string(),
    }
}

fn parts(input: &str) -> Vec<String> {
    // source: split "/", trim, trimGitSuffix, filter Boolean. Verbatim.
    input
        .split('/')
        .map(|i| trim_git_suffix(i.trim()))
        .filter(|s| !s.is_empty())
        .collect()
}

fn safe_host(input: &str) -> bool {
    // source: Boolean && !startsWith("-") && !/[\s/\\]/. Verbatim.
    !input.is_empty()
        && !input.starts_with('-')
        && !input
            .chars()
            .any(|c| c.is_whitespace() || c == '/' || c == '\\')
}

fn safe_segment(input: &str) -> bool {
    // source: !== "." && !== ".." && !includes(":") && !/[\s/\\]/. Verbatim.
    input != "."
        && input != ".."
        && !input.contains(':')
        && !input
            .chars()
            .any(|c| c.is_whitespace() || c == '/' || c == '\\')
}

fn host_like(input: &str) -> bool {
    // source: includes "." || includes ":" || === "localhost". Verbatim.
    input.contains('.') || input.contains(':') || input == "localhost"
}

fn with_slash(input: &str) -> String {
    if input.ends_with('/') {
        input.to_string()
    } else {
        format!("{}/", input)
    }
}

/// source: githubRemote — OPENCODE_REPO_CLONE_GITHUB_BASE_URL override, verbatim.
pub fn github_remote(pathname: &str, base_url: Option<&str>) -> String {
    match base_url {
        None => format!("https://github.com/{}.git", pathname),
        Some(base) => format!("{}{}.git", with_slash(base), pathname),
    }
}

fn build_remote_reference(
    host: &str,
    segments: Vec<String>,
    remote: Option<String>,
    protocol: Option<String>,
    github_base: Option<&str>,
) -> Option<RemoteReference> {
    let segments: Vec<String> = segments
        .into_iter()
        .map(|s| trim_git_suffix(&s))
        .filter(|s| !s.is_empty())
        .collect();
    if !safe_host(host) || segments.is_empty() || segments.iter().any(|s| !safe_segment(s)) {
        return None;
    }
    let pathname = segments.join("/");
    let repo = segments[segments.len() - 1].clone();
    let host = host.to_lowercase();
    let owner = if segments.len() == 2 {
        Some(segments[0].clone())
    } else {
        None
    };
    let default_remote = if host == "github.com" {
        github_remote(&pathname, github_base)
    } else {
        format!("https://{}/{}.git", host, pathname)
    };
    let label = if host == "github.com" && segments.len() == 2 {
        pathname.clone()
    } else {
        format!("{}/{}", host, pathname)
    };
    Some(RemoteReference {
        base: BaseReference {
            host,
            path: pathname,
            segments,
            owner,
            repo,
            remote: remote.unwrap_or(default_remote),
            label,
        },
        protocol,
    })
}

/// source: parseRepositoryReference — verbatim branch order.
pub fn parse_repository_reference(input: &str, github_base: Option<&str>) -> Option<Reference> {
    let cleaned = normalize_repository_input(input);
    if cleaned.is_empty() {
        return None;
    }
    // github:owner/repo
    if let Some(rest) = cleaned.strip_prefix("github:") {
        let mut it = rest.split('/');
        match (it.next(), it.next(), it.next()) {
            (Some(o), Some(r), None)
                if !o.is_empty()
                    && !r.is_empty()
                    && !o.contains(char::is_whitespace)
                    && !r.contains(char::is_whitespace) =>
            {
                return build_remote_reference(
                    "github.com",
                    vec![o.to_string(), r.to_string()],
                    None,
                    None,
                    github_base,
                )
                .map(Reference::Remote);
            }
            _ => {}
        }
    }
    if !cleaned.contains("://") {
        // scp-like [user@]host:path
        if let Some(colon) = cleaned.find(':') {
            let (left, right) = (&cleaned[..colon], &cleaned[colon + 1..]);
            if !left.contains('@') || left.contains('/') || left.contains(char::is_whitespace) {
                // source regex: /^(?:[^@/\s]+@)?([^:/\s]+):(.+)$/
                let host = left.rsplit('@').next().unwrap_or(left);
                if !host.contains('/') && !host.contains(char::is_whitespace) && !host.is_empty() {
                    if let Some(r) = build_remote_reference(
                        host,
                        parts(right),
                        Some(cleaned.clone()),
                        None,
                        github_base,
                    ) {
                        return Some(Reference::Remote(r));
                    }
                }
            } else {
                let host = left.rsplit('@').next().unwrap_or(left);
                if let Some(r) = build_remote_reference(
                    host,
                    parts(right),
                    Some(cleaned.clone()),
                    None,
                    github_base,
                ) {
                    return Some(Reference::Remote(r));
                }
            }
        }
        let direct = parts(&cleaned);
        if direct.len() >= 2 && host_like(&direct[0]) {
            if let Some(r) = build_remote_reference(
                &direct[0].clone(),
                direct[1..].to_vec(),
                None,
                None,
                github_base,
            ) {
                return Some(Reference::Remote(r));
            }
        }
        if direct.len() == 2 {
            if let Some(r) = build_remote_reference("github.com", direct, None, None, github_base) {
                return Some(Reference::Remote(r));
            }
        }
    }
    // URL form
    parse_url_reference(&cleaned, github_base)
}

fn parse_url_reference(cleaned: &str, github_base: Option<&str>) -> Option<Reference> {
    let (scheme, rest) = cleaned.split_once("://")?;
    let protocol = format!("{}:", scheme);
    if protocol == "file:" {
        let path = rest.to_string();
        let segments: Vec<String> = path
            .split(['/', '\\'])
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_end_matches(':').to_string())
            .collect();
        if segments.is_empty() {
            return None;
        }
        let repo = trim_git_suffix(&segments[segments.len() - 1]);
        return Some(Reference::File(FileReference {
            base: BaseReference {
                host: "file".to_string(),
                path: path.clone(),
                segments,
                owner: None,
                repo,
                remote: cleaned.to_string(),
                label: path,
            },
        }));
    }
    let (host, pathname) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, "/"),
    };
    if host.is_empty() {
        return None;
    }
    let segs = parts(pathname);
    let remote = if host == "github.com" {
        github_remote(&segs.join("/"), github_base)
    } else {
        cleaned.to_string()
    };
    build_remote_reference(host, segs, Some(remote), Some(protocol), github_base)
        .map(Reference::Remote)
}

/// source: isFileRepositoryReference — protocol === "file:", verbatim.
pub fn is_file_reference(r: &Reference) -> bool {
    r.protocol() == Some("file:")
}

/// source: isRemoteRepositoryReference — verbatim negation.
pub fn is_remote_reference(r: &Reference) -> bool {
    !is_file_reference(r)
}

/// source: parseRemoteRepositoryReference — verbatim error branches.
pub fn parse_remote_reference(
    input: &str,
    github_base: Option<&str>,
) -> Result<RemoteReference, RepositoryError> {
    match parse_repository_reference(input, github_base) {
        None => Err(RepositoryError::InvalidReference(
            InvalidRepositoryReferenceError {
                repository: input.to_string(),
                message: INVALID_REF_MESSAGE.to_string(),
            },
        )),
        Some(Reference::File(_)) => Err(RepositoryError::UnsupportedLocal(
            UnsupportedLocalRepositoryError {
                repository: input.to_string(),
                message: UNSUPPORTED_LOCAL_MESSAGE.to_string(),
            },
        )),
        Some(Reference::Remote(r)) => Ok(r),
    }
}

/// source: validateRepositoryBranch — verbatim regex + rules.
pub fn validate_repository_branch(branch: &str) -> Result<(), RepositoryError> {
    let ok_chars = branch
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '/' || c == '_' || c == '.' || c == '-');
    if !ok_chars || branch.starts_with('-') || branch.contains("..") {
        return Err(RepositoryError::InvalidBranch(
            InvalidRepositoryBranchError {
                branch: branch.to_string(),
                message: INVALID_BRANCH_MESSAGE.to_string(),
            },
        ));
    }
    Ok(())
}

/// source: parseGitHubRemote — verbatim gates.
pub fn parse_github_remote(input: &str, github_base: Option<&str>) -> Option<(String, String)> {
    let cleaned = normalize_repository_input(input);
    if !cleaned.contains("://")
        && !cleaned.starts_with("github.com:")
        && !cleaned
            .split('@')
            .next_back()
            .map(|h| h.starts_with("github.com:"))
            .unwrap_or(false)
    {
        return None;
    }
    match parse_repository_reference(&cleaned, github_base) {
        Some(Reference::Remote(r))
            if r.base.host == "github.com"
                && r.base.owner.is_some()
                && r.base.segments.len() == 2 =>
        {
            Some((r.base.owner.unwrap(), r.base.repo))
        }
        _ => None,
    }
}

/// source: repositoryCachePath — join(repos, ...host.split(":"), ...segments), verbatim.
pub fn repository_cache_path(repos: &str, r: &Reference) -> String {
    let mut p = repos.to_string();
    for s in r
        .host()
        .split(':')
        .chain(r.segments().iter().map(|s| s.as_str()))
    {
        p.push('/');
        p.push_str(s);
    }
    p
}

/// source: repositoryCacheIdentity — `${host}/${path}`, verbatim.
pub fn repository_cache_identity(r: &Reference) -> String {
    format!("{}/{}", r.host(), r.path())
}

/// source: sameRepositoryReference — verbatim.
pub fn same_repository_reference(left: &Reference, right: &Reference) -> bool {
    repository_cache_identity(left) == repository_cache_identity(right)
}
