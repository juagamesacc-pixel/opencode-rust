// source: src/mcp/browser.ts — exports: Interface, Service, node, McpBrowser
// (open() 500ms success-grace, error/exit-code branches verbatim).

/// source: success grace 500ms — verbatim.
pub const OPEN_GRACE_MS: u64 = 500;

/// source: `Browser open failed with exit code ${code}` — verbatim.
pub fn exit_message(code: i32) -> String {
    format!("Browser open failed with exit code {}", code)
}

/// source: Service "@opencode/McpBrowser" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/McpBrowser";

/// source: node deps [] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[];
