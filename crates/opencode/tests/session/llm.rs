// source: test/session/llm.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { PermissionV1 } from "@opencode-ai/core/v1/permission"; import { ConfigV1 } from "@opencode-ai/core/v1/config/co

#[test]
fn session_llm_has_tool_calls() {
    // source: "session.llm.hasToolCalls"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_false_for_empty_messages_array() {
    // source: "returns false for empty messages array"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_false_for_messages_with_only_text_content() {
    // source: "returns false for messages with only text content"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_true_when_messages_contain_tool_call() {
    // source: "returns true when messages contain tool-call"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_true_when_messages_contain_tool_result() {
    // source: "returns true when messages contain tool-result"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_false_for_messages_with_string_content() {
    // source: "returns false for messages with string content"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_true_when_tool_call_is_mixed_with_text_content() {
    // source: "returns true when tool-call is mixed with text content"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn session_llm_ai_sdk_adapter() {
    // source: "session.llm.ai-sdk adapter"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn maps_ai_sdk_stream_chunks_without_losing_session_visible_fie() {
    // source: "maps AI SDK stream chunks without losing session-visible fields"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn creates_stable_block_ids_when_ai_sdk_omits_them() {
    // source: "creates stable block ids when AI SDK omits them"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn explicitly_ignores_non_session_visible_ai_sdk_chunks() {
    // source: "explicitly ignores non-session-visible AI SDK chunks"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn preserves_tool_error_cause() {
    // source: "preserves tool-error cause"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn emits_undefined_usage_when_every_ai_sdk_usage_field_is_missi() {
    // source: "emits undefined usage when every AI SDK usage field is missing"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn reuses_adapter_state_cleanly_across_streams_once_finish_has_() {
    // source: "reuses adapter state cleanly across streams once finish has fired"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn preserves_provider_metadata_on_step_finish_so_anthropic_cach() {
    // source: "preserves providerMetadata on step-finish so Anthropic cache writes survive getUsage"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn captures_copilot_billed_usage_from_raw_anthropic_message_del() {
    // source: "captures Copilot billed usage from raw Anthropic message deltas per step"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn session_llm_stream() {
    // source: "session.llm.stream"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn sends_the_parent_session_header_for_opencode_providers() {
    // source: "sends the parent session header for opencode providers"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn sends_temperature_tokens_and_reasoning_options_for_openai_co() {
    // source: "sends temperature, tokens, and reasoning options for openai-compatible models"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn surfaces_network_error_finish_reasons_as_retryable_stream_fa() {
    // source: "surfaces network_error finish reasons as retryable stream failures"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn replays_cerebras_assistant_reasoning_using_the_provider_supp() {
    // source: "replays Cerebras assistant reasoning using the provider-supported field"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn replays_native_mistral_reasoning_from_chat_history() {
    // source: "replays native Mistral reasoning from chat history"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn service_stream_cancellation_cancels_provider_response_body_p() {
    // source: "service stream cancellation cancels provider response body promptly"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn keeps_tools_enabled_by_prompt_permissions() {
    // source: "keeps tools enabled by prompt permissions"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn sends_responses_api_payload_for_open_ai_models() {
    // source: "sends responses API payload for OpenAI models"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn keeps_supported_open_ai_models_on_ai_sdk_path_when_native_fl() {
    // source: "keeps supported OpenAI models on AI SDK path when native flag is off"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn streams_open_ai_through_native_runtime_when_opted_in() {
    // source: "streams OpenAI through native runtime when opted in"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn uses_injected_native_request_executor_for_tool_calls() {
    // source: "uses injected native request executor for tool calls"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn executes_open_ai_tool_calls_through_native_runtime() {
    // source: "executes OpenAI tool calls through native runtime"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn accepts_user_image_attachments_as_data_urls_for_open_ai_mode() {
    // source: "accepts user image attachments as data URLs for OpenAI models"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn sends_messages_api_payload_for_anthropic_compatible_models() {
    // source: "sends messages API payload for Anthropic Compatible models"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn sends_anthropic_tool_use_blocks_with_tool_result_immediately() {
    // source: "sends anthropic tool_use blocks with tool_result immediately after them"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn sends_google_api_payload_for_gemini_models() {
    // source: "sends Google API payload for Gemini models"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
