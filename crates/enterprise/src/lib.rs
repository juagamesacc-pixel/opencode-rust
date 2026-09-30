// source: packages/enterprise/src — SolidStart share service (opencode v1.18.30).
//! 1:1 port — Share/Storage logic fully ported; HTTP transport + Solid runtime as descriptors.
//! PROVISIONAL: S3/R2 HTTP transport (aws4fetch SigV4) and Solid/Hono runtimes have no Rust
//! equivalent here — ordering, names, strings, and pure logic preserved verbatim.

pub mod app;
pub mod core;
pub mod entry_client;
pub mod entry_server;
pub mod global_d;
pub mod routes;
