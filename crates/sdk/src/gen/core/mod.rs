// source: /root/opencode-rust/crates/sdk/src/gen/core/mod.rs
#![allow(dead_code)]
#![allow(clippy::all)]
pub mod auth_gen;
// 1:1 with upstream generated `*.gen.ts` basenames — camelCase kept verbatim
#[allow(non_snake_case)]
pub mod bodySerializer_gen;
pub mod params_gen;
#[allow(non_snake_case)]
pub mod pathSerializer_gen;
#[allow(non_snake_case)]
pub mod queryKeySerializer_gen;
#[allow(non_snake_case)]
pub mod serverSentEvents_gen;
pub mod types_gen;
pub mod utils_gen;
