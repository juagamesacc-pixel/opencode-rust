// source: test/acp/content.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { describe, expect, test } from "bun:test"; import type { ContentBlock } from "@agentclientprotocol/sdk"; import

#[test]
fn acp_content_conversion() {
    // source: "acp content conversion"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn plain_text_block_becomes_a_text_part() {
    // source: "plain text block becomes a text part"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn assistant_only_text_audience_becomes_synthetic() {
    // source: "assistant-only text audience becomes synthetic"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn user_only_text_audience_becomes_ignored() {
    // source: "user-only text audience becomes ignored"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn image_block_with_base64_data_becomes_a_data_url_file_part() {
    // source: "image block with base64 data becomes a data URL file part"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn image_block_with_http_uri_becomes_a_file_part() {
    // source: "image block with http URI becomes a file part"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resource_link_file_url_becomes_a_file_part_with_name_and_fal() {
    // source: "resource_link file URL becomes a file part with name and fallback mime"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resource_link_zed_path_becomes_a_file_url_part() {
    // source: "resource_link zed path becomes a file URL part"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resource_with_text_becomes_a_sourced_text_part() {
    // source: "resource with text becomes a sourced text part"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resource_with_text_uses_uri_fallback_for_non_file_resources() {
    // source: "resource with text uses URI fallback for non-file resources"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resource_with_text_includes_file_path() {
    // source: "resource with text includes file path"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resource_with_blob_and_mime_type_becomes_a_data_url_file_par() {
    // source: "resource with blob and mimeType becomes a data URL file part"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn data_url_resource_is_preserved_as_a_file_part() {
    // source: "data URL resource is preserved as a file part"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn unsupported_blocks_are_ignored() {
    // source: "unsupported blocks are ignored"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn acp_replay_conversion() {
    // source: "acp replay conversion"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn replays_text_audience_annotations() {
    // source: "replays text audience annotations"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn replays_file_and_data_url_parts_as_acp_content() {
    // source: "replays file and data URL parts as ACP content"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
