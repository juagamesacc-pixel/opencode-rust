//! Rust port of `src/main/external-url.ts` (opencode v1.18.30).
//!
//! Boundary note (§5.2 of the plan): `URL.canParse` + `new URL(...).href`
//! are reproduced by a focused WHATWG-subset parser. It covers the source's
//! whole reachable surface: scheme validation, `http:`/`https:` with
//! empty-path → `/` normalization (and scheme/host lowercasing, as WHATWG
//! mandates), `mailto:` passthrough, `file:` host rejection, percent-decoded
//! file pathnames, and the `fileURLToPath` Windows drive-letter branch.
//! A pathological non-WHATWG input could parse differently than in Bun's
//! `URL`; the source's own tests are covered exactly.
//!
//! Original file: `packages/desktop/src/main/external-url.ts`

pub fn resolve_external_url(value: &str) -> Option<String> {
    let parsed = parse_url(value)?;
    match parsed.scheme.as_str() {
        "http" | "https" => Some(serialize_hierarchical(&parsed)),
        "mailto" => Some(format!("mailto:{}", parsed.rest)),
        _ => None,
    }
}

pub fn resolve_local_file_path(value: &str) -> Option<String> {
    let parsed = parse_url(value)?;
    if parsed.scheme != "file" || !parsed.host.is_empty() {
        return None;
    }
    let decoded = percent_decode(&parsed.path)?;
    // Mirrors the `fileURLToPath` Windows drive-letter branch (`/C:/…`).
    let path = match decoded.strip_prefix('/') {
        Some(rest) if is_windows_drive_prefix(rest) => rest.to_string(),
        _ => decoded,
    };
    Some(path)
}

struct ParsedUrl {
    scheme: String,
    host: String,
    path: String,
    rest: String,
}

fn parse_url(value: &str) -> Option<ParsedUrl> {
    let colon = value.find(':')?;
    let scheme_raw = &value[..colon];
    if !is_valid_scheme(scheme_raw) {
        return None;
    }
    let scheme = scheme_raw.to_ascii_lowercase();
    let after = &value[colon + 1..];
    // Reject ASCII whitespace/control characters, as `URL.canParse` does.
    if after.bytes().any(|byte| byte <= 0x20 || byte == 0x7f) {
        return None;
    }
    if let Some(authority_and_path) = after.strip_prefix("//") {
        let end = authority_and_path
            .find(|c| c == '/' || c == '?' || c == '#')
            .unwrap_or(authority_and_path.len());
        let authority = &authority_and_path[..end];
        let path = &authority_and_path[end..];
        let host = authority
            .rsplit('@')
            .next()
            .unwrap_or("")
            .to_ascii_lowercase();
        Some(ParsedUrl {
            scheme,
            host,
            path: path.to_string(),
            rest: after.to_string(),
        })
    } else {
        Some(ParsedUrl {
            scheme,
            host: String::new(),
            path: after.to_string(),
            rest: after.to_string(),
        })
    }
}

fn is_valid_scheme(scheme: &str) -> bool {
    let mut chars = scheme.chars();
    match chars.next() {
        Some(first) if first.is_ascii_alphabetic() => (),
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.')
}

fn serialize_hierarchical(parsed: &ParsedUrl) -> String {
    let rest = &parsed.rest;
    let without_authority = rest.strip_prefix("//").unwrap_or(rest);
    let path_start = without_authority
        .find(|c| c == '/' || c == '?' || c == '#')
        .unwrap_or(without_authority.len());
    let authority = &without_authority[..path_start];
    let after_authority = &without_authority[path_start..];
    let host = authority
        .rsplit('@')
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    let userinfo = authority
        .rfind('@')
        .map(|index| &authority[..=index])
        .unwrap_or("");
    // WHATWG serializes an empty path as `/`.
    let path = if after_authority.is_empty()
        || after_authority.starts_with('?')
        || after_authority.starts_with('#')
    {
        format!("/{}", after_authority)
    } else {
        after_authority.to_string()
    };
    format!("{}://{}{}{}", parsed.scheme, userinfo, host, path)
}

fn is_windows_drive_prefix(rest: &str) -> bool {
    let bytes = rest.as_bytes();
    bytes.len() >= 2
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && rest[2..].starts_with('/')
}

fn percent_decode(input: &str) -> Option<String> {
    let bytes = input.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            if index + 2 >= bytes.len() {
                return None;
            }
            let hex = std::str::from_utf8(&bytes[index + 1..index + 3]).ok()?;
            let byte = u8::from_str_radix(hex, 16).ok()?;
            out.push(byte);
            index += 3;
        } else {
            out.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(out).ok()
}

#[cfg(test)]
mod tests {
    // Mirrors `src/main/external-url.test.ts` (`describe("external URLs")`).
    use super::*;

    #[test]
    fn opens_web_urls_externally() {
        assert_eq!(
            resolve_external_url("https://example.com/a?b=c"),
            Some("https://example.com/a?b=c".to_string())
        );
        assert_eq!(
            resolve_external_url("http://example.com"),
            Some("http://example.com/".to_string())
        );
    }

    #[test]
    fn opens_mail_links_externally() {
        assert_eq!(
            resolve_external_url("mailto:hello@opencode.ai"),
            Some("mailto:hello@opencode.ai".to_string())
        );
    }

    #[test]
    fn rejects_file_urls_and_unsupported_protocols() {
        assert_eq!(resolve_external_url("file:///tmp/index.html"), None);
        assert_eq!(resolve_external_url("javascript:alert(1)"), None);
        assert_eq!(resolve_external_url("data:text/html,hello"), None);
        assert_eq!(resolve_external_url("not a url"), None);
    }

    #[test]
    fn resolves_only_local_file_urls() {
        // Mirrors `pathToFileURL(resolve("example.html")).href` on POSIX.
        let path = "/opencode-rust/crates/desktop/example.html";
        let href = format!("file://{}", path);
        assert_eq!(resolve_local_file_path(&href), Some(path.to_string()));
        assert_eq!(
            resolve_local_file_path("file://example.com/share/index.html"),
            None
        );
        assert_eq!(
            resolve_local_file_path("https://example.com/index.html"),
            None
        );
    }
}
