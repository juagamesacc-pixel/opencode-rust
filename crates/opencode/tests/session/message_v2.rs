// source: test/session/message-v2.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { describe, expect, test } from "bun:test"; import { SessionV1 } from "@opencode-ai/core/v1/session"; import { AP

#[test]
fn session_message_v2_to_model_message() {
    // source: "session.message-v2.toModelMessage"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn filters_out_messages_with_no_parts() {
    // source: "filters out messages with no parts"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn filters_out_messages_with_only_ignored_parts() {
    // source: "filters out messages with only ignored parts"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn filters_out_user_messages_with_only_empty_text_parts() {
    // source: "filters out user messages with only empty text parts"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn filters_empty_user_text_parts_while_keeping_non_empty_parts() {
    // source: "filters empty user text parts while keeping non-empty parts"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn includes_synthetic_text_parts() {
    // source: "includes synthetic text parts"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn converts_user_text_file_parts_and_injects_compaction_subtask() {
    // source: "converts user text/file parts and injects compaction/subtask prompts"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn converts_assistant_tool_completion_into_tool_call_tool_resul() {
    // source: "converts assistant tool completion into tool-call + tool-result messages with attachments"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn preserves_jpeg_tool_result_media_for_anthropic_models() {
    // source: "preserves jpeg tool-result media for anthropic models"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn moves_bedrock_pdf_tool_result_media_into_a_separate_user_mes() {
    // source: "moves bedrock pdf tool-result media into a separate user message"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn omits_provider_metadata_when_assistant_model_differs() {
    // source: "omits provider metadata when assistant model differs"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn replaces_compacted_tool_output_with_placeholder() {
    // source: "replaces compacted tool output with placeholder"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn truncates_tool_output_when_requested() {
    // source: "truncates tool output when requested"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn converts_assistant_tool_error_into_error_text_tool_result() {
    // source: "converts assistant tool error into error-text tool result"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn forwards_partial_bash_output_for_aborted_tool_calls() {
    // source: "forwards partial bash output for aborted tool calls"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn filters_assistant_messages_with_non_abort_errors() {
    // source: "filters assistant messages with non-abort errors"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn includes_aborted_assistant_messages_only_when_they_have_non_() {
    // source: "includes aborted assistant messages only when they have non-step-start/reasoning content"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn preserves_open_router_reasoning_details_through_provider_tra() {
    // source: "preserves OpenRouter reasoning details through provider transform"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn splits_assistant_messages_on_step_start_boundaries() {
    // source: "splits assistant messages on step-start boundaries"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn drops_messages_that_only_contain_step_start_parts() {
    // source: "drops messages that only contain step-start parts"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn converts_pending_running_tool_calls_to_error_results_to_prev() {
    // source: "converts pending/running tool calls to error results to prevent dangling tool_use"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn substitutes_space_for_empty_text_between_signed_reasoning_bl() {
    // source: "substitutes space for empty text between signed reasoning blocks"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn leaves_empty_text_alone_when_reasoning_signature_is_under() {
    // source: "leaves empty text alone when reasoning signature is under "
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn leaves_empty_text_alone_when_reasoning_has_no_anthropic_sign() {
    // source: "leaves empty text alone when reasoning has no Anthropic signature"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn leaves_empty_text_alone_in_assistant_messages_without_reason() {
    // source: "leaves empty text alone in assistant messages without reasoning"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn session_message_v2_from_error() {
    // source: "session.message-v2.fromError"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn serializes_context_length_exceeded_as_context_overflow_error() {
    // source: "serializes context_length_exceeded as ContextOverflowError"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn serializes_response_error_codes() {
    // source: "serializes response error codes"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn serializes_open_ai_response_server_error_stream_chunks_as_re() {
    // source: "serializes OpenAI response server_error stream chunks as retryable APIError"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn detects_context_overflow_from_apicall_error_provider_message() {
    // source: "detects context overflow from APICallError provider messages"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn detects_context_overflow_from_context_length_exceeded_code_i() {
    // source: "detects context overflow from context_length_exceeded code in response body"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_classify_429_no_body_as_context_overflow() {
    // source: "does not classify 429 no body as context overflow"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn serializes_unknown_inputs() {
    // source: "serializes unknown inputs"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn serializes_tagged_errors_with_their_message() {
    // source: "serializes tagged errors with their message"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn classifies_zlib_error_from_fetch_as_retryable_apierror() {
    // source: "classifies ZlibError from fetch as retryable APIError"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn classifies_zlib_error_as_aborted_error_when_abort_context_is() {
    // source: "classifies ZlibError as AbortedError when abort context is provided"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn session_message_v2_latest() {
    // source: "session.message-v2.latest"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn selects_latest_messages_by_creation_time_when_ids_are_nonmon() {
    // source: "selects latest messages by creation time when IDs are nonmonotonic"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn uses_id_as_a_deterministic_tie_breaker_for_equal_creation_ti() {
    // source: "uses ID as a deterministic tie-breaker for equal creation times"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn finished_is_the_chronologically_latest_finished_assistant_no() {
    // source: "finished is the chronologically-latest finished assistant, not the array-latest"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn a_fresh_compaction_user_newer_than_the_latest_summary_surfac() {
    // source: "a fresh compaction-user newer than the latest summary surfaces in tasks"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn selects_compaction_and_subtask_work_after_the_finished_bound() {
    // source: "selects compaction and subtask work after the finished boundary by creation time"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
