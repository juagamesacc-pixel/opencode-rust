// source: src/cli/cmd/debug/lsp.ts — exports: [LSPCommand, SymbolsCommand, DocumentSymbolsCommand]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/lsp/lsp"
/// - "LSP debugging utilities"
/// - "diagnostics <file>"
/// - "get diagnostics for a file"
/// - "Cli.debug.lsp.diagnostics"
/// - "symbols <query>"
/// - "search workspace symbols"
/// - "Cli.debug.lsp.symbols"
/// source: `export const LSPCommand` — shape as JSON value; CI verifies.
pub type LSPCommand = serde_json::Value;
/// source: `export const SymbolsCommand` — shape as JSON value; CI verifies.
pub type SymbolsCommand = serde_json::Value;
/// source: `export const DocumentSymbolsCommand` — shape as JSON value; CI verifies.
pub type DocumentSymbolsCommand = serde_json::Value;
