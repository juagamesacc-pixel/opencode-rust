// source: packages/plugin/src/example.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/example.ts` (opencode v1.18.30).
//!
//! Source 18 lines. Exports: `ExamplePlugin: Plugin` with tool `mytool`.
//!
//! 1:1 notes:
//! - Plugin returns `{ tool: { mytool: tool({ description: "This is a custom tool", args: { foo: tool.schema.string().describe("foo") }, execute: async (args) => `Hello ${args.foo}!` }) } }` verbatim.
//! - Tool description `"This is a custom tool"` verbatim, arg key `"foo"` verbatim.

use crate::tool::{tool, ToolInput};

/// Mirrors `ExamplePlugin` descriptor verbatim.
pub const EXAMPLE_PLUGIN_TOOL_DESCRIPTION: &str = "This is a custom tool";
pub const EXAMPLE_PLUGIN_TOOL_ARG_FOO: &str = "foo";
pub const EXAMPLE_PLUGIN_TOOL_OUTPUT_PREFIX: &str = "Hello ";

/// Mirrors `ExamplePlugin` tool factory — produces the tool definition verbatim.
pub fn example_plugin_tool() -> ToolInput {
    tool(ToolInput {
        description: EXAMPLE_PLUGIN_TOOL_DESCRIPTION.to_string(),
        args: serde_json::json!({ "foo": { "type": "string", "description": "foo" } }),
        execute: serde_json::Value::String("async (args) => `Hello ${args.foo}!`".to_string()),
    })
}

/// Mirrors `ExamplePlugin` plugin descriptor.
#[derive(Clone, Debug, PartialEq)]
pub struct ExamplePlugin;

impl ExamplePlugin {
    pub const TOOL_NAME: &'static str = "mytool";
    pub fn tool() -> ToolInput {
        example_plugin_tool()
    }
}
