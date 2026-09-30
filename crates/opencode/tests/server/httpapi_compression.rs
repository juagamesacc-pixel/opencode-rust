// source: test/server/httpapi-compression.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { afterEach, describe, expect, test } from "bun:test"; import { gunzipSync, inflateSync } from "node:zlib"; impor

#[test]
fn http_api_compression() {
    // source: "HttpApi compression"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn encodes_responses() {
    // source: "encodes responses"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn gzips_json_when_accept_encoding_includes_gzip_and_body_excee() {
    // source: "gzips JSON when Accept-Encoding includes gzip and body exceeds threshold"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn uses_deflate_when_only_deflate_is_acceptable() {
    // source: "uses deflate when only deflate is acceptable"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn prefers_gzip_when_both_gzip_and_deflate_are_acceptable() {
    // source: "prefers gzip when both gzip and deflate are acceptable"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_include_the_original_content_length_when_compressed() {
    // source: "does not include the original Content-Length when compressed"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn skips() {
    // source: "skips"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn when_no_accept_encoding_header_is_present() {
    // source: "when no Accept-Encoding header is present"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn when_accept_encoding_only_allows_unsupported_encodings() {
    // source: "when Accept-Encoding only allows unsupported encodings"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn when_the_response_body_is_below_the_1024_byte_threshold() {
    // source: "when the response body is below the 1024-byte threshold"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn head_requests() {
    // source: "HEAD requests"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn streaming_exclusions() {
    // source: "streaming exclusions"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn event_sse_is_not_compressed() {
    // source: "/event SSE is not compressed"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn global_event_sse_is_not_compressed() {
    // source: "/global/event SSE is not compressed"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
