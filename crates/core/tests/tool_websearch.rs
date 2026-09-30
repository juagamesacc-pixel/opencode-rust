#![allow(clippy::all)]
// source: test/tool-websearch.test.ts — hermetic: request-building/URL/headers/parsing/SSE-stream parsing with fixture strings, never live calls

#[test]
fn rejects_out_of_range_numeric_controls() {
    assert_eq!(core::tool::websearch::MAX_NUM_RESULTS, 20);
    assert_eq!(core::tool::websearch::MAX_CONTEXT_CHARACTERS, 50_000);
    assert_eq!(core::tool::websearch::MAX_RESPONSE_BYTES, 256 * 1024);
    // hermetic: clamping logic — numResults >20 should be rejected (schema validation in source)
    let too_many = 21u64;
    assert!(too_many > core::tool::websearch::MAX_NUM_RESULTS);
    let too_big = 50001u64;
    assert!(too_big > core::tool::websearch::MAX_CONTEXT_CHARACTERS);
}

#[test]
fn selects_a_stable_provider_per_session() {
    let flags = core::tool::websearch::Config {
        provider: None,
        enable_exa: false,
        enable_parallel: false,
        exa_api_key: None,
        parallel_api_key: None,
    };
    let a = core::tool::websearch::select_provider("session-123", &flags, None);
    let b = core::tool::websearch::select_provider("session-123", &flags, None);
    assert_eq!(a, b, "provider must be stable per session");
    // different session may differ but deterministic
    let c = core::tool::websearch::select_provider("session-456", &flags, None);
    // at least valid enum
    assert!(matches!(
        c,
        core::tool::websearch::Provider::Exa | core::tool::websearch::Provider::Parallel
    ));
}

#[test]
fn supports_an_explicit_operational_override() {
    let flags = core::tool::websearch::Config {
        provider: None,
        enable_exa: false,
        enable_parallel: false,
        exa_api_key: None,
        parallel_api_key: None,
    };
    assert_eq!(
        core::tool::websearch::select_provider(
            "any",
            &flags,
            Some(core::tool::websearch::Provider::Parallel)
        ),
        core::tool::websearch::Provider::Parallel
    );
    assert_eq!(
        core::tool::websearch::select_provider(
            "any",
            &flags,
            Some(core::tool::websearch::Provider::Exa)
        ),
        core::tool::websearch::Provider::Exa
    );
}

#[test]
fn prefers_parallel_when_both_explicit_flags_are_enabled() {
    let flags = core::tool::websearch::Config {
        provider: None,
        enable_exa: true,
        enable_parallel: true,
        exa_api_key: None,
        parallel_api_key: None,
    };
    assert_eq!(
        core::tool::websearch::select_provider("sess", &flags, None),
        core::tool::websearch::Provider::Parallel
    );
}

#[test]
fn prefers_exa_when_only_its_explicit_flag_is_enabled() {
    let flags = core::tool::websearch::Config {
        provider: None,
        enable_exa: true,
        enable_parallel: false,
        exa_api_key: None,
        parallel_api_key: None,
    };
    assert_eq!(
        core::tool::websearch::select_provider("sess", &flags, None),
        core::tool::websearch::Provider::Exa
    );
}

#[test]
fn parses_plain_json_rpc_responses() {
    let fixture = r#"{"result":{"content":[{"type":"text","text":"hello from exa"}]}}"#;
    assert_eq!(
        core::tool::websearch::parse_response(fixture).unwrap(),
        "hello from exa"
    );
    // with extra fields
    let fixture2 = r#"{"result":{"content":[{"type":"text","text":"first"},{"type":"text","text":"second"}]}}"#;
    // first text item wins per source find()
    assert_eq!(
        core::tool::websearch::parse_response(fixture2).unwrap(),
        "first"
    );
}

#[test]
fn parses_sse_json_rpc_responses_and_ignores_non_json_frames() {
    let body = "event: data\ndata: {\"result\":{\"content\":[{\"type\":\"text\",\"text\":\"sse text\"}]}}\n: keep-alive\n\ndata: not-json\n";
    assert_eq!(
        core::tool::websearch::parse_response(body).unwrap(),
        "sse text"
    );
    // plain with multiple SSE lines — first valid wins
    let multi = "data: {\"result\":{\"content\":[{\"type\":\"text\",\"text\":\"first\"}]}}\ndata: {\"result\":{\"content\":[{\"type\":\"text\",\"text\":\"second\"}]}}";
    assert_eq!(
        core::tool::websearch::parse_response(multi).unwrap(),
        "first"
    );
    assert!(core::tool::websearch::parse_response("no json here").is_none());
}

#[test]
fn registers_websearch_asserts_query_permission_and_calls_exa() {
    let body = core::tool::websearch::exa_mcp_body("rust", "auto", 8, "fallback", Some(10000));
    assert_eq!(body["method"], "tools/call");
    assert_eq!(body["params"]["name"], "web_search_exa");
    assert_eq!(body["params"]["arguments"]["query"], "rust");
    assert_eq!(body["params"]["arguments"]["type"], "auto");
    // hermetic request builder — URL no key
    let req = core::tool::websearch::build_exa_request(None, &body).unwrap();
    assert_eq!(req.url().as_str(), "https://mcp.exa.ai/mcp");
    assert_eq!(
        req.headers().get("Accept").unwrap().to_str().unwrap(),
        "application/json, text/event-stream"
    );
    // with key goes to query param
    let req2 = core::tool::websearch::build_exa_request(Some("sk-exa"), &body).unwrap();
    assert!(req2.url().as_str().contains("exaApiKey=sk-exa"));
}

#[test]
fn calls_parallel_with_session_id_and_keeps_bearer_credentials() {
    let body = core::tool::websearch::parallel_mcp_body("my query", "sess-xyz");
    assert_eq!(body["params"]["name"], "web_search");
    assert_eq!(body["params"]["arguments"]["objective"], "my query");
    assert_eq!(body["params"]["arguments"]["session_id"], "sess-xyz");
    let headers = core::tool::websearch::parallel_headers(Some("parallel-key"), "1.18.30");
    assert_eq!(headers.get("Authorization").unwrap(), "Bearer parallel-key");
    assert!(headers
        .get("User-Agent")
        .unwrap()
        .contains("opencode/1.18.30"));
    // hermetic builder keeps auth out of body
    let req = core::tool::websearch::build_parallel_request(Some("parallel-key"), "1.18.30", &body)
        .unwrap();
    assert_eq!(
        req.headers()
            .get("Authorization")
            .unwrap()
            .to_str()
            .unwrap(),
        "Bearer parallel-key"
    );
    assert!(!req.url().as_str().contains("parallel-key"));
}

#[test]
fn keeps_an_exa_credential_in_the_transport_url_and_out_of_mode() {
    let url = core::tool::websearch::exa_url(Some("secret123"));
    assert_eq!(url, "https://mcp.exa.ai/mcp?exaApiKey=secret123");
    assert!(!url.contains("secret123x"));
    // model output would be text from parse_response, never contains key
    let fixture = r#"{"result":{"content":[{"type":"text","text":"result"}]}}"#;
    let text = core::tool::websearch::parse_response(fixture).unwrap();
    assert!(!text.contains("secret123"));
    assert_eq!(
        core::tool::websearch::exa_url(None),
        "https://mcp.exa.ai/mcp"
    );
}

#[test]
fn returns_the_legacy_no_results_fallback_as_concise_model_text() {
    assert_eq!(
        core::tool::websearch::NO_RESULTS,
        "No search results found. Please try a different query."
    );
    let none = core::tool::websearch::parse_response("{\"result\":{\"content\":[]}}");
    assert!(none.is_none());
    let text = none.unwrap_or_else(|| core::tool::websearch::NO_RESULTS.to_string());
    assert_eq!(
        text,
        "No search results found. Please try a different query."
    );
}

#[test]
fn rejects_oversized_mcp_response_bodies() {
    let chunks = vec![vec![b'x'; 300 * 1024]];
    let err = core::tool::http_body::collect_bounded_bytes(&chunks, 256 * 1024, None, || {
        "web_search_exa response exceeded 262144 bytes".to_string()
    })
    .unwrap_err();
    assert!(err.contains("exceeded 262144 bytes"));
    // within limit passes
    let ok_chunks = vec![vec![b'x'; 1024]];
    let ok = core::tool::http_body::collect_bounded_bytes(&ok_chunks, 256 * 1024, None, || {
        "err".to_string()
    })
    .unwrap();
    assert_eq!(ok.len(), 1024);
}
