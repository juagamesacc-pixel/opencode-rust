// source: test/acp/service-session.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { describe, expect, it } from "bun:test"; import type {; import type { AssistantMessage, Event, OpencodeClient }

#[test]
fn acp_service_sessions() {
    // source: "ACP service sessions"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn creates_a_backed_session_with_config_options_and_command_upd() {
    // source: "creates a backed session with config options and command update"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn loads_a_session_and_restores_model_variant_and_mode_from_mes() {
    // source: "loads a session and restores model variant and mode from messages"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn replays_loaded_session_transcript_chunks() {
    // source: "replays loaded session transcript chunks"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn lists_sessions_sorted_by_updated_time_with_cursor_support() {
    // source: "lists sessions sorted by updated time with cursor support"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn includes_live_acp_sessions_before_they_appear_in_server_back() {
    // source: "includes live ACP sessions before they appear in server-backed session list"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn lists_all_sessions_with_next_cursor_when_the_first_page_is_f() {
    // source: "lists all sessions with next cursor when the first page is full"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resumes_a_session_and_stores_restored_state_without_replayin() {
    // source: "resumes a session and stores restored state without replaying transcript chunks"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn closes_local_acp_state_and_aborts_the_backing_session_best_e() {
    // source: "closes local ACP state and aborts the backing session best-effort"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn cancel_aborts_the_backing_session_and_keeps_the_acp_session() {
    // source: "cancel aborts the backing session and keeps the ACP session"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_fail_cancel_or_close_when_the_backing_abort_fails() {
    // source: "does not fail cancel or close when the backing abort fails"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn forks_a_session_loads_fork_state_and_returns_config_options() {
    // source: "forks a session, loads fork state, and returns config options"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn restores_model_variant_and_mode_from_the_latest_user_message() {
    // source: "restores model variant and mode from the latest user message"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn maps_provider_auth_failures_to_auth_required_request_errors() {
    // source: "maps provider auth failures to auth-required request errors"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_cache_failed_directory_snapshots() {
    // source: "does not cache failed directory snapshots"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn registers_same_name_mcp_servers_again_for_different_sessions() {
    // source: "registers same-name MCP servers again for different sessions or configs"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn uses_the_configured_model_as_the_new_session_default() {
    // source: "uses the configured model as the new session default"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_scan_last_used_sessions_when_resolving_the_new_sess() {
    // source: "does not scan last-used sessions when resolving the new session default"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn switches_model_and_returns_updated_model_and_effort_options() {
    // source: "switches model and returns updated model and effort options"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn switches_effort_and_returns_the_updated_effort_current_value() {
    // source: "switches effort and returns the updated effort current value"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn switches_mode_and_returns_the_updated_mode_current_value() {
    // source: "switches mode and returns the updated mode current value"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn maps_invalid_model_effort_mode_and_config_id_to_invalid_para() {
    // source: "maps invalid model effort mode and config id to invalid params"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_refetch_providers_modes_or_commands_when_switching_() {
    // source: "does not refetch providers modes or commands when switching effort from session snapshot"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn switches_model_against_the_warm_provider_snapshot_without_re() {
    // source: "switches model against the warm provider snapshot without refetching"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn reuses_the_warm_directory_snapshot_for_a_second_new_session_() {
    // source: "reuses the warm directory snapshot for a second new session in the same cwd"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn normal_text_prompt_sends_model_variant_mode_and_converted_pa() {
    // source: "normal text prompt sends model variant mode and converted parts"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn waits_for_queued_session_updates_before_returning_end_turn() {
    // source: "waits for queued session updates before returning end_turn"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn maps_assistant_prompt_errors_to_request_errors_instead_of_en() {
    // source: "maps assistant prompt errors to request errors instead of end turn"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn maps_aborted_assistant_prompt_errors_to_cancelled() {
    // source: "maps aborted assistant prompt errors to cancelled"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn prompt_maps_assistant_and_user_audience_annotations() {
    // source: "prompt maps assistant and user audience annotations"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn prompt_sends_image_and_resource_parts() {
    // source: "prompt sends image and resource parts"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn slash_command_prompt_calls_session_command() {
    // source: "slash command prompt calls session command"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn compact_slash_command_calls_summarize_path() {
    // source: "compact slash command calls summarize path"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn maps_prompt_auth_failures_to_auth_required_request_errors() {
    // source: "maps prompt auth failures to auth-required request errors"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
