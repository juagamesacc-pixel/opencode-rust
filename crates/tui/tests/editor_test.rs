//! Port of `test/editor.test.ts` (opencode v1.18.30).
//!
//! `normalizePromptContent` cases plus the missing-editor rejection. The
//! editor cases mutate `EDITOR`/`VISUAL`, so an RAII guard restores them
//! (the port equivalent of `afterEach`).
use tui::editor::{normalize_prompt_content, open_editor, EditorRenderer, OpenEditorInput};

struct ScriptRenderer;

impl EditorRenderer for ScriptRenderer {
    fn suspend(&mut self) {}
    fn resume(&mut self) {}
    fn request_render(&mut self) {}
    fn clear_render_buffer(&mut self) {}
}

/// Restores `EDITOR`/`VISUAL` on drop (mirrors `afterEach`).
struct EnvGuard {
    editor: Option<String>,
    visual: Option<String>,
}

impl EnvGuard {
    fn take() -> Self {
        Self {
            editor: std::env::var("EDITOR").ok(),
            visual: std::env::var("VISUAL").ok(),
        }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.editor {
            Some(value) => std::env::set_var("EDITOR", value),
            None => std::env::remove_var("EDITOR"),
        }
        match &self.visual {
            Some(value) => std::env::set_var("VISUAL", value),
            None => std::env::remove_var("VISUAL"),
        }
    }
}

#[test]
fn rejects_when_the_external_editor_cannot_start() {
    let _guard = EnvGuard::take();
    std::env::remove_var("VISUAL");
    std::env::set_var("EDITOR", "opencode-editor-that-does-not-exist");
    let mut renderer = ScriptRenderer;
    let result = open_editor(OpenEditorInput {
        value: "original".to_string(),
        renderer: &mut renderer,
        cwd: None,
        stdin: None,
    });
    assert!(result.is_err(), "expected Err, got {result:?}");
}

#[test]
fn normalizes_a_single_trailing_editor_newline_for_one_line_prompts() {
    assert_eq!(normalize_prompt_content("hello\n"), "hello");
    assert_eq!(normalize_prompt_content("hello\r\n"), "hello");
}

#[test]
fn preserves_multiline_prompts_that_end_with_a_newline() {
    assert_eq!(normalize_prompt_content("hello\nworld\n"), "hello\nworld\n");
}
