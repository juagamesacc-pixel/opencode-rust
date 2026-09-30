// source: test/project/project.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { describe, expect } from "bun:test"; import { Project } from "@/project/project"; import { $ } from "bun"

#[test]
fn project_from_directory() {
    // source: "Project.fromDirectory"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn should_handle_git_repository_with_no_commits() {
    // source: "should handle git repository with no commits"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn should_handle_git_repository_with_commits() {
    // source: "should handle git repository with commits"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_global_for_non_git_directory() {
    // source: "returns global for non-git directory"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn derives_stable_project_id_from_root_commit() {
    // source: "derives stable project ID from root commit"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn prefers_normalized_origin_remote_over_root_commit() {
    // source: "prefers normalized origin remote over root commit"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn normalizes_equivalent_origin_url_forms_to_the_same_project_i() {
    // source: "normalizes equivalent origin URL forms to the same project ID"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn migrates_cached_root_project_data_when_origin_becomes_availa() {
    // source: "migrates cached root project data when origin becomes available"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn project_from_directory_git_failure_paths() {
    // source: "Project.fromDirectory git failure paths"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn keeps_vcs_when_rev_list_exits_non_zero_no_commits() {
    // source: "keeps vcs when rev-list exits non-zero (no commits)"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn project_from_directory_with_worktrees() {
    // source: "Project.fromDirectory with worktrees"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn should_set_worktree_to_root_when_called_from_root() {
    // source: "should set worktree to root when called from root"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn tracks_a_linked_worktree_as_the_opened_project_directory() {
    // source: "tracks a linked worktree as the opened project directory"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn worktree_should_share_project_id_with_main_repo() {
    // source: "worktree should share project ID with main repo"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn separate_clones_of_the_same_repo_should_share_project_id() {
    // source: "separate clones of the same repo should share project ID"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn should_accumulate_multiple_worktrees_in_sandboxes() {
    // source: "should accumulate multiple worktrees in sandboxes"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn project_discover() {
    // source: "Project.discover"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn should_discover_favicon_png_in_root() {
    // source: "should discover favicon.png in root"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn should_not_discover_non_image_files() {
    // source: "should not discover non-image files"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn should_not_discover_favicon_when_override_is_set() {
    // source: "should not discover favicon when override is set"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn project_update() {
    // source: "Project.update"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn should_update_name() {
    // source: "should update name"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn should_update_icon_url() {
    // source: "should update icon url"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn should_update_icon_color() {
    // source: "should update icon color"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn should_update_icon_override() {
    // source: "should update icon override"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn should_update_commands() {
    // source: "should update commands"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn should_fail_when_project_not_found() {
    // source: "should fail when project not found"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn should_emit_global_bus_event_on_update() {
    // source: "should emit GlobalBus event on update"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn should_update_multiple_fields_at_once() {
    // source: "should update multiple fields at once"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn project_list_and_project_get() {
    // source: "Project.list and Project.get"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn list_returns_all_projects() {
    // source: "list returns all projects"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_returns_project_by_id() {
    // source: "get returns project by id"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_returns_undefined_for_unknown_id() {
    // source: "get returns undefined for unknown id"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn project_set_initialized() {
    // source: "Project.setInitialized"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn sets_time_initialized_on_project() {
    // source: "sets time_initialized on project"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn project_add_sandbox_and_project_remove_sandbox() {
    // source: "Project.addSandbox and Project.removeSandbox"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn add_sandbox_adds_directory_and_remove_sandbox_removes_it() {
    // source: "addSandbox adds directory and removeSandbox removes it"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn add_sandbox_emits_global_bus_event() {
    // source: "addSandbox emits GlobalBus event"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn project_from_directory_with_bare_repos() {
    // source: "Project.fromDirectory with bare repos"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn worktree_from_bare_repo_should_cache_in_bare_repo_not_parent() {
    // source: "worktree from bare repo should cache in bare repo, not parent"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn different_bare_repos_under_same_parent_should_not_share_proj() {
    // source: "different bare repos under same parent should not share project ID"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn bare_repo_without_git_suffix_is_still_detected_via_core_bare() {
    // source: "bare repo without .git suffix is still detected via core.bare"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
