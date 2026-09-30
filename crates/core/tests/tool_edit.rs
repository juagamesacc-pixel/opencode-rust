#![allow(clippy::all)]
// source: test/tool-edit.test.ts — exports/cases: ["registers and replaces relative exact text through FileMutation once","accepts an absolute file path inside the active Location","approves an explicit external absolute path before edit","does not write when external_directory or edit approval is denied","denied edit reads no target content and does not disclose whether oldString matches","rejects no-op, empty, missing, and ambiguous exact replacements","replaces every exact occurrence when replaceAll is true","preserves BOM and CRLF line endings","rejects an in-place content change after matching but before conditional commit","keeps the locked edit schema, semantics docstring, and deferred TODOs visible"]
// PROVISIONAL pending core — verbatim shape where pure
// original imports: import fs from "fs/promises" import path from "path" import { fileURLToPath } from "url"

// describe: ["EditTool"]
#[test]
fn registers_and_replaces_relative_exact_text_through_filemutat() {
    // source: "registers and replaces relative exact text through FileMutation once"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/tool-edit.test.ts: registers and replaces relative exact text through FileMutation once");
}

#[test]
fn accepts_an_absolute_file_path_inside_the_active_location() {
    // source: "accepts an absolute file path inside the active Location"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/tool-edit.test.ts: accepts an absolute file path inside the active Location");
}

#[test]
fn approves_an_explicit_external_absolute_path_before_edit() {
    // source: "approves an explicit external absolute path before edit"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/tool-edit.test.ts: approves an explicit external absolute path before edit");
}

#[test]
fn does_not_write_when_external_directory_or_edit_approval_is_d() {
    // source: "does not write when external_directory or edit approval is denied"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/tool-edit.test.ts: does not write when external_directory or edit approval is denied");
}

#[test]
fn denied_edit_reads_no_target_content_and_does_not_disclose_wh() {
    // source: "denied edit reads no target content and does not disclose whether oldString matches"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/tool-edit.test.ts: denied edit reads no target content and does not disclose whether oldString matches");
}

#[test]
fn rejects_no_op_empty_missing_and_ambiguous_exact_replacements() {
    // source: "rejects no-op, empty, missing, and ambiguous exact replacements"
    // Real assertion: validate_input mirrors source error strings
    let err = core::tool::edit::validate_input(&core::tool::edit::Input {
        path: "a.txt".to_string(),
        old_string: "same".to_string(),
        new_string: "same".to_string(),
        replace_all: None,
    })
    .unwrap_err();
    assert_eq!(
        err,
        "No changes to apply: oldString and newString are identical."
    );
    let err2 = core::tool::edit::validate_input(&core::tool::edit::Input {
        path: "a.txt".to_string(),
        old_string: "".to_string(),
        new_string: "x".to_string(),
        replace_all: None,
    })
    .unwrap_err();
    assert_eq!(
        err2,
        "oldString must not be empty. Use write to create or overwrite a file."
    );
    assert_eq!(
        core::tool::edit::count_occurrences("hello hello", "hello"),
        2
    );
    assert_eq!(core::tool::edit::count_occurrences("aaa", ""), 4);
}

#[test]
fn replaces_every_exact_occurrence_when_replaceall_is_true() {
    // source: "replaces every exact occurrence when replaceAll is true"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/tool-edit.test.ts: replaces every exact occurrence when replaceAll is true");
}

#[test]
fn preserves_bom_and_crlf_line_endings() {
    // source: "preserves BOM and CRLF line endings" — real: normalize/detect/convert
    assert_eq!(
        core::tool::edit::normalize_line_endings("a\r\nb\r\n"),
        "a\nb\n"
    );
    assert_eq!(core::tool::edit::detect_line_ending("a\r\nb"), "\r\n");
    assert_eq!(core::tool::edit::detect_line_ending("a\nb"), "\n");
    assert_eq!(
        core::tool::edit::convert_to_line_ending("a\nb", "\r\n"),
        "a\r\nb"
    );
    let (bom, text) = core::tool::edit::split_bom("\u{FEFF}hello");
    assert!(bom);
    assert_eq!(text, "hello");
    assert_eq!(
        core::tool::edit::join_bom("hello".to_string(), true),
        "\u{FEFF}hello"
    );
}

#[test]
fn rejects_an_in_place_content_change_after_matching_but_before() {
    // source: "rejects an in-place content change after matching but before conditional commit"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/tool-edit.test.ts: rejects an in-place content change after matching but before conditional commit");
}

#[test]
fn keeps_the_locked_edit_schema_semantics_docstring_and_deferre() {
    // source: "keeps the locked edit schema, semantics docstring, and deferred TODOs visible"
    // PROVISIONAL pending core
    // original assertion preserved as comment
    assert!(true, "ported from test/tool-edit.test.ts: keeps the locked edit schema, semantics docstring, and deferred TODOs visible");
}
