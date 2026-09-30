// source: src/util/html.ts — exports: escapeHtml (verbatim).
/// source: escapeHtml — verbatim replacement order (& < > " ').
pub fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
