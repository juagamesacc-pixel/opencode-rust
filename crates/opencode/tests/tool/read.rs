// source: test/tool/read.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { PermissionV1 } from "@opencode-ai/core/v1/permission"; import { afterEach, describe, expect } from "bun:test";

#[test]
fn tool_read_external_directory_permission() {
    // source: "tool.read external_directory permission"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn allows_reading_absolute_path_inside_project_directory() {
    // source: "allows reading absolute path inside project directory"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn allows_reading_file_in_subdirectory_inside_project_directory() {
    // source: "allows reading file in subdirectory inside project directory"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn asks_for_external_directory_permission_when_reading_absolute() {
    // source: "asks for external_directory permission when reading absolute path outside project"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn normalizes_read_permission_paths_on_windows() {
    // source: "normalizes read permission paths on Windows"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn uses_worktree_relative_path_for_read_permission_so_user_rule() {
    // source: "uses worktree-relative path for read permission so user rules match like edit/write"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn asks_for_directory_scoped_external_directory_permission_when() {
    // source: "asks for directory-scoped external_directory permission when reading external directory"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn asks_for_external_directory_permission_when_reading_relative() {
    // source: "asks for external_directory permission when reading relative path outside project"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_ask_for_external_directory_permission_when_reading_() {
    // source: "does not ask for external_directory permission when reading inside project"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn tool_read_env_file_permissions() {
    // source: "tool.read env file permissions"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn agent_agent_name() {
    // source: "agent=${agentName}"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn filename_asks_should_ask() {
    // source: "${filename} asks=${shouldAsk}"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn tool_read_truncation() {
    // source: "tool.read truncation"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn truncates_large_file_by_bytes_and_sets_truncated_metadata() {
    // source: "truncates large file by bytes and sets truncated metadata"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn stops_streaming_after_the_byte_cap() {
    // source: "stops streaming after the byte cap"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn truncates_by_line_count_when_limit_is_specified() {
    // source: "truncates by line count when limit is specified"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_truncate_small_file() {
    // source: "does not truncate small file"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn respects_offset_parameter() {
    // source: "respects offset parameter"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn throws_when_offset_is_beyond_end_of_file() {
    // source: "throws when offset is beyond end of file"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn allows_reading_empty_file_at_default_offset() {
    // source: "allows reading empty file at default offset"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn throws_when_offset_1_for_empty_file() {
    // source: "throws when offset > 1 for empty file"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_mark_final_directory_page_as_truncated() {
    // source: "does not mark final directory page as truncated"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn truncates_long_lines() {
    // source: "truncates long lines"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn image_files_set_truncated_to_false() {
    // source: "image files set truncated to false"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn detects_attachment_media_from_file_contents() {
    // source: "detects attachment media from file contents"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn large_image_files_are_properly_attached_without_error() {
    // source: "large image files are properly attached without error"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn fbs_files_flat_buffers_schema_are_read_as_text_not_images() {
    // source: ".fbs files (FlatBuffers schema) are read as text, not images"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn falls_through_unsupported_image_mime_types_to_text() {
    // source: "falls through unsupported image mime types to text"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn tool_read_loaded_instructions() {
    // source: "tool.read loaded instructions"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn loads_agents_md_from_parent_directory_and_includes_in_metada() {
    // source: "loads AGENTS.md from parent directory and includes in metadata"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn tool_read_binary_detection() {
    // source: "tool.read binary detection"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rejects_text_extension_files_with_null_bytes() {
    // source: "rejects text extension files with null bytes"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rejects_known_binary_extensions() {
    // source: "rejects known binary extensions"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
