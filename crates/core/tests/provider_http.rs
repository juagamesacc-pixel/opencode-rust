#![allow(clippy::all)]
// hermetic: provider request-building / URL / headers / SSE parsing with fixture strings

#[test]
fn anthropic_beta_header_verbatim() {
    let mut h = std::collections::HashMap::new();
    h = core::plugin::provider::anthropic::headers_with_beta(h);
    assert_eq!(
        h.get("anthropic-beta").unwrap(),
        "interleaved-thinking-2025-05-14,fine-grained-tool-streaming-2025-05-14"
    );
    // request builder includes header
    let req = core::plugin::provider::anthropic::build_request(
        "https://api.anthropic.com/v1",
        "sk-ant",
        "claude-3",
        serde_json::json!([{"role":"user","content":"hi"}]),
        true,
    )
    .unwrap();
    assert!(req.url().as_str().contains("/messages"));
    assert_eq!(
        req.headers()
            .get("anthropic-beta")
            .unwrap()
            .to_str()
            .unwrap(),
        "interleaved-thinking-2025-05-14,fine-grained-tool-streaming-2025-05-14"
    );
}

#[test]
fn cerebras_third_party_header() {
    let h = core::plugin::provider::cerebras::headers(Default::default());
    assert_eq!(
        h.get("X-Cerebras-3rd-Party-Integration").unwrap(),
        "opencode"
    );
}

#[test]
fn openrouter_headers_and_disabled_models() {
    let h = core::plugin::provider::openrouter::headers(Default::default());
    assert_eq!(h.get("HTTP-Referer").unwrap(), "https://opencode.ai/");
    assert_eq!(h.get("X-Title").unwrap(), "opencode");
    assert!(core::plugin::provider::openrouter::is_disabled_model(
        "gpt-5-chat-latest"
    ));
    assert!(core::plugin::provider::openrouter::is_disabled_model(
        "openai/gpt-5-chat"
    ));
    assert!(!core::plugin::provider::openrouter::is_disabled_model(
        "gpt-4"
    ));
}

#[test]
fn azure_missing_resource_error_string() {
    let err = core::plugin::provider::azure::assert_resource_name(None, None, None).unwrap_err();
    assert_eq!(
        err,
        "AZURE_RESOURCE_NAME is missing, set it using env var or reconnecting the azure provider and setting it"
    );
    assert_eq!(
        core::plugin::provider::azure::cognitive_services_url("myres"),
        "https://myres.cognitiveservices.azure.com/openai"
    );
}

#[test]
fn bedrock_resolve_model_ids() {
    assert_eq!(
        core::plugin::provider::amazon_bedrock::resolve_model_id(
            "claude-3-sonnet",
            Some("us-east-1")
        ),
        "us.claude-3-sonnet"
    );
    assert_eq!(
        core::plugin::provider::amazon_bedrock::resolve_model_id(
            "arn:aws:bedrock:foo",
            Some("eu-west-1")
        ),
        "arn:aws:bedrock:foo"
    );
    assert_eq!(
        core::plugin::provider::amazon_bedrock::resolve_model_id(
            "global.claude-foo",
            Some("us-east-1")
        ),
        "global.claude-foo"
    );
}

#[test]
fn cloudflare_workers_endpoint_and_expand() {
    assert_eq!(
        core::plugin::provider::cloudflare_workers_ai::workers_endpoint("acct123"),
        "https://api.cloudflare.com/client/v4/accounts/acct123/ai/v1"
    );
    // sse parsing via provider
    let body = "data: {\"choices\":[{\"delta\":{\"content\":\"hello\"}}]}\n\ndata: {\"choices\":[{\"delta\":{\"content\":\" world\"}}]}\n\ndata: [DONE]\n";
    let chunks = core::provider::parse_sse_body(body);
    assert_eq!(chunks.len(), 2);
    assert_eq!(
        core::provider::extract_chat_text_deltas(&chunks),
        "hello world"
    );
    // non-stream parse
    let json = r#"{"choices":[{"message":{"content":"hi"},"finish_reason":"stop"}]}"#;
    let (text, finish) = core::provider::parse_chat_response(json).unwrap();
    assert_eq!(text, "hi");
    assert_eq!(finish.unwrap(), "stop");
}

#[test]
fn google_vertex_vars() {
    let v = core::plugin::provider::google_vertex::replace_vertex_vars(
        "https://${GOOGLE_VERTEX_ENDPOINT}/v1/projects/${GOOGLE_VERTEX_PROJECT}/locations/${GOOGLE_VERTEX_LOCATION}/publishers/anthropic/models",
        Some("proj1"),
        "us-central1",
    );
    assert!(v.contains("us-central1-aiplatform.googleapis.com"));
    assert!(v.contains("proj1"));
}

#[test]
fn github_copilot_endpoint_selection() {
    assert_eq!(
        core::plugin::provider::github_copilot::select_endpoint(
            true,
            true,
            Some("responses"),
            "gpt-5"
        ),
        "responses"
    );
    assert_eq!(
        core::plugin::provider::github_copilot::select_endpoint(true, true, None, "gpt-5"),
        "responses"
    );
    assert_eq!(
        core::plugin::provider::github_copilot::select_endpoint(true, true, None, "gpt-5-mini"),
        "chat"
    );
    assert_eq!(
        core::plugin::provider::github_copilot::select_endpoint(true, true, None, "gpt-4"),
        "chat"
    );
}

#[test]
fn provider_build_chat_request_auth_headers() {
    let req = core::provider::build_chat_request(
        "openai",
        "gpt-4",
        "https://api.openai.com/v1",
        Some("sk-test"),
        serde_json::json!([{"role":"user","content":"hi"}]),
        true,
        Default::default(),
        Default::default(),
    )
    .unwrap();
    assert_eq!(
        req.headers()
            .get("Authorization")
            .unwrap()
            .to_str()
            .unwrap(),
        "Bearer sk-test"
    );
    assert_eq!(
        req.url().as_str(),
        "https://api.openai.com/v1/chat/completions"
    );
}
