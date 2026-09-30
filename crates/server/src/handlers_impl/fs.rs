//! Rust port of `packages/server/src/handlers/fs.ts` (opencode v1.18.30).
//!
//! Source 37 lines: `FileSystemHandler` with `fs.read` (raw, pathname slice 13), `fs.list`, `fs.find` (location-wrapped).
//!
//! PROVISIONAL: `FileSystem.Service`, `RelativePath` pending `crates/core`.

pub const GROUP: &str = "server.fs";
pub const OPERATIONS: &[&str] = &["fs.read", "fs.list", "fs.find"];
/// `fs.read` uses raw handler and slices 13 chars (`/api/fs/read/` prefix handling via slice 13).
pub const FS_READ_RAW: bool = true;
pub const FS_READ_PATH_SLICE: usize = 13;
pub const SERVICE_ID: &str = "@opencode/FileSystem";
