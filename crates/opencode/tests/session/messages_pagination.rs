// source: test/session/messages-pagination.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { describe, expect, test } from "bun:test"; import { LayerNode } from "@opencode-ai/core/effect/layer-node"; impo

#[test]
fn message_v2_page() {
    // source: "MessageV2.page"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_page_result() {
    // source: "returns page result"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn pages_backward_with_opaque_cursors() {
    // source: "pages backward with opaque cursors"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_items_in_chronological_order_within_a_page() {
    // source: "returns items in chronological order within a page"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_empty_items_for_session_with_no_messages() {
    // source: "returns empty items for session with no messages"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn fails_with_not_found_error_for_non_existent_session() {
    // source: "fails with NotFoundError for non-existent session"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn handles_exact_limit_boundary() {
    // source: "handles exact limit boundary"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn limit_of_1_returns_single_newest_message() {
    // source: "limit of 1 returns single newest message"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn hydrates_multiple_parts_per_message() {
    // source: "hydrates multiple parts per message"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn accepts_cursors_from_fractional_timestamps() {
    // source: "accepts cursors from fractional timestamps"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn messages_with_same_timestamp_are_ordered_by_id() {
    // source: "messages with same timestamp are ordered by id"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_return_messages_from_other_sessions() {
    // source: "does not return messages from other sessions"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn large_limit_returns_all_messages_without_cursor() {
    // source: "large limit returns all messages without cursor"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn message_v2_stream() {
    // source: "MessageV2.stream"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn yields_items_newest_first() {
    // source: "yields items newest first"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn yields_nothing_for_empty_session() {
    // source: "yields nothing for empty session"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn yields_single_message() {
    // source: "yields single message"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn hydrates_parts_for_each_yielded_message() {
    // source: "hydrates parts for each yielded message"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn handles_sets_exceeding_internal_page_size() {
    // source: "handles sets exceeding internal page size"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_an_effect() {
    // source: "returns an Effect"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn message_v2_parts() {
    // source: "MessageV2.parts"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_parts_for_a_message() {
    // source: "returns parts for a message"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_empty_array_for_message_with_no_parts() {
    // source: "returns empty array for message with no parts"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_multiple_parts_in_order() {
    // source: "returns multiple parts in order"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_empty_for_non_existent_message_id() {
    // source: "returns empty for non-existent message id"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn parts_contain_session_id_and_message_id() {
    // source: "parts contain sessionID and messageID"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn message_v2_get() {
    // source: "MessageV2.get"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_message_with_hydrated_parts() {
    // source: "returns message with hydrated parts"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn fails_with_not_found_error_for_non_existent_message() {
    // source: "fails with NotFoundError for non-existent message"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn scopes_by_session_id() {
    // source: "scopes by session id"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_message_with_multiple_parts() {
    // source: "returns message with multiple parts"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_assistant_message_with_correct_role() {
    // source: "returns assistant message with correct role"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_message_with_zero_parts() {
    // source: "returns message with zero parts"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn session_messages() {
    // source: "Session.messages"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_all_messages_in_chronological_order_across_pages() {
    // source: "returns all messages in chronological order across pages"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn session_find_message() {
    // source: "Session.findMessage"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn searches_newest_first() {
    // source: "searches newest-first"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn message_v2_filter_compacted() {
    // source: "MessageV2.filterCompacted"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_all_messages_when_no_compaction() {
    // source: "returns all messages when no compaction"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn stops_at_compaction_boundary_and_returns_chronological_order() {
    // source: "stops at compaction boundary and returns chronological order"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn handles_empty_iterable() {
    // source: "handles empty iterable"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_break_on_compaction_part_without_matching_summary() {
    // source: "does not break on compaction part without matching summary"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn skips_assistant_with_error_even_if_marked_as_summary() {
    // source: "skips assistant with error even if marked as summary"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn skips_assistant_without_finish_even_if_marked_as_summary() {
    // source: "skips assistant without finish even if marked as summary"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn retains_original_tail_when_compaction_stores_tail_start_id() {
    // source: "retains original tail when compaction stores tail_start_id"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn fork_remaps_compaction_tail_start_id_for_filter_compacted() {
    // source: "fork remaps compaction tail_start_id for filterCompacted"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn retains_an_assistant_tail_when_compaction_starts_inside_a_tu() {
    // source: "retains an assistant tail when compaction starts inside a turn"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn prefers_latest_compaction_boundary_when_repeated_compactions() {
    // source: "prefers latest compaction boundary when repeated compactions exist"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn works_with_array_input() {
    // source: "works with array input"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn message_v2_cursor() {
    // source: "MessageV2.cursor"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn encode_decode_roundtrip() {
    // source: "encode/decode roundtrip"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn encode_decode_with_fractional_time() {
    // source: "encode/decode with fractional time"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn encoded_cursor_is_base64url() {
    // source: "encoded cursor is base64url"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn message_v2_consistency() {
    // source: "MessageV2 consistency"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn page_hydration_matches_get_for_each_message() {
    // source: "page hydration matches get for each message"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn parts_from_get_match_standalone_parts_call() {
    // source: "parts from get match standalone parts call"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn stream_collects_same_messages_as_exhaustive_page_iteration() {
    // source: "stream collects same messages as exhaustive page iteration"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn filter_compacted_of_full_stream_returns_same_as_array_from_w() {
    // source: "filterCompacted of full stream returns same as Array.from when no compaction"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
