// source: core/src/util/env.cloudflare.ts
//! 1:1 port of `env.cloudflare.ts`, which exists only to attach the
//! `cloudflare:workers` module type declaration (`export {}`).
//! Source pin: v1.18.30 @3104c14.

/// The ambient augmentation this module contributes is type-only in the source;
/// the runtime surface is the bare module, reproduced here as an empty marker so
/// the file still has a Rust counterpart.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EnvCloudflare;
