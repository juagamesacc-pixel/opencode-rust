//! Rust port of `packages/function/src/github.ts` (opencode v1.18.30).
//!
//! Source 14 lines. Exports: `parseRepositoryClaim`.
//!
//! 1:1 notes:
//! - `payload.repository` claim must be string else `"Repository claim is missing"`.
//! - `claim.split("/")` must yield exactly 2 non-empty parts else `"Repository claim is invalid"`.
//! - Returns `{ owner: parts[0], repo: parts[1] }` verbatim.

/// Mirrors `JWTPayload` `repository` field — `Option<String>` covers missing/non-string.
#[derive(Clone, Debug, PartialEq)]
pub struct RepositoryClaim {
    pub owner: String,
    pub repo: String,
}

/// Port of `export function parseRepositoryClaim(payload: JWTPayload)`.
///
/// Error strings verbatim:
/// - `"Repository claim is missing"` when `payload.repository` is not a string.
/// - `"Repository claim is invalid"` when `split("/")` does not yield 2 non-empty parts.
pub fn parse_repository_claim(payload: &serde_json::Value) -> Result<RepositoryClaim, String> {
    let claim = payload.get("repository");
    let claim_str = match claim {
        Some(serde_json::Value::String(s)) => s.as_str(),
        _ => return Err("Repository claim is missing".to_string()),
    };
    let parts: Vec<&str> = claim_str.split('/').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err("Repository claim is invalid".to_string());
    }
    Ok(RepositoryClaim {
        owner: parts[0].to_string(),
        repo: parts[1].to_string(),
    })
}

/// Alias matching TS export name `parseRepositoryClaim`.
pub fn parseRepositoryClaim(payload: &serde_json::Value) -> Result<RepositoryClaim, String> {
    parse_repository_claim(payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn reads_repository_identity_with_legacy_subject() {
        let payload = json!({
            "repository": "octocat/my-repo",
            "sub": "repo:octocat/my-repo:ref:refs/heads/main"
        });
        assert_eq!(
            parseRepositoryClaim(&payload).unwrap(),
            RepositoryClaim {
                owner: "octocat".into(),
                repo: "my-repo".into()
            }
        );
    }

    #[test]
    fn reads_repository_identity_with_immutable_subject() {
        let payload = json!({
            "repository": "octocat/my-repo",
            "sub": "repo:octocat@123456/my-repo@456789:ref:refs/heads/main"
        });
        assert_eq!(
            parseRepositoryClaim(&payload).unwrap(),
            RepositoryClaim {
                owner: "octocat".into(),
                repo: "my-repo".into()
            }
        );
    }

    #[test]
    fn does_not_depend_on_repository_path_in_customized_subject() {
        let payload = json!({
            "repository": "octocat/my-repo",
            "sub": "repository_owner:octocat:repository_visibility:private"
        });
        assert_eq!(
            parseRepositoryClaim(&payload).unwrap(),
            RepositoryClaim {
                owner: "octocat".into(),
                repo: "my-repo".into()
            }
        );
    }

    #[test]
    fn rejects_missing_repository_claim() {
        let payload = json!({});
        let err = parseRepositoryClaim(&payload).unwrap_err();
        assert_eq!(err, "Repository claim is missing");
    }

    #[test]
    fn rejects_invalid_repository_claim() {
        let payload = json!({ "repository": "octocat" });
        let err = parseRepositoryClaim(&payload).unwrap_err();
        assert_eq!(err, "Repository claim is invalid");
    }
}
