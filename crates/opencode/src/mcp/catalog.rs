// source: src/mcp/catalog.ts — exports: paginate, defs, convertTool,
// fetch, sanitize, toolName, prompts, resources, resourceTemplates, McpCatalog
// PROVISIONAL pending MCP sdk + ai sdk: DEFAULT_TIMEOUT, MAX_LIST_PAGES,
// duplicate-cursor + page-overflow errors, sanitize regex, toolName join,
// resource key escaping (%→%25, :→%3A), `failed to get ${label}`, "MCP tool
// returned an error", outputSchema-validation fallback regex verbatim.

/// source: DEFAULT_TIMEOUT = 30_000 — verbatim.
pub const DEFAULT_TIMEOUT: u64 = 30_000;
/// source: MAX_LIST_PAGES = 1_000 — verbatim.
pub const MAX_LIST_PAGES: usize = 1_000;

/// source: `MCP list returned duplicate cursor: ${cursor}` — verbatim.
pub fn duplicate_cursor_message(cursor: &str) -> String {
    format!("MCP list returned duplicate cursor: {}", cursor)
}
/// source: `MCP list exceeded ${MAX} pages` — verbatim.
pub fn overflow_message() -> String {
    format!("MCP list exceeded {} pages", MAX_LIST_PAGES)
}

/// source: sanitize() — [^a-zA-Z0-9_-] → _. Verbatim.
pub fn sanitize(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// source: toolName() — sanitize + "_" + sanitize. Verbatim.
pub fn tool_name(client: &str, name: &str) -> String {
    format!("{}_{}", sanitize(client), sanitize(name))
}

/// source: resource key escaping — % → %25 then : → %3A. Verbatim order.
pub fn resource_client(client: &str) -> String {
    client.replace('%', "%25").replace(':', "%3A")
}

/// source: `failed to get ${label}` — verbatim.
pub fn fetch_failed_message(label: &str) -> String {
    format!("failed to get {}", label)
}

/// source: "MCP tool returned an error" — verbatim fallback.
pub const TOOL_ERROR_FALLBACK: &str = "MCP tool returned an error";

/// source: outputSchema validation-error regex — verbatim pattern text.
pub const OUTPUT_SCHEMA_ERROR_PATTERN: &str =
    "can't resolve reference|resolves to more than one schema|outputSchema|schema.*reference|reference.*schema";

/// source: inputSchema normalization — type "object", properties ?? {},
/// additionalProperties false. Verbatim rule.
pub const SCHEMA_TYPE: &str = "object";
