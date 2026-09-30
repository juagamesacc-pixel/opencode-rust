// source: src/lsp/client.ts — exports: Info, Diagnostic,
// InitializeError, create, LSPClient
// PROVISIONAL (650-line JSON-RPC client): timeout consts, LSP spec consts,
// InitializeError tag, getFilePath/getSyncKind rules, endPosition,
// dedupeDiagnostics key set, configurationValue section-walk,
// shouldSeedDiagnosticsOnFirstPush ("typescript") verbatim.

/// source: debounce/timeout consts — verbatim.
pub const DIAGNOSTICS_DEBOUNCE_MS: u64 = 150;
pub const DIAGNOSTICS_DOCUMENT_WAIT_TIMEOUT_MS: u64 = 5_000;
pub const DIAGNOSTICS_FULL_WAIT_TIMEOUT_MS: u64 = 10_000;
pub const DIAGNOSTICS_REQUEST_TIMEOUT_MS: u64 = 3_000;
pub const INITIALIZE_TIMEOUT_MS: u64 = 45_000;

/// source: LSP spec consts — verbatim values.
pub const FILE_CHANGE_CREATED: u32 = 1;
pub const FILE_CHANGE_CHANGED: u32 = 2;
pub const TEXT_DOCUMENT_SYNC_INCREMENTAL: u32 = 2;

/// source: InitializeError ("LSPInitializeError" { serverID, cause? }) — verbatim.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeError {
    pub server_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

/// source: getFilePath() — non-file:// → undefined. Verbatim rule.
pub fn is_file_uri(uri: &str) -> bool {
    uri.starts_with("file://")
}

/// source: endPosition() — split /\r\n|\r|\n/, last-line length. Verbatim.
pub fn end_position(text: &str) -> (usize, usize) {
    let lines: Vec<&str> = text.split('\n').collect();
    let last = lines
        .last()
        .map(|l| l.strip_suffix('\r').unwrap_or(l).len())
        .unwrap_or(0);
    (lines.len().saturating_sub(1), last)
}

/// source: dedupeDiagnostics() key set — verbatim fields.
pub const DEDUPE_KEYS: &[&str] = &["code", "severity", "message", "source", "range"];

/// source: shouldSeedDiagnosticsOnFirstPush() — serverID === "typescript". Verbatim.
pub fn seed_on_first_push(server_id: &str) -> bool {
    server_id == "typescript"
}
