//! Rust port of `packages/core/src/plugin/provider/google-vertex.ts`.

pub const ID: &str = "google-vertex";
pub const PACKAGES: &[&str] = &["@ai-sdk/google-vertex", "@ai-sdk/openai-compatible"];

pub fn resolve_project(options: Option<&str>) -> Option<String> {
    options
        .map(|s| s.to_string())
        .or_else(|| std::env::var("GOOGLE_VERTEX_PROJECT").ok())
        .or_else(|| std::env::var("GOOGLE_CLOUD_PROJECT").ok())
        .or_else(|| std::env::var("GCP_PROJECT").ok())
        .or_else(|| std::env::var("GCLOUD_PROJECT").ok())
}

pub fn resolve_location(options: Option<&str>) -> String {
    options
        .map(|s| s.to_string())
        .or_else(|| std::env::var("GOOGLE_VERTEX_LOCATION").ok())
        .or_else(|| std::env::var("GOOGLE_CLOUD_LOCATION").ok())
        .or_else(|| std::env::var("VERTEX_LOCATION").ok())
        .unwrap_or_else(|| "us-central1".to_string())
}

pub fn vertex_endpoint(location: &str) -> String {
    if location == "global" {
        "aiplatform.googleapis.com".to_string()
    } else {
        format!("{location}-aiplatform.googleapis.com")
    }
}

pub fn replace_vertex_vars(value: &str, project: Option<&str>, location: &str) -> String {
    value
        .replace(
            "${GOOGLE_VERTEX_PROJECT}",
            project.unwrap_or("${GOOGLE_VERTEX_PROJECT}"),
        )
        .replace("${GOOGLE_VERTEX_LOCATION}", location)
        .replace("${GOOGLE_VERTEX_ENDPOINT}", &vertex_endpoint(location))
}
