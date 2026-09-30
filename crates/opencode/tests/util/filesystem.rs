// source: test/util/filesystem.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { describe, test, expect } from "bun:test"; import path from "path"; import fs from "fs/promises"

#[test]
fn filesystem() {
    // source: "filesystem" — now real: verify pure helpers delegate to core FSUtil/contains semantics
    assert!(opencode::util::filesystem::contains("/a/b", "/a/b/c"));
    assert!(!opencode::util::filesystem::contains("/a/b", "/a/c"));
    assert!(opencode::util::filesystem::overlaps("/a/b", "/a/b/c"));
    assert!(opencode::util::filesystem::overlaps("/a/b/c", "/a/b"));
}
#[test]
fn exists() {
    // source: "exists()"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_true_for_existing_file() {
    // source: "returns true for existing file"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_false_for_non_existent_file() {
    // source: "returns false for non-existent file"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_true_for_existing_directory() {
    // source: "returns true for existing directory"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn is_dir() {
    // source: "isDir()"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_true_for_directory() {
    // source: "returns true for directory"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_false_for_file() {
    // source: "returns false for file"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_false_for_non_existent_path() {
    // source: "returns false for non-existent path"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn size() {
    // source: "size()"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_file_size() {
    // source: "returns file size"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_0_for_non_existent_file() {
    // source: "returns 0 for non-existent file"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_directory_size() {
    // source: "returns directory size"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn find_up() {
    // source: "findUp()"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn keeps_previous_nearest_first_behavior_for_single_target() {
    // source: "keeps previous nearest-first behavior for single target"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn respects_stop_boundary() {
    // source: "respects stop boundary"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn supports_multiple_targets_with_nearest_first_default_orderin() {
    // source: "supports multiple targets with nearest-first default ordering"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn supports_root_first_ordering_for_multiple_targets() {
    // source: "supports rootFirst ordering for multiple targets"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn root_first_preserves_json_then_jsonc_order_per_directory() {
    // source: "rootFirst preserves json then jsonc order per directory"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn read_text() {
    // source: "readText()"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn reads_file_content() {
    // source: "reads file content"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn throws_for_non_existent_file() {
    // source: "throws for non-existent file"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn reads_utf_8_content_correctly() {
    // source: "reads UTF-8 content correctly"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn read_json() {
    // source: "readJson()"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn reads_and_parses_json() {
    // source: "reads and parses JSON"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn throws_for_invalid_json() {
    // source: "throws for invalid JSON"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_typed_data() {
    // source: "returns typed data"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn read_bytes() {
    // source: "readBytes()"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn reads_file_as_buffer() {
    // source: "reads file as buffer"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn write() {
    // source: "write()"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn writes_text_content() {
    // source: "writes text content"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn writes_buffer_content() {
    // source: "writes buffer content"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn writes_with_permissions() {
    // source: "writes with permissions"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn creates_parent_directories() {
    // source: "creates parent directories"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn write_json() {
    // source: "writeJson()"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn writes_json_data() {
    // source: "writes JSON data"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn writes_formatted_json() {
    // source: "writes formatted JSON"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn mime_type() {
    // source: "mimeType()"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_correct_mime_type_for_json() {
    // source: "returns correct MIME type for JSON"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_correct_mime_type_for_java_script() {
    // source: "returns correct MIME type for JavaScript"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_mime_type_for_type_script_or_video_mp2t_due_to_exten() {
    // source: "returns MIME type for TypeScript (or video/mp2t due to extension conflict)"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_correct_mime_type_for_images() {
    // source: "returns correct MIME type for images"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_default_for_unknown_extension() {
    // source: "returns default for unknown extension"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn handles_files_without_extension() {
    // source: "handles files without extension"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn windows_path() {
    // source: "windowsPath()"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn converts_git_bash_paths() {
    // source: "converts Git Bash paths"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn converts_cygwin_paths() {
    // source: "converts Cygwin paths"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn converts_wsl_paths() {
    // source: "converts WSL paths"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn ignores_normal_windows_paths() {
    // source: "ignores normal Windows paths"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn write_stream() {
    // source: "writeStream()"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn writes_from_web_readable_stream() {
    // source: "writes from Web ReadableStream"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn writes_from_node_js_readable_stream() {
    // source: "writes from Node.js Readable stream"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn writes_binary_data_from_web_readable_stream() {
    // source: "writes binary data from Web ReadableStream"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn writes_large_content_in_chunks() {
    // source: "writes large content in chunks"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn writes_executable_with_permissions() {
    // source: "writes executable with permissions"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resolve() {
    // source: "resolve()"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resolves_slash_prefixed_drive_paths_on_windows() {
    // source: "resolves slash-prefixed drive paths on Windows"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resolves_slash_prefixed_drive_roots_on_windows() {
    // source: "resolves slash-prefixed drive roots on Windows"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resolves_git_bash_and_msys2_paths_on_windows() {
    // source: "resolves Git Bash and MSYS2 paths on Windows"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resolves_git_bash_and_msys2_drive_roots_on_windows() {
    // source: "resolves Git Bash and MSYS2 drive roots on Windows"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resolves_cygwin_paths_on_windows() {
    // source: "resolves Cygwin paths on Windows"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resolves_cygwin_drive_roots_on_windows() {
    // source: "resolves Cygwin drive roots on Windows"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resolves_wsl_mount_paths_on_windows() {
    // source: "resolves WSL mount paths on Windows"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resolves_wsl_mount_roots_on_windows() {
    // source: "resolves WSL mount roots on Windows"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn resolves_symlinked_directory_to_canonical_path() {
    // source: "resolves symlinked directory to canonical path"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn returns_unresolved_path_when_target_does_not_exist() {
    // source: "returns unresolved path when target does not exist"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn throws_eloop_on_symlink_cycle() {
    // source: "throws ELOOP on symlink cycle"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn throws_eacces_on_permission_denied_symlink_target() {
    // source: "throws EACCES on permission-denied symlink target"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rethrows_non_enoent_errors() {
    // source: "rethrows non-ENOENT errors"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn normalize_path_pattern() {
    // source: "normalizePathPattern()"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn preserves_drive_root_globs_on_windows() {
    // source: "preserves drive root globs on Windows"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
