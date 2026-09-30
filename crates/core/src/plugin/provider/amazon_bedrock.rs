//! Rust port of `packages/core/src/plugin/provider/amazon-bedrock.ts`.

pub const ID: &str = "amazon-bedrock";
pub const PACKAGES: &[&str] = &["@ai-sdk/amazon-bedrock", "@ai-sdk/amazon-bedrock/mantle"];

pub fn resolve_model_id(model_id: &str, region: Option<&str>) -> String {
    if model_id.starts_with("arn:") {
        return model_id.to_string();
    }
    let prefixes = ["global.", "us.", "eu.", "jp.", "apac.", "au."];
    if prefixes.iter().any(|p| model_id.starts_with(*p)) {
        return model_id.to_string();
    }
    let resolved = region.unwrap_or("us-east-1");
    let prefix = resolved.split('-').next().unwrap_or("us");
    if prefix == "us" {
        let requires = [
            "nova-micro",
            "nova-lite",
            "nova-pro",
            "nova-premier",
            "nova-2",
            "claude",
            "deepseek.r1",
        ]
        .iter()
        .any(|k| model_id.contains(k));
        if requires && !resolved.starts_with("us-gov") {
            return format!("us.{model_id}");
        }
        return model_id.to_string();
    }
    if prefix == "eu" {
        let regions = [
            "eu-west-1",
            "eu-west-2",
            "eu-west-3",
            "eu-north-1",
            "eu-central-1",
            "eu-south-1",
            "eu-south-2",
        ];
        let region_match = regions.iter().any(|r| resolved.contains(r));
        let model_match = ["claude", "nova-lite", "nova-micro", "llama3", "pixtral"]
            .iter()
            .any(|k| model_id.contains(k));
        if region_match && model_match {
            return format!("eu.{model_id}");
        }
        return model_id.to_string();
    }
    if prefix != "ap" {
        return model_id.to_string();
    }
    let australia = ["ap-southeast-2", "ap-southeast-4"].contains(&resolved);
    if australia
        && ["anthropic.claude-sonnet-4-5", "anthropic.claude-haiku"]
            .iter()
            .any(|k| model_id.contains(k))
    {
        return format!("au.{model_id}");
    }
    let pfx = if resolved == "ap-northeast-1" {
        "jp"
    } else {
        "apac"
    };
    if ["claude", "nova-lite", "nova-micro", "nova-pro"]
        .iter()
        .any(|k| model_id.contains(k))
    {
        return format!("{pfx}.{model_id}");
    }
    model_id.to_string()
}

pub fn select_mantle_model(model_id: &str) -> &'static str {
    if model_id == "openai.gpt-oss-safeguard-20b" || model_id == "openai.gpt-oss-safeguard-120b" {
        "chat"
    } else {
        "responses"
    }
}

pub fn move_endpoint_to_url(endpoint: Option<String>) -> Option<String> {
    endpoint
}
