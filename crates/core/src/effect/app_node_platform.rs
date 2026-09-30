//! Rust port of `packages/core/src/effect/app-node-platform.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

// PROVISIONAL pending @effect/platform-node NodeFileSystem/NodePath, @opencode-ai/llm/route LLMClient/RequestExecutor, FetchHttpClient

use serde::{Deserialize, Serialize};

pub const FILESYSTEM_SERVICE: &str = "FileSystem.FileSystem";
pub const PATH_SERVICE: &str = "Path.Path";
pub const HTTP_CLIENT_SERVICE: &str = "HttpClient.HttpClient";
pub const REQUEST_EXECUTOR_SERVICE: &str = "RequestExecutor.Service";
pub const LLM_CLIENT_SERVICE: &str = "LLMClient.Service";

pub const FILESYSTEM_DEPS: &[&str] = &[];
pub const PATH_DEPS: &[&str] = &[];
pub const HTTP_CLIENT_DEPS: &[&str] = &[];
pub const REQUEST_EXECUTOR_DEPS: &[&str] = &["HttpClient.HttpClient"];
pub const LLM_CLIENT_DEPS: &[&str] = &["RequestExecutor.Service"];
