// source: src/config/tui-host-attention.ts — exports: resolveHostAttentionSoundPaths
// PROVISIONAL pending @opencode-ai/tui/config: non-record → {}, unknown
// names/non-strings/empties skipped, trim option, resolveFilePath join verbatim.

/// source: resolveHostAttentionSoundPaths() — verbatim filter chain.
pub fn resolve_host_attention_sound_paths(
    root: &str,
    sounds: &serde_json::Value,
    trim: bool,
    is_name: &dyn Fn(&str) -> bool,
) -> std::collections::HashMap<String, String> {
    let mut out = std::collections::HashMap::new();
    let obj = match sounds.as_object() {
        Some(o) => o,
        None => return out,
    };
    for (name, file) in obj {
        if !is_name(name) {
            continue;
        }
        let file = match file.as_str() {
            Some(f) => f,
            None => continue,
        };
        let value = if trim { file.trim() } else { file };
        if value.is_empty() {
            continue;
        }
        out.insert(
            name.clone(),
            crate::util::filesystem::resolve_file_path(root, value),
        );
    }
    out
}
