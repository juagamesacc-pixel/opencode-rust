//! Rust port of `src/editor.ts` (opencode v1.18.30).
//!
//! External-editor flow (`$VISUAL`/`$EDITOR` round-trip through a temp file)
//! plus Claude-IDE lockfile discovery. Only std + serde_json are used:
//! spawning, temp files, and home-directory lookup map directly; the
//! renderer's suspend/resume/render surface becomes [`EditorRenderer`] (the
//! source takes a `CliRenderer`); Zed selection stays in
//! [`crate::editor_zed`].
//!
//! Original file: `packages/tui/src/editor.ts`

/// The renderer surface `openEditor` touches (`suspend`, `resume`,
/// `requestRender`, `currentRenderBuffer.clear`).
pub trait EditorRenderer {
    fn suspend(&mut self);
    fn resume(&mut self);
    fn request_render(&mut self);
    fn clear_render_buffer(&mut self);
}

/// An editor binary discovered from a `.lock` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorConnection {
    pub url: String,
    pub auth_token: Option<String>,
    pub source: String,
}

/// Mirrors `normalizePromptContent`: strip one trailing newline pair when the
/// rest of the prompt is a single line.
pub fn normalize_prompt_content(content: &str) -> String {
    for ending in ["\r\n", "\n"] {
        if let Some(body) = content.strip_suffix(ending) {
            if !body.contains('\n') && !body.contains('\r') {
                return body.to_string();
            }
            return content.to_string();
        }
    }
    content.to_string()
}

/// Home directory (`os.homedir()`).
pub fn home_dir() -> Option<String> {
    #[cfg(windows)]
    {
        std::env::var("USERPROFILE").ok()
    }
    #[cfg(not(windows))]
    {
        std::env::var("HOME").ok()
    }
}

/// Whether `path` is a file (`statSync` guarded `isFile`).
pub fn is_file(path: &str) -> bool {
    std::fs::metadata(path)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

/// Lexical `path.relative(parent, child)` (no I/O, like Node's).
pub fn relative_path(parent: &str, child: &str) -> String {
    use std::path::Component;
    let mut parents: Vec<String> = Vec::new();
    for component in std::path::Path::new(parent).components() {
        match component {
            Component::Normal(part) => parents.push(part.to_string_lossy().into_owned()),
            Component::RootDir | Component::Prefix(_) => {}
            Component::CurDir => {}
            Component::ParentDir => {
                parents.pop();
            }
        }
    }
    let mut children: Vec<String> = Vec::new();
    for component in std::path::Path::new(child).components() {
        match component {
            Component::Normal(part) => children.push(part.to_string_lossy().into_owned()),
            Component::RootDir | Component::Prefix(_) => {}
            Component::CurDir => {}
            Component::ParentDir => {
                children.pop();
            }
        }
    }
    let mut shared = 0;
    while shared < parents.len() && shared < children.len() && parents[shared] == children[shared] {
        shared += 1;
    }
    let mut parts = vec!["..".to_string(); parents.len() - shared];
    parts.extend(children[shared..].iter().cloned());
    parts.join("/")
}

/// Whether `child` is inside `parent` (mirrors `pathContains` in
/// `editor.ts`/`editor-zed.ts`: empty relative, or neither `..`-prefixed nor
/// absolute).
pub fn path_contains(parent: &str, child: &str) -> bool {
    // Resolve against the current directory for relative inputs, like
    // `path.resolve`. Absolute inputs resolve to themselves.
    let resolve = |value: &str| {
        let path = std::path::Path::new(value);
        if path.is_absolute() {
            return value.to_string();
        }
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::env::temp_dir());
        format!("{}/{}", cwd.to_string_lossy(), value)
    };
    let relative = relative_path(&resolve(parent), &resolve(child));
    relative.is_empty() || (!relative.starts_with("..") && !relative.starts_with('/'))
}

/// Input to [`open_editor`] (mirrors the inline parameter object).
pub struct OpenEditorInput<'a> {
    pub value: String,
    pub renderer: &'a mut dyn EditorRenderer,
    pub cwd: Option<String>,
    /// `stdin` (`EditorStdio`); `None` means `"inherit"`. Number/stream
    /// variants have no std equivalent and map to inherit.
    pub stdin: Option<std::process::Stdio>,
}

/// Mirrors `openEditor`: write `value` to a temp markdown file, suspend the
/// renderer, run `$VISUAL`/`$EDITOR` (literal-space split, like the source),
/// read the file back, and always clean up + resume. `None` when no editor
/// is configured or the file comes back empty.
pub fn open_editor(input: OpenEditorInput) -> Result<Option<String>, String> {
    let editor = std::env::var("VISUAL")
        .ok()
        .filter(|value| !value.is_empty())
        .or_else(|| {
            std::env::var("EDITOR")
                .ok()
                .filter(|value| !value.is_empty())
        });
    let Some(editor) = editor else {
        return Ok(None);
    };
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0);
    let file = std::env::temp_dir().join(format!("{now}.md"));

    let cleanup = |renderer: &mut dyn EditorRenderer| {
        let _ = std::fs::remove_file(&file);
        renderer.clear_render_buffer();
        renderer.resume();
        renderer.request_render();
    };

    std::fs::write(&file, &input.value).map_err(|error| error.to_string())?;
    input.renderer.suspend();
    input.renderer.clear_render_buffer();

    // Moved out before the `run` closure so the renderer stays usable for
    // cleanup (edition 2021 closures capture whole variables).
    let cwd = input.cwd;
    let stdin = input.stdin;

    let run = || -> Result<Option<String>, String> {
        let mut parts = editor.split(' ');
        let program = parts.next().unwrap_or("");
        let mut command = std::process::Command::new(program);
        command.args(parts.collect::<Vec<_>>());
        command.arg(&file);
        let cwd = cwd
            .as_deref()
            .filter(|cwd| is_file(cwd) || std::path::Path::new(cwd).is_dir())
            .map(str::to_string)
            .unwrap_or_else(|| {
                std::env::current_dir()
                    .unwrap_or_else(|_| std::env::temp_dir())
                    .to_string_lossy()
                    .into_owned()
            });
        command.current_dir(cwd);
        match stdin {
            // `None` means `"inherit"`; number/stream variants have no std
            // equivalent and map to inherit at the call site.
            Some(stdio) => {
                command.stdin(stdio);
            }
            None => {
                command.stdin(std::process::Stdio::inherit());
            }
        }
        command.stdout(std::process::Stdio::inherit());
        command.stderr(std::process::Stdio::inherit());
        #[cfg(windows)]
        {
            // `shell: process.platform === "win32"`.
            let mut shell = std::process::Command::new("cmd");
            shell.arg("/C").arg(program);
            let _ = shell;
        }
        let status = command.status().map_err(|error| error.to_string())?;
        if !status.success() {
            #[cfg(unix)]
            let signal: Option<i32> = std::os::unix::process::ExitStatusExt::signal(&status);
            #[cfg(not(unix))]
            let signal: Option<i32> = None;
            let reason = match signal {
                Some(signal) => format!("signal {signal}"),
                None => match status.code() {
                    Some(code) => format!("code {code}"),
                    None => "unknown termination".to_string(),
                },
            };
            return Err(format!("Editor exited with {reason}"));
        }
        let text =
            String::from_utf8_lossy(&std::fs::read(&file).map_err(|error| error.to_string())?)
                .into_owned();
        Ok(if text.is_empty() { None } else { Some(text) })
    };

    let result = run();
    cleanup(input.renderer);
    result
}

/// Mirrors `discoverEditorConnection`: scan `~/.claude/ide/*.lock` for the
/// best workspace match for `directory`.
pub fn discover_editor_connection(directory: &str) -> Option<EditorConnection> {
    let home = home_dir()?;
    let root = format!("{home}/.claude/ide");
    let entries = std::fs::read_dir(&root).ok()?;
    // (url, auth_token, source, score, mtime_ms)
    let mut candidates: Vec<(String, Option<String>, String, usize, f64)> = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let Some(port_text) = name.strip_suffix(".lock") else {
            continue;
        };
        let Ok(port) = port_text.parse::<i64>() else {
            continue;
        };
        if port <= 0 || port > 65535 {
            continue;
        }
        let file = format!("{root}/{name}");
        let Ok(contents) = std::fs::read_to_string(&file) else {
            continue;
        };
        let Ok(value) = serde_json::from_str::<serde_json::Value>(&contents) else {
            continue;
        };
        if value
            .get("transport")
            .and_then(|transport| transport.as_str())
            != Some("ws")
            && value.get("transport").is_some()
        {
            continue;
        }
        let folders: Vec<String> = value
            .get("workspaceFolders")
            .and_then(|folders| folders.as_array())
            .map(|folders| {
                folders
                    .iter()
                    .filter_map(|item| item.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        let score = folders
            .iter()
            .map(|folder| {
                if path_contains(folder, directory) {
                    folder.chars().count()
                } else {
                    0
                }
            })
            .max()
            .unwrap_or(0);
        if score == 0 {
            continue;
        }
        let mtime = entry
            .metadata()
            .ok()
            .and_then(|metadata| metadata.modified().ok())
            .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs_f64() * 1000.0)
            .unwrap_or(0.0);
        candidates.push((
            format!("ws://127.0.0.1:{port}"),
            value
                .get("authToken")
                .and_then(|token| token.as_str())
                .map(str::to_string),
            format!("lock:{port}"),
            score,
            mtime,
        ));
    }
    candidates.sort_by(|left, right| {
        right.3.cmp(&left.3).then(
            right
                .4
                .partial_cmp(&left.4)
                .unwrap_or(std::cmp::Ordering::Equal),
        )
    });
    candidates
        .into_iter()
        .next()
        .map(|(url, auth_token, source, _, _)| EditorConnection {
            url,
            auth_token,
            source,
        })
}

/// Mirrors `editorIntegration.connection`.
pub fn editor_integration_connection(directory: &str) -> Option<EditorConnection> {
    discover_editor_connection(directory)
}

/// Mirrors `editorIntegration.selection`.
pub fn editor_integration_selection(directory: &str) -> crate::editor_zed::ZedSelectionResult {
    crate::editor_zed::resolve_zed_selection(
        &crate::editor_zed::resolve_zed_db_path().unwrap_or_default(),
        directory,
    )
}

/// Mirrors `editorIntegration`.
pub struct EditorIntegration {
    pub connection: fn(&str) -> Option<EditorConnection>,
    pub selection: fn(&str) -> crate::editor_zed::ZedSelectionResult,
}

/// The shared editor integration (Claude lockfiles + Zed selection).
pub fn editor_integration() -> EditorIntegration {
    EditorIntegration {
        connection: editor_integration_connection,
        selection: editor_integration_selection,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_content_strips_a_single_trailing_newline() {
        assert_eq!(normalize_prompt_content("hello\n"), "hello");
        assert_eq!(normalize_prompt_content("hello\r\n"), "hello");
    }

    #[test]
    fn prompt_content_keeps_multiline_trailing_newlines() {
        assert_eq!(normalize_prompt_content("hello\nworld\n"), "hello\nworld\n");
        assert_eq!(normalize_prompt_content("hello"), "hello");
        assert_eq!(normalize_prompt_content(""), "");
    }

    #[test]
    fn relative_paths_match_node_semantics() {
        assert_eq!(relative_path("/a/b", "/a/b/c"), "c");
        assert_eq!(relative_path("/a/b/c", "/a/b"), "..");
        assert_eq!(relative_path("/a/b", "/a/b"), "");
        assert!(path_contains("/a/b", "/a/b/c"));
        assert!(path_contains("/a/b", "/a/b"));
        assert!(!path_contains("/a/b", "/a/bc"));
        assert!(!path_contains("/a/b", "/other"));
    }
}
