//! Rust port of `packages/core/src/plugin/provider/gitlab.ts`.

pub const ID: &str = "gitlab";
pub const PACKAGE: &str = "gitlab-ai-provider";

pub fn instance_url(options: Option<&str>) -> String {
    options.map(|s| s.to_string()).unwrap_or_else(|| {
        std::env::var("GITLAB_INSTANCE_URL").unwrap_or_else(|_| "https://gitlab.com".to_string())
    })
}

pub fn api_key(options: Option<&str>) -> Option<String> {
    options
        .map(|s| s.to_string())
        .or_else(|| std::env::var("GITLAB_TOKEN").ok())
}

pub fn is_workflow_model(model_id: &str) -> bool {
    model_id.starts_with("duo-workflow-")
}

pub fn headers(installation_version: &str) -> std::collections::HashMap<String, String> {
    let mut m = std::collections::HashMap::new();
    m.insert(
        "User-Agent".to_string(),
        format!("opencode/{installation_version} gitlab-ai-provider"),
    );
    m.insert(
        "anthropic-beta".to_string(),
        "context-1m-2025-08-07".to_string(),
    );
    m
}
