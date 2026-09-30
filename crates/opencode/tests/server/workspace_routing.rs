// source: test/server/workspace-routing.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { describe, expect, test } from "bun:test"; import {; import { SessionID } from "../../src/session/schema"

#[test]
fn is_local_workspace_route() {
    // source: "isLocalWorkspaceRoute"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_session_is_local() {
    // source: "GET /session is local"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_session_ses_abc_is_local_prefix_match() {
    // source: "GET /session/ses_abc is local (prefix match)"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn post_session_is_not_local_method_mismatch() {
    // source: "POST /session is not local (method mismatch)"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn session_status_is_forwarded_regardless_of_method() {
    // source: "/session/status is forwarded regardless of method"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn unrecognized_paths_are_not_local() {
    // source: "unrecognized paths are not local"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_workspace_route_session_id() {
    // source: "getWorkspaceRouteSessionID"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn extracts_session_id_from_path() {
    // source: "extracts session ID from path"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn extracts_session_id_without_trailing_path() {
    // source: "extracts session ID without trailing path"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn extracts_session_id_from_experimental_background_path() {
    // source: "extracts session ID from experimental background path"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_null_for_session_status() {
    // source: "returns null for /session/status"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_null_for_non_session_paths() {
    // source: "returns null for non-session paths"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_null_for_bare_session_path() {
    // source: "returns null for bare /session path"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn workspace_proxy_url() {
    // source: "workspaceProxyURL"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn appends_request_path_to_target() {
    // source: "appends request path to target"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn strips_trailing_slash_on_target_before_appending() {
    // source: "strips trailing slash on target before appending"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn preserves_query_params_from_request_but_removes_workspace() {
    // source: "preserves query params from request but removes workspace"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn strips_the_host_directory_param_so_the_remote_resolves_its_o() {
    // source: "strips the host directory param so the remote resolves its own root"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn preserves_hash_from_request() {
    // source: "preserves hash from request"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn works_with_url_object_as_target() {
    // source: "works with URL object as target"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
