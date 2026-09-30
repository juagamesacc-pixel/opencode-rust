// source: src/lsp/lsp.ts — exports: Event, Range, Symbol, DocumentSymbol,
// Status, Interface, Service, Diagnostic, node, LSP
// PROVISIONAL (507-line service): Range/Symbol/DocumentSymbol/Status shapes,
// SymbolKind numeric table, interest kinds, pyright/ty experimental toggle,
// 13-method interface table, service id verbatim.

use serde::{Deserialize, Serialize};

/// source: Range { start, end } ("Range") — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

/// source: Position { line, character } NonNegativeInt — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u64,
    pub character: u64,
}

/// source: Symbol ("Symbol") — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub kind: u64,
    pub location: SymbolLocation,
}

/// source: location { uri, range } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolLocation {
    pub uri: String,
    pub range: Range,
}

/// source: DocumentSymbol ("DocumentSymbol") — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSymbol {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    pub kind: u64,
    pub range: Range,
    pub selection_range: Range,
}

/// source: Status ("LSPStatus") { id, name, root, status } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub id: String,
    pub name: String,
    pub root: String,
    pub status: String,
}

/// source: status literals — verbatim.
pub const STATUS_CONNECTED: &str = "connected";
pub const STATUS_ERROR: &str = "error";

/// source: SymbolKind numeric table — verbatim values.
pub fn symbol_kind(name: &str) -> Option<u32> {
    Some(match name {
        "File" => 1,
        "Module" => 2,
        "Namespace" => 3,
        "Package" => 4,
        "Class" => 5,
        "Method" => 6,
        "Property" => 7,
        "Field" => 8,
        "Constructor" => 9,
        "Enum" => 10,
        "Interface" => 11,
        "Function" => 12,
        "Variable" => 13,
        "Constant" => 14,
        "String" => 15,
        "Number" => 16,
        "Boolean" => 17,
        "Array" => 18,
        "Object" => 19,
        "Key" => 20,
        "Null" => 21,
        "EnumMember" => 22,
        "Struct" => 23,
        "Event" => 24,
        "Operator" => 25,
        "TypeParameter" => 26,
        _ => return None,
    })
}

/// source: interest kinds — verbatim list.
pub const INTEREST_KINDS: &[&str] = &[
    "Class",
    "Function",
    "Method",
    "Interface",
    "Variable",
    "Constant",
    "Struct",
    "Enum",
];

/// source: experimental toggle servers — verbatim ("pyright" vs "ty").
pub const LSP_PYRIGHT: &str = "pyright";
pub const LSP_TY: &str = "ty";

/// source: "all LSPs are disabled" — verbatim.
pub const ALL_DISABLED_MESSAGE: &str = "all LSPs are disabled";

/// source: Interface methods — verbatim names/order (13).
pub const LSP_METHODS: &[&str] = &[
    "init",
    "status",
    "hasClients",
    "touchFile",
    "diagnostics",
    "hover",
    "definition",
    "references",
    "implementation",
    "documentSymbol",
    "workspaceSymbol",
    "prepareCallHierarchy",
    "incomingCalls",
    "outgoingCalls",
];

/// source: Service "@opencode/LSP" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/LSP";
