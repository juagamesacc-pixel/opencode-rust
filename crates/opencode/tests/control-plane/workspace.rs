// source: test/control-plane/workspace.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { afterEach, beforeEach, describe, expect, mock, test } from "bun:test"; import { $ } from "bun"; import fs from

#[test]
fn workspace_schemas_and_exports() {
    // source: "workspace schemas and exports"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn keeps_the_historical_event_type_names() {
    // source: "keeps the historical event type names"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn validates_create_input_with_workspace_id_project_id_branch_t() {
    // source: "validates create input with workspace id, project id, branch, type, and extra"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn workspace_crud() {
    // source: "workspace CRUD"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_returns_undefined_for_a_missing_workspace() {
    // source: "get returns undefined for a missing workspace"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn list_maps_database_rows_filters_by_project_and_sorts_by_id() {
    // source: "list maps database rows, filters by project, and sorts by id"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn list_is_disabled_by_the_experimental_workspace_flag() {
    // source: "list is disabled by the experimental workspace flag"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn create_configures_persists_creates_starts_local_sync_and_pas() {
    // source: "create configures, persists, creates, starts local sync, and passes environment"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn create_propagates_configure_failures_and_does_not_insert_a_w() {
    // source: "create propagates configure failures and does not insert a workspace"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn create_leaves_the_inserted_row_when_adapter_create_fails() {
    // source: "create leaves the inserted row when adapter create fails"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn create_returns_after_a_local_workspace_reports_error() {
    // source: "create returns after a local workspace reports error"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn sync_list_registers_adapter_listed_workspaces_that_are_missi() {
    // source: "syncList registers adapter-listed workspaces that are missing by name"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn sync_list_calls_every_registered_adapter_with_a_list_method() {
    // source: "syncList calls every registered adapter with a list method"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn remote_create_connects_to_routed_event_and_history_endpoints() {
    // source: "remote create connects to routed event and history endpoints"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn remove_returns_undefined_for_a_missing_workspace() {
    // source: "remove returns undefined for a missing workspace"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn remove_deletes_the_workspace_associated_sessions_adapter_res() {
    // source: "remove deletes the workspace, associated sessions, adapter resources, and status"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn remove_still_deletes_the_row_when_the_adapter_cannot_remove_() {
    // source: "remove still deletes the row when the adapter cannot remove resources"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn session_warp_moves_a_session_into_a_local_workspace_and_clai() {
    // source: "sessionWarp moves a session into a local workspace and claims ownership"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn session_warp_applies_source_workspace_patch_to_local_target_() {
    // source: "sessionWarp applies source workspace patch to local target workspace"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn session_warp_detaches_a_session_to_the_local_project_and_cla() {
    // source: "sessionWarp detaches a session to the local project and claims project ownership"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn session_warp_syncs_previous_remote_history_replays_it_steals() {
    // source: "sessionWarp syncs previous remote history, replays it, steals, and claims the sequence"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn workspace_sync_state() {
    // source: "workspace sync state"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn start_workspace_syncing_is_disabled_by_the_experimental_work() {
    // source: "startWorkspaceSyncing is disabled by the experimental workspace flag"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn start_workspace_syncing_starts_all_workspaces() {
    // source: "startWorkspaceSyncing starts all workspaces"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn local_start_reports_error_when_the_target_directory_is_missi() {
    // source: "local start reports error when the target directory is missing"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn duplicate_local_status_updates_are_suppressed() {
    // source: "duplicate local status updates are suppressed"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn remote_start_emits_disconnected_connecting_and_connected_the() {
    // source: "remote start emits disconnected, connecting, and connected then refuses duplicate listeners"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn remote_connection_http_failures_set_error_and_clear_syncing() {
    // source: "remote connection HTTP failures set error and clear syncing"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn remote_history_http_failures_set_error() {
    // source: "remote history HTTP failures set error"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn sync_history_sends_the_local_sequence_fence_and_replays_retu() {
    // source: "sync history sends the local sequence fence and replays returned events in workspace context"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn sse_forwards_non_heartbeat_events_and_ignores_heartbeats() {
    // source: "SSE forwards non-heartbeat events and ignores heartbeats"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn sse_sync_events_are_replayed_and_forwarded() {
    // source: "SSE sync events are replayed and forwarded"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn workspace_wait_for_sync() {
    // source: "workspace waitForSync"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_immediately_for_an_empty_fence() {
    // source: "returns immediately for an empty fence"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_immediately_when_the_stored_sequence_already_satisfi() {
    // source: "returns immediately when the stored sequence already satisfies the fence"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn waits_until_the_database_reaches_the_requested_sequence_and_() {
    // source: "waits until the database reaches the requested sequence and a workspace event arrives"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn event() {
    // source: "event"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn a_sync_event_for_a_different_workspace_can_also_release_the_() {
    // source: "a sync event for a different workspace can also release the fence"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rejects_with_the_abort_reason_when_aborted() {
    // source: "rejects with the abort reason when aborted"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn times_out_with_the_requested_fence_in_the_error_message() {
    // source: "times out with the requested fence in the error message"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
