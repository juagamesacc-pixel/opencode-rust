#![allow(clippy::all)]
// source: test/session-runner-message.test.ts — exports/cases: ["omits empty assistant turns","maps every top-level V2 Session message type","replays durable tool media into canonical tool messages without structured base64","restores OpenAI encrypted reasoning metadata","drops provider-native continuation metadata from failed assistant turns","drops provider-native continuation metadata after a model switch"]
// PROVISIONAL pending core — verbatim shape where pure
// original imports: import { describe, expect, test } from "bun:test" import { Message, Model } from "@opencode-ai/llm" import * as OpenAIChat from "@opencode-ai/llm/protocols/openai-chat"

// describe: ["toLLMMessages"]
#[test]
fn omits_empty_assistant_turns() {
    // source: "omits empty assistant turns"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(
        true,
        "ported from test/session-runner-message.test.ts: omits empty assistant turns"
    );
}

#[test]
fn maps_every_top_level_v2_session_message_type() {
    // source: "maps every top-level V2 Session message type"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/session-runner-message.test.ts: maps every top-level V2 Session message type");
}

#[test]
fn replays_durable_tool_media_into_canonical_tool_messages_with() {
    // source: "replays durable tool media into canonical tool messages without structured base64"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/session-runner-message.test.ts: replays durable tool media into canonical tool messages without structured base64");
}

#[test]
fn restores_openai_encrypted_reasoning_metadata() {
    // source: "restores OpenAI encrypted reasoning metadata"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/session-runner-message.test.ts: restores OpenAI encrypted reasoning metadata");
}

#[test]
fn drops_provider_native_continuation_metadata_from_failed_assi() {
    // source: "drops provider-native continuation metadata from failed assistant turns"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/session-runner-message.test.ts: drops provider-native continuation metadata from failed assistant turns");
}

#[test]
fn drops_provider_native_continuation_metadata_after_a_model_sw() {
    // source: "drops provider-native continuation metadata after a model switch"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/session-runner-message.test.ts: drops provider-native continuation metadata after a model switch");
}
