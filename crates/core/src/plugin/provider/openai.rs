//! Rust port of `packages/core/src/plugin/provider/openai.ts`.

pub const ID: &str = "openai";
pub const PACKAGE: &str = "@ai-sdk/openai";
pub const BASE_URL: &str = "https://api.openai.com/v1";
pub const CLIENT_ID: &str = "app_EMoamEEZ73f0CkXaXp7hrann";
pub const ISSUER: &str = "https://auth.openai.com";
pub const CALLBACK_PORT: u16 = 1455;

pub fn headers(content_type: &str, version: &str) -> std::collections::HashMap<String, String> {
    let mut m = std::collections::HashMap::new();
    m.insert("Content-Type".to_string(), content_type.to_string());
    m.insert("User-Agent".to_string(), format!("opencode/{version}"));
    m
}

pub fn authorize_url(redirect: &str, challenge: &str, state: &str) -> String {
    format!("{ISSUER}/oauth/authorize?response_type=code&client_id={CLIENT_ID}&redirect_uri={redirect}&scope=openid%20profile%20email%20offline_access&code_challenge={challenge}&code_challenge_method=S256&id_token_add_organizations=true&codex_cli_simplified_flow=true&state={state}&originator=opencode")
}

pub fn is_gpt_5_chat_disabled(model_id: &str) -> bool {
    model_id == "gpt-5-chat-latest"
}
