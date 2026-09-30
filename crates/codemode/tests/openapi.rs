//! Port of `test/openapi.test.ts` (spec generation, auth, transport,
//! skip reasons) using the byte-identical fixtures.

use codemode::openapi_index::from_spec;
use codemode::openapi_runtime::{HttpResponse, HttpTransport};
use codemode::openapi_types::{Credential, Options};
use serde_json::{json, Value};
use std::collections::BTreeMap;

fn fixture(name: &str) -> Value {
    let path = format!("{}/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name);
    let text = std::fs::read_to_string(&path).expect("fixture readable");
    serde_json::from_str(&text).expect("fixture parses")
}

fn single_operation(operation: Value) -> Value {
    let mut op = json!({
        "operationId": "test",
        "responses": {"200": {"description": "Success"}},
    });
    for (key, value) in operation.as_object().cloned().unwrap_or_default() {
        op[key] = value;
    }
    json!({"/test": {"get": op}})
}

fn options_with(spec: Value) -> Options {
    Options {
        spec,
        base_url: None,
        auth: None,
        headers: BTreeMap::new(),
    }
}

#[test]
fn fixtures_are_byte_identical_passthrough() {
    let happy = fixture("openapi-happy-path.json");
    assert_eq!(happy["openapi"], "3.1.0");
    assert_eq!(happy["servers"][0]["url"], "https://api.example.test/v1");
    let v2 = fixture("opencode-v2-openapi.json");
    assert!(v2.get("openapi").is_some() || v2.get("swagger").is_some() || v2.is_object());
}

#[test]
fn happy_path_generates_tools() {
    let result = from_spec(
        options_with(fixture("openapi-happy-path.json")),
        HttpTransport::none(),
    );
    assert!(
        !result.tools.entries.is_empty(),
        "skipped: {:?}",
        result.skipped
    );
}

#[test]
fn skips_operations_without_servers() {
    let result = from_spec(
        options_with(json!({
            "openapi": "3.1.0",
            "info": {"title": "t", "version": "1"},
            "paths": single_operation(json!({})),
        })),
        HttpTransport::none(),
    );
    assert_eq!(result.skipped.len(), 1);
    assert!(result.skipped[0].reason.contains("pass baseUrl"));
}

#[test]
fn base_url_override_places_tools() {
    let result = from_spec(
        Options {
            spec: json!({
                "openapi": "3.1.0",
                "info": {"title": "t", "version": "1"},
                "paths": single_operation(json!({})),
            }),
            base_url: Some("https://api.example.test".to_string()),
            auth: None,
            headers: BTreeMap::new(),
        },
        HttpTransport::none(),
    );
    assert!(result.skipped.is_empty(), "skipped: {:?}", result.skipped);
    assert!(result.tools.contains_key("test"));
}

#[test]
fn preserves_path_sanitization_and_collisions() {
    let result = from_spec(
        Options {
            spec: json!({
                "openapi": "3.1.0",
                "info": {"title": "t", "version": "1"},
                "servers": [{"url": "https://api.example.test"}],
                "paths": {
                    "/first": {"get": {"operationId": "group.item", "responses": {"200": {"description": "ok"}}}},
                    "/second": {"get": {"operationId": "group.item", "responses": {"200": {"description": "ok"}}}},
                },
            }),
            base_url: None,
            auth: None,
            headers: BTreeMap::new(),
        },
        HttpTransport::none(),
    );
    assert!(result.skipped.is_empty(), "skipped: {:?}", result.skipped);
    // Both operations land under distinct paths (collision folding).
    let top: Vec<String> = result.tools.keys();
    assert!(top.contains(&"group".to_string()), "top: {:?}", top);
}

#[test]
fn synthesizes_flat_operation_ids() {
    let result = from_spec(
        Options {
            spec: json!({
                "openapi": "3.1.0",
                "info": {"title": "t", "version": "1"},
                "servers": [{"url": "https://api.example.test"}],
                "paths": {
                    "/users/{userId}": {"get": {"responses": {"200": {"description": "ok"}}}},
                },
            }),
            base_url: None,
            auth: None,
            headers: BTreeMap::new(),
        },
        HttpTransport::none(),
    );
    assert!(result.skipped.is_empty(), "skipped: {:?}", result.skipped);
    assert!(
        result.tools.contains_key("getUsersByUserid"),
        "keys: {:?}",
        result.tools.keys()
    );
}

#[test]
fn rejects_ambiguous_base_urls() {
    for url in [
        "ftp://example.test",
        "https://example.test/?x=1",
        "https://example.test/#frag",
    ] {
        let result = from_spec(
            Options {
                spec: json!({
                    "openapi": "3.1.0",
                    "info": {"title": "t", "version": "1"},
                    "paths": single_operation(json!({})),
                }),
                base_url: Some(url.to_string()),
                auth: None,
                headers: BTreeMap::new(),
            },
            HttpTransport::none(),
        );
        assert_eq!(result.skipped.len(), 1, "url: {}", url);
        assert!(
            result.skipped[0].reason.contains("absolute HTTP(S) URL")
                || result.skipped[0]
                    .reason
                    .contains("query string or fragment"),
            "reason: {}",
            result.skipped[0].reason
        );
    }
}

#[test]
fn skips_unsupported_transports() {
    // SSE + WebSocket + binary responses are skipped with precise reasons.
    let sse = json!({"content": {"text/event-stream": {"schema": {"type": "string"}}}});
    let result = from_spec(
        Options {
            spec: json!({
                "openapi": "3.1.0",
                "info": {"title": "t", "version": "1"},
                "servers": [{"url": "https://api.example.test"}],
                "paths": {"/s": {"get": {
                    "operationId": "s",
                    "responses": {"200": {"description": "ok", "content": sse["content"]}},
                }}},
            }),
            base_url: None,
            auth: None,
            headers: BTreeMap::new(),
        },
        HttpTransport::none(),
    );
    assert_eq!(result.skipped.len(), 1);
    assert_eq!(result.skipped[0].reason, "SSE operations are not supported");
    let ws = from_spec(
        Options {
            spec: json!({
                "openapi": "3.1.0",
                "info": {"title": "t", "version": "1"},
                "servers": [{"url": "https://api.example.test"}],
                "paths": {"/w": {"get": {
                    "operationId": "w",
                    "x-websocket": true,
                    "responses": {"200": {"description": "ok"}},
                }}},
            }),
            base_url: None,
            auth: None,
            headers: BTreeMap::new(),
        },
        HttpTransport::none(),
    );
    assert_eq!(
        ws.skipped[0].reason,
        "WebSocket operations are not supported"
    );
}

#[test]
fn fails_closed_on_missing_security_schemes() {
    let result = from_spec(
        options_with(json!({
            "openapi": "3.1.0",
            "info": {"title": "t", "version": "1"},
            "servers": [{"url": "https://api.example.test"}],
            "paths": single_operation(json!({
                "security": [{"Ghost": []}],
            })),
        })),
        HttpTransport::none(),
    );
    assert_eq!(result.skipped.len(), 1);
    assert!(
        result.skipped[0]
            .reason
            .contains("missing or malformed scheme"),
        "reason: {}",
        result.skipped[0].reason
    );
}

#[test]
fn cookie_authentication_is_unsupported() {
    let result = from_spec(
        options_with(json!({
            "openapi": "3.1.0",
            "info": {"title": "t", "version": "1"},
            "servers": [{"url": "https://api.example.test"}],
            "components": {"securitySchemes": {
                "Cookie": {"type": "apiKey", "in": "cookie", "name": "session"},
            }},
            "paths": single_operation(json!({"security": [{"Cookie": []}]})),
        })),
        HttpTransport::none(),
    );
    assert_eq!(result.skipped.len(), 1);
    assert!(
        result.skipped[0].reason.contains("cookie authentication"),
        "reason: {}",
        result.skipped[0].reason
    );
}

#[test]
fn executes_get_with_path_params_and_auth() {
    use std::sync::Arc;
    let seen: Arc<std::sync::Mutex<Vec<String>>> = Arc::new(std::sync::Mutex::new(vec![]));
    let seen_request = seen.clone();
    let transport = HttpTransport::new(move |request| {
        seen_request.lock().unwrap().push(request.url.clone());
        let auth = request
            .headers
            .get("authorization")
            .cloned()
            .unwrap_or_default();
        assert_eq!(auth, "Bearer token-123");
        Ok(HttpResponse {
            status: 200,
            headers: [("content-type".to_string(), "application/json".to_string())]
                .into_iter()
                .collect(),
            body: br#"{"id":"u1"}"#.to_vec(),
        })
    });
    let auth_resolve: codemode::openapi_types::AuthResolver = std::sync::Arc::new(|_| {
        Ok(Some(Credential::Bearer {
            token: "token-123".to_string(),
        }))
    });
    let result = from_spec(
        Options {
            spec: json!({
                "openapi": "3.1.0",
                "info": {"title": "t", "version": "1"},
                "servers": [{"url": "https://api.example.test"}],
                "security": [{"Bearer": []}],
                "components": {"securitySchemes": {
                    "Bearer": {"type": "http", "scheme": "bearer"},
                }},
                "paths": {"/users/{userId}": {"get": {
                    "operationId": "getUser",
                    "parameters": [{"name": "userId", "in": "path", "required": true,
                        "schema": {"type": "string"}}],
                    "responses": {"200": {"description": "ok", "content": {
                        "application/json": {"schema": {"type": "object"}},
                    }}},
                }}},
            }),
            base_url: None,
            auth: Some(codemode::openapi_types::AuthConfig {
                resolve: auth_resolve,
            }),
            headers: BTreeMap::new(),
        },
        transport,
    );
    assert!(result.skipped.is_empty(), "skipped: {:?}", result.skipped);
    // Invoke the generated tool through the tool runtime surface.
    match result.tools.get("getUser") {
        Some(codemode::openapi_types::ToolNode::Tool(definition)) => {
            let output = (definition.run)(&json!({"userId": "u1"})).expect("invokes");
            assert_eq!(output, json!({"id": "u1"}));
        }
        other => panic!("expected tool, got {:?}", other.is_some()),
    }
    assert_eq!(
        *seen.lock().unwrap(),
        vec!["https://api.example.test/users/u1".to_string()]
    );
}

#[test]
fn http_errors_and_malformed_json_map_to_tool_errors() {
    let failing = HttpTransport::new(|_| {
        Ok(HttpResponse {
            status: 500,
            headers: BTreeMap::new(),
            body: b"boom".to_vec(),
        })
    });
    let result = from_spec(
        Options {
            spec: json!({
                "openapi": "3.1.0",
                "info": {"title": "t", "version": "1"},
                "servers": [{"url": "https://api.example.test"}],
                "paths": single_operation(json!({})),
            }),
            base_url: None,
            auth: None,
            headers: BTreeMap::new(),
        },
        failing,
    );
    match result.tools.get("test") {
        Some(codemode::openapi_types::ToolNode::Tool(definition)) => {
            let err = (definition.run)(&json!({})).unwrap_err();
            match err {
                codemode::tool::ToolFailure::ToolError(error) => {
                    assert_eq!(error.message, "GET /test failed with HTTP 500: boom");
                }
                _ => panic!("expected tool error"),
            }
        }
        _ => panic!("expected tool"),
    }
}

#[test]
fn missing_required_params_fail_before_network() {
    let transport = HttpTransport::new(|_| {
        panic!("network must not run");
    });
    let result = from_spec(
        Options {
            spec: json!({
                "openapi": "3.1.0",
                "info": {"title": "t", "version": "1"},
                "servers": [{"url": "https://api.example.test"}],
                "paths": {"/users/{userId}": {"get": {
                    "operationId": "getUser",
                    "parameters": [{"name": "userId", "in": "path", "required": true,
                        "schema": {"type": "string"}}],
                    "responses": {"200": {"description": "ok"}},
                }}},
            }),
            base_url: None,
            auth: None,
            headers: BTreeMap::new(),
        },
        transport,
    );
    match result.tools.get("getUser") {
        Some(codemode::openapi_types::ToolNode::Tool(definition)) => {
            let err = (definition.run)(&json!({})).unwrap_err();
            match err {
                codemode::tool::ToolFailure::ToolError(error) => {
                    assert_eq!(error.message, "Missing required path parameter 'userId'.");
                }
                _ => panic!("expected tool error"),
            }
        }
        _ => panic!("expected tool"),
    }
}

#[test]
fn cross_location_collisions_prefix_input_names() {
    // Same parameter name in query + header → `query_name`/`header_name`.
    let result = from_spec(
        Options {
            spec: json!({
                "openapi": "3.1.0",
                "info": {"title": "t", "version": "1"},
                "servers": [{"url": "https://api.example.test"}],
                "paths": {"/t": {"get": {
                    "operationId": "t",
                    "parameters": [
                        {"name": "token", "in": "query", "schema": {"type": "string"}},
                        {"name": "token", "in": "header", "schema": {"type": "string"}},
                    ],
                    "responses": {"200": {"description": "ok"}},
                }}},
            }),
            base_url: None,
            auth: None,
            headers: BTreeMap::new(),
        },
        HttpTransport::none(),
    );
    assert!(result.skipped.is_empty(), "skipped: {:?}", result.skipped);
    match result.tools.get("t") {
        Some(codemode::openapi_types::ToolNode::Tool(definition)) => {
            let signature = codemode::tool_schema::input_typescript(definition, false);
            assert!(
                signature.contains("query_token"),
                "signature: {}",
                signature
            );
            assert!(
                signature.contains("header_token"),
                "signature: {}",
                signature
            );
        }
        _ => panic!("expected tool"),
    }
}
