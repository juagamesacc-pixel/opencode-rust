// source: src/util/path.ts — exports: getFilename, getDirectory,
// getFileExtension, getFilenameTruncated, truncateMiddle
//
// UTF-16 note: TS string `length`/`slice` are UTF-16 code-unit based; indices
// below are computed over UTF-16 units and rebuilt via `from_utf16_lossy`
// (lone surrogates are the only deviation — they become U+FFFD instead of a
// raw surrogate code unit).

fn utf16(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn from_units(units: &[u16]) -> String {
    String::from_utf16_lossy(units)
}

/// source: `getFilename(path)` — strips trailing `/` + `\`, returns the last
/// path segment; falsy input -> "".
pub fn get_filename(path: Option<&str>) -> String {
    let Some(p) = path else { return String::new() };
    if p.is_empty() {
        return String::new();
    }
    let trimmed = p.trim_end_matches(|c| c == '/' || c == '\\');
    let parts: Vec<&str> = trimmed.split(|c| c == '/' || c == '\\').collect();
    parts.last().copied().unwrap_or("").to_string()
}

/// source: `getDirectory(path)` — all segments but the last, joined with `/`,
/// plus trailing `/`; falsy -> "".
pub fn get_directory(path: Option<&str>) -> String {
    let Some(p) = path else { return String::new() };
    if p.is_empty() {
        return String::new();
    }
    let trimmed = p.trim_end_matches(|c| c == '/' || c == '\\');
    let parts: Vec<&str> = trimmed.split(|c| c == '/' || c == '\\').collect();
    let joined = parts[..parts.len().saturating_sub(1)].join("/");
    format!("{joined}/")
}

/// source: `getFileExtension(path)` — last `.`-split segment; falsy -> "".
pub fn get_file_extension(path: Option<&str>) -> String {
    let Some(p) = path else { return String::new() };
    if p.is_empty() {
        return String::new();
    }
    let parts: Vec<&str> = p.split('.').collect();
    parts.last().copied().unwrap_or("").to_string()
}

/// source: `getFilenameTruncated(path, maxLength = 20)` — keeps the extension
/// and truncates the stem with a `…` ellipsis.
pub fn get_filename_truncated(path: Option<&str>, max_length: Option<usize>) -> String {
    let max_length = max_length.unwrap_or(20);
    let filename = get_filename(path);
    let units = utf16(&filename);
    if units.len() <= max_length {
        return filename;
    }
    // JS lastIndexOf(".")
    let last_dot = units.iter().rposition(|&u| u == b'.' as u16);
    let ext: Vec<u16> = match last_dot {
        Some(i) if i > 0 => units[i..].to_vec(),
        // lastDot <= 0 -> "" (dot at index 0 → no extension; no dot → none)
        _ => Vec::new(),
    };
    // available = maxLength - ext.length - 1 (-1 for ellipsis)
    let available = max_length.saturating_sub(ext.len()).saturating_sub(1);
    if available == 0 {
        // JS: filename.slice(0, maxLength - 1) + "…"
        let head: Vec<u16> = units[..max_length.saturating_sub(1)].to_vec();
        return format!("{}…", from_units(&head));
    }
    let head: Vec<u16> = units[..available.min(units.len())].to_vec();
    format!("{}…{}", from_units(&head), from_units(&ext))
}

/// source: `truncateMiddle(text, maxLength = 20)` — symmetric `…` ellipsis.
/// Note: with `maxLength = 1`, JS `slice(-0)` is `slice(0)` (whole string),
/// so the tail is the full text — mirrored.
pub fn truncate_middle(text: &str, max_length: Option<usize>) -> String {
    let max_length = max_length.unwrap_or(20);
    let units = utf16(text);
    if units.len() <= max_length {
        return text.to_string();
    }
    let available = max_length - 1; // -1 for ellipsis
    let start = available.div_ceil(2);
    let end = available / 2;
    let head: Vec<u16> = units[..start.min(units.len())].to_vec();
    let tail: Vec<u16> = if end == 0 {
        units.to_vec() // JS slice(-0) === slice(0) → whole string
    } else {
        units[units.len() - end..].to_vec()
    };
    format!("{}…{}", from_units(&head), from_units(&tail))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_filename_basic() {
        assert_eq!(get_filename(Some("a/b/c.ts")), "c.ts");
        assert_eq!(get_filename(Some("a/b/")), "b");
        assert_eq!(get_filename(Some("/")), "");
        assert_eq!(get_filename(Some("")), "");
        assert_eq!(get_filename(None), "");
        assert_eq!(get_filename(Some("a\\b\\c.ts")), "c.ts");
        assert_eq!(get_filename(Some("a/b//c//")), "c");
    }

    #[test]
    fn get_directory_basic() {
        assert_eq!(get_directory(Some("a/b/c.ts")), "a/b/");
        assert_eq!(get_directory(Some("a")), "/");
        assert_eq!(get_directory(Some("a/b/")), "a/");
        assert_eq!(get_directory(Some("")), "");
        assert_eq!(get_directory(None), "");
    }

    #[test]
    fn get_file_extension_basic() {
        assert_eq!(get_file_extension(Some("a/b/c.ts")), "ts");
        assert_eq!(get_file_extension(Some("file")), "file");
        assert_eq!(get_file_extension(Some("a.b.c")), "c");
        assert_eq!(get_file_extension(Some("file.")), "");
        assert_eq!(get_file_extension(None), "");
    }

    #[test]
    fn get_filename_truncated_basic() {
        assert_eq!(
            get_filename_truncated(Some("this-is-a-very-long-filename.ts"), Some(20)),
            "this-is-a-ver…filename.ts"
        );
        assert_eq!(
            get_filename_truncated(Some("short.ts"), Some(20)),
            "short.ts"
        );
    }

    #[test]
    fn truncate_middle_basic() {
        assert_eq!(truncate_middle("0123456789", Some(5)), "01…89");
        assert_eq!(truncate_middle("0123456789", Some(6)), "012…789");
        assert_eq!(truncate_middle("0123456789", Some(20)), "0123456789");
    }
}
