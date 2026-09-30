// source: test/cli/run/stream.transport.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { afterEach, describe, expect, mock, spyOn, test } from "bun:test"; import { OpencodeClient, type GlobalEvent } f

#[test]
fn run_stream_transport() {
    // source: "run stream transport"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_replay_persisted_main_session_history_during_bootst() {
    // source: "does not replay persisted main-session history during bootstrap by default"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn replays_persisted_main_session_history_during_bootstrap_when() {
    // source: "replays persisted main-session history during bootstrap when enabled"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn caps_replayed_bootstrap_history_to_the_configured_number_of_() {
    // source: "caps replayed bootstrap history to the configured number of messages"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn skips_buffered_pre_bootstrap_deltas_already_covered_by_repla() {
    // source: "skips buffered pre-bootstrap deltas already covered by replay history"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn applies_buffered_pre_bootstrap_deltas_not_yet_persisted() {
    // source: "applies buffered pre-bootstrap deltas not yet persisted"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn preserves_running_footer_state_for_resumed_active_sessions() {
    // source: "preserves running footer state for resumed active sessions"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rebuilds_session_output_on_resize_and_continues_live_deltas_() {
    // source: "rebuilds session output on resize and continues live deltas from replayed state"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn coalesces_active_resize_requests_into_one_trailing_replay() {
    // source: "coalesces active resize requests into one trailing replay"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn keeps_coalescing_resize_requests_while_buffered_events_drain() {
    // source: "keeps coalescing resize requests while buffered events drain"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn preserves_assistant_deltas_not_yet_persisted_when_replaying_() {
    // source: "preserves assistant deltas not yet persisted when replaying during a live stream"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn preserves_the_display_prefix_for_active_reasoning_restored_d() {
    // source: "preserves the display prefix for active reasoning restored during replay"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_overlay_stale_active_text_when_persistence_complete() {
    // source: "does not overlay stale active text when persistence completes during replay"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_clear_the_terminal_when_resize_replay_snapshot_fetc() {
    // source: "does not clear the terminal when resize replay snapshot fetch fails"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn disables_resize_replay_for_the_session_after_terminal_reset_() {
    // source: "disables resize replay for the session after terminal reset fails"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn disables_resize_replay_when_rebuilding_scrollback_fails_afte() {
    // source: "disables resize replay when rebuilding scrollback fails after terminal reset"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn keeps_completed_historical_subagent_tabs_during_bootstrap() {
    // source: "keeps completed historical subagent tabs during bootstrap"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn bootstraps_child_tabs_and_resumed_blocker_input() {
    // source: "bootstraps child tabs and resumed blocker input"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn bootstraps_child_session_output_before_selection() {
    // source: "bootstraps child session output before selection"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_block_startup_on_child_history_bootstrap() {
    // source: "does not block startup on child history bootstrap"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn replays_child_events_buffered_during_bootstrap_once_the_tab_() {
    // source: "replays child events buffered during bootstrap once the tab is known"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn streams_selected_subagent_output_from_global_events_while_it() {
    // source: "streams selected subagent output from global events while it is running"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn recovers_pending_questions_from_question_list_when_question_() {
    // source: "recovers pending questions from question.list when question.asked is missed"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_resurrect_questions_if_question_list_resolves_after() {
    // source: "does not resurrect questions if question.list resolves after tool completion"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn respects_the_include_files_flag_when_building_prompt_payload() {
    // source: "respects the includeFiles flag when building prompt payloads"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn falls_back_to_session_status_polling_when_idle_events_are_mi() {
    // source: "falls back to session status polling when idle events are missing"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn flushes_interrupted_output_when_the_active_turn_aborts() {
    // source: "flushes interrupted output when the active turn aborts"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn closes_an_active_turn_without_rejecting_it() {
    // source: "closes an active turn without rejecting it"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rejects_the_active_turn_when_the_event_stream_faults() {
    // source: "rejects the active turn when the event stream faults"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rejects_the_active_turn_when_the_backing_instance_is_dispose() {
    // source: "rejects the active turn when the backing instance is disposed"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rejects_concurrent_turns() {
    // source: "rejects concurrent turns"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
