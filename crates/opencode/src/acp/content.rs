// source: src/acp/content.ts — exports: PromptPart, ReplayPart,
// promptContentToParts, contentBlockToParts, partsToContentChunks,
// partToContentChunks
// PROVISIONAL pending ACP sdk + core v1/session: block-type branches
// (text/image/resource_link/resource), data-URL regex, audience flags
// (assistant→synthetic, user→ignored), zed:// path, file fallbacks
// ("image"/"file"), hash #L-line prefix verbatim.

use serde::{Deserialize, Serialize};

/// source: data-URL regex ^data:([^;]+);base64,(.*)$ — verbatim rule.
pub fn decode_data_url(url: &str) -> Option<(&str, &str)> {
    let rest = url.strip_prefix("data:")?;
    let semi = rest.find(';')?;
    let (mime, after) = (&rest[..semi], &rest[semi..]);
    let base64 = after.strip_prefix(";base64,")?;
    Some((mime, base64))
}

/// source: image block — data → data-URL; data: URI; http(s) URI; else []. Verbatim.
pub fn image_url(mime: &str, data: Option<&str>, uri: Option<&str>) -> Option<(String, String)> {
    if let Some(d) = data {
        return Some((format!("data:{};base64,{}", mime, d), "image".to_string()));
    }
    if let Some(u) = uri {
        if u.starts_with("data:") || u.starts_with("http://") || u.starts_with("https://") {
            return Some((u.to_string(), "image".to_string()));
        }
    }
    None
}

/// source: audienceFlags() — [assistant]→synthetic, [user]→ignored. Verbatim.
pub fn audience_flags(audience: Option<&[&str]>) -> (bool, bool) {
    match audience {
        Some(["assistant"]) => (true, false),
        Some(["user"]) => (false, true),
        _ => (false, false),
    }
}

/// source: resource text prefix `[${filepath}${:line}]\n${text}` — verbatim.
pub fn resource_text_prefix(filepath: &str, line: Option<&str>, text: &str) -> String {
    match line {
        Some(l) => format!("[{}:{}]\n{}", filepath, l, text),
        None => format!("[{}]\n{}", filepath, text),
    }
}

/// source: hash #L(\d+) line suffix — verbatim.
pub fn hash_line(hash: &str) -> Option<&str> {
    hash.strip_prefix("#L")
        .filter(|l| !l.is_empty() && l.chars().all(|c| c.is_ascii_digit()))
}

/// source: ReplayPart tags — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ReplayPart {
    #[serde(rename = "text")]
    Text {
        text: String,
        synthetic: Option<bool>,
        ignored: Option<bool>,
    },
    #[serde(rename = "file")]
    File {
        url: String,
        mime: String,
        filename: Option<String>,
    },
    #[serde(rename = "reasoning")]
    Reasoning { text: String },
}

/// source: empty-text → [] rule (text + reasoning). Verbatim.
pub fn is_empty_text(text: &str) -> bool {
    text.is_empty()
}

/// source: file part routing — file:// → resource_link; non-data → []; image
/// → image chunk; text/json → text resource; else blob. Verbatim kinds.
pub const CHUNK_RESOURCE_LINK: &str = "resource_link";
pub const CHUNK_IMAGE: &str = "image";
pub const CHUNK_RESOURCE: &str = "resource";
pub const TEXT_MIME_PREFIX: &str = "text/";
pub const JSON_MIME: &str = "application/json";
