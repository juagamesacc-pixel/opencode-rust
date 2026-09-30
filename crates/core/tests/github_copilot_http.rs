#![allow(clippy::all)]
// hermetic: copilot endpoints, request-building, SSE-stream parsing with fixture strings

#[test]
fn copilot_base_url_and_headers() {
    let settings = core::github_copilot::copilot_provider::OpenaiCompatibleProviderSettings {
        api_key: Some("tok".to_string()),
        base_url: Some("https://api.githubcopilot.com".to_string()),
        name: Some("github-copilot".to_string()),
        headers: None,
    };
    let h = core::github_copilot::copilot_provider::build_headers(&settings);
    assert_eq!(h.get("Authorization").unwrap(), "Bearer tok");
    // User-Agent suffix
    assert!(h
        .get("User-Agent")
        .unwrap()
        .contains("ai-sdk/openai-compatible/0.1.0"));
    assert_eq!(
        core::github_copilot::copilot_provider::base_url(&settings),
        "https://api.githubcopilot.com"
    );
    assert_eq!(
        core::github_copilot::copilot_provider::chat_url(
            &core::github_copilot::copilot_provider::base_url(&settings)
        ),
        "https://api.githubcopilot.com/chat/completions"
    );
    assert_eq!(
        core::github_copilot::copilot_provider::responses_url(
            &core::github_copilot::copilot_provider::base_url(&settings)
        ),
        "https://api.githubcopilot.com/responses"
    );
}

#[test]
fn copilot_chat_build_request_verbatim() {
    let settings = core::github_copilot::copilot_provider::OpenaiCompatibleProviderSettings {
        api_key: Some("ghp_123".to_string()),
        base_url: None,
        ..Default::default()
    };
    let body = serde_json::json!({"model":"gpt-4","messages":[{"role":"user","content":"hi"}],"stream":true});
    let req = core::github_copilot::copilot_provider::build_chat_request(
        &settings,
        "gpt-4",
        body.clone(),
        Default::default(),
    )
    .unwrap();
    assert!(req.url().as_str().contains("/chat/completions"));
    assert_eq!(
        req.headers()
            .get("Authorization")
            .unwrap()
            .to_str()
            .unwrap(),
        "Bearer ghp_123"
    );
}

#[test]
fn copilot_sse_parse() {
    let body = "data: {\"choices\":[{\"delta\":{\"content\":\"hello\"}}]}\n\ndata: {\"choices\":[{\"delta\":{\"content\":\" world\"}}]}\n\ndata: [DONE]\n";
    let chunks = core::github_copilot::copilot_provider::parse_chat_sse(body);
    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0]["choices"][0]["delta"]["content"], "hello");
}

#[test]
fn copilot_chat_chunk_helpers() {
    let line = "data: {\"id\":\"x\",\"choices\":[{\"delta\":{\"content\":\"hi\",\"reasoning_text\":\"think\"},\"finish_reason\":null}]}";
    let chunk =
        core::github_copilot::chat::openai_compatible_chat_language_model::parse_chunk(line)
            .unwrap();
    assert_eq!(
        chunk.choices[0]
            .delta
            .as_ref()
            .unwrap()
            .content
            .as_deref()
            .unwrap(),
        "hi"
    );
    assert_eq!(
        core::github_copilot::chat::openai_compatible_chat_language_model::map_finish_reason(Some(
            "stop"
        )),
        "stop"
    );
    assert!(
        core::github_copilot::chat::openai_compatible_chat_language_model::is_parsable_json(
            "{\"a\":1}"
        )
    );
    assert!(
        !core::github_copilot::chat::openai_compatible_chat_language_model::is_parsable_json(
            "{a:1}"
        )
    );
}

#[test]
fn responses_model_helpers() {
    assert_eq!(
        core::github_copilot::responses::openai_responses_language_model::map_responses_finish(
            Some("max_output_tokens"),
            false
        ),
        "length"
    );
    assert_eq!(
        core::github_copilot::responses::openai_responses_language_model::is_text_delta(
            "response.output_text.delta"
        ),
        true
    );
    let body = "data: {\"type\":\"response.output_text.delta\",\"item_id\":\"x\",\"delta\":\"hi\"}\n\ndata: [DONE]\n";
    let chunks =
        core::github_copilot::responses::openai_responses_language_model::parse_responses_sse(body);
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].delta.as_deref().unwrap(), "hi");
}
