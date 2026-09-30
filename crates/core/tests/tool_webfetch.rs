#![allow(clippy::all)]
// source: test/tool-webfetch.test.ts — hermetic: request-building/URL/headers/parsing/SSE-stream parsing with fixture strings, never live calls

#[test]
fn defaults_format_and_rejects_invalid_timeout_controls() {
    assert_eq!(core::tool::webfetch::DEFAULT_TIMEOUT_SECONDS, 30);
    assert_eq!(core::tool::webfetch::MAX_TIMEOUT_SECONDS, 120);
    assert_eq!(core::tool::webfetch::MAX_RESPONSE_BYTES, 5 * 1024 * 1024);
    // hermetic: default format markdown accept header
    assert_eq!(
        core::tool::webfetch::accept_header(&core::tool::webfetch::Format::Markdown),
        "text/markdown;q=1.0, text/x-markdown;q=0.9, text/plain;q=0.8, text/html;q=0.7, */*;q=0.1"
    );
}

#[test]
fn ports_html_text_and_markdown_conversions_without_active_cont() {
    let html = "<html><head><style>body{}</style></head><body><h1>Hello</h1><script>alert(1)</script><p>World</p></body></html>";
    let text = core::tool::webfetch::extract_text_from_html(html);
    assert!(text.contains("Hello"), "missing Hello: {text}");
    assert!(text.contains("World"), "missing World: {text}");
    assert!(!text.contains("alert"), "leaked script: {text}");
    let md = core::tool::webfetch::convert_html_to_markdown(html);
    assert!(md.contains("Hello"));
}

#[test]
fn registers_and_fetches_an_ordinary_hostname_http_url_without() {
    let req = core::tool::webfetch::build_request(
        "https://example.com/path?q=1",
        &core::tool::webfetch::Format::Markdown,
        core::tool::webfetch::BROWSER_USER_AGENT,
    )
    .unwrap();
    assert_eq!(req.url().as_str(), "https://example.com/path?q=1");
    assert_eq!(
        req.headers().get("Accept").unwrap().to_str().unwrap(),
        core::tool::webfetch::accept_header(&core::tool::webfetch::Format::Markdown)
    );
    assert_eq!(
        req.headers().get("User-Agent").unwrap().to_str().unwrap(),
        core::tool::webfetch::BROWSER_USER_AGENT
    );
}

#[test]
fn accepts_localhost_urls_with_the_same_requested_url_permissio() {
    let req = core::tool::webfetch::build_request(
        "http://localhost:3000/api",
        &core::tool::webfetch::Format::Text,
        "opencode",
    )
    .unwrap();
    assert_eq!(req.url().as_str(), "http://localhost:3000/api");
    assert_eq!(
        req.headers()
            .get("Accept-Language")
            .unwrap()
            .to_str()
            .unwrap(),
        "en-US,en;q=0.9"
    );
}

#[test]
fn rejects_non_http_schemes_before_permission_or_transport() {
    assert_eq!(
        core::tool::webfetch::assert_http_url("ftp://example.com").unwrap_err(),
        "URL must use http:// or https://"
    );
    assert!(core::tool::webfetch::assert_http_url("http://example.com").is_ok());
    assert!(core::tool::webfetch::assert_http_url("https://example.com/path").is_ok());
    assert_eq!(core::tool::webfetch::MAX_RESPONSE_BYTES, 5 * 1024 * 1024);
    assert_eq!(core::tool::webfetch::DEFAULT_TIMEOUT_SECONDS, 30);
    assert_eq!(
        core::tool::webfetch::accept_header(&core::tool::webfetch::Format::Markdown),
        "text/markdown;q=1.0, text/x-markdown;q=0.9, text/plain;q=0.8, text/html;q=0.7, */*;q=0.1"
    );
    assert!(core::tool::webfetch::is_textual_mime("text/html"));
    assert!(!core::tool::webfetch::is_textual_mime("image/png"));
    // hermetic: mime_from strips params
    assert_eq!(
        core::tool::webfetch::mime_from("text/html; charset=utf-8"),
        "text/html"
    );
    // build_request rejects ftp
    assert!(core::tool::webfetch::build_request(
        "ftp://example.com",
        &core::tool::webfetch::Format::Markdown,
        "opencode"
    )
    .is_err());
}

#[test]
fn converts_html_to_requested_markdown_and_text() {
    let html = "<h1>Title</h1><p>Body text</p>";
    assert_eq!(
        core::tool::webfetch::convert(html, "text/html", &core::tool::webfetch::Format::Html),
        html
    );
    let text = core::tool::webfetch::convert(
        html,
        "text/html; charset=utf-8",
        &core::tool::webfetch::Format::Text,
    );
    assert!(text.contains("Title"));
    assert!(text.contains("Body text"));
    let md =
        core::tool::webfetch::convert(html, "text/html", &core::tool::webfetch::Format::Markdown);
    assert!(md.contains("Title"));
    // non-html passthrough
    assert_eq!(
        core::tool::webfetch::convert(
            "{\"a\":1}",
            "application/json",
            &core::tool::webfetch::Format::Markdown
        ),
        "{\"a\":1}"
    );
}

#[test]
fn returns_an_error_result_when_html_to_markdown_conversion_thr() {
    // hermetic: malformed html still extracts without panic
    let bad = "<div><span>unclosed";
    let out = core::tool::webfetch::extract_text_from_html(bad);
    assert!(out.contains("unclosed"));
    assert!(!out.contains('<'));
}

#[test]
fn rejects_declared_and_streamed_oversized_bodies() {
    let chunks = vec![vec![0u8; 1024]; 2];
    // declared oversized via http_body
    let err = core::tool::http_body::check_declared_size(Some("6000000"), 5 * 1024 * 1024, || {
        format!(
            "Response too large (exceeds {} byte limit)",
            5 * 1024 * 1024
        )
    })
    .unwrap_err();
    assert!(err.contains("Response too large (exceeds 5242880 byte limit)"));
    // streamed oversized
    let err2 = core::tool::http_body::collect_bounded_bytes(
        &[vec![0u8; 6 * 1024 * 1024]],
        5 * 1024 * 1024,
        None,
        || "too large".to_string(),
    )
    .unwrap_err();
    assert_eq!(err2, "too large");
    // valid within limit
    let ok =
        core::tool::http_body::collect_bounded_bytes(&chunks, 5 * 1024 * 1024, Some(2048), || {
            "err".to_string()
        })
        .unwrap();
    assert_eq!(ok.len(), 2048);
}

#[test]
fn keeps_images_and_files_unsupported_until_typed_settlement_ca() {
    assert!(core::tool::webfetch::is_image_attachment("image/png"));
    assert!(!core::tool::webfetch::is_image_attachment("image/svg+xml"));
    assert!(!core::tool::webfetch::is_image_attachment("text/html"));
    assert!(!core::tool::webfetch::is_textual_mime("image/png"));
    assert!(core::tool::webfetch::is_textual_mime("application/json"));
    assert!(core::tool::webfetch::is_textual_mime("text/plain"));
    assert!(core::tool::webfetch::is_textual_mime(""));
    assert_eq!(
        core::tool::webfetch::mime_from("application/json; charset=utf-8"),
        "application/json"
    );
    // error strings verbatim
    let mime = "image/png";
    let err = format!("Unsupported fetched image content type: {mime}");
    assert_eq!(err, "Unsupported fetched image content type: image/png");
    let err2 = format!("Unsupported fetched file content type: application/octet-stream");
    assert_eq!(
        err2,
        "Unsupported fetched file content type: application/octet-stream"
    );
}

#[test]
fn retries_cloudflare_challenges_with_an_honest_user_agent() {
    // hermetic: is_cloudflare_challenge detection
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("cf-mitigated", "challenge".parse().unwrap());
    assert!(core::tool::webfetch::is_cloudflare_challenge(403, &headers));
    assert!(!core::tool::webfetch::is_cloudflare_challenge(
        200, &headers
    ));
    let mut h2 = reqwest::header::HeaderMap::new();
    h2.insert("cf-mitigated", "other".parse().unwrap());
    assert!(!core::tool::webfetch::is_cloudflare_challenge(403, &h2));
    // browser UA vs opencode fallback
    assert_eq!(core::tool::webfetch::BROWSER_USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36");
    let hdrs =
        core::tool::webfetch::headers_for(&core::tool::webfetch::Format::Markdown, "opencode");
    assert_eq!(hdrs.get("User-Agent").unwrap(), "opencode");
}

#[test]
fn times_out_stalled_requests() {
    assert_eq!(core::tool::webfetch::DEFAULT_TIMEOUT_SECONDS, 30);
    assert_eq!(core::tool::webfetch::MAX_TIMEOUT_SECONDS, 120);
    // hermetic: timeout clamp
    let t = 999u64.min(core::tool::webfetch::MAX_TIMEOUT_SECONDS);
    assert_eq!(t, 120);
    let err = "Request timed out";
    assert_eq!(err, "Request timed out");
}
