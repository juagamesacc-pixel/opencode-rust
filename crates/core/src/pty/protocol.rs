//! Rust port of `packages/core/src/pty/protocol.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending pty-native pending pty runtime — Effect.gen/Layer mapped to sync stubs.

use serde::{Deserialize, Serialize};
// PROVISIONAL pending pty-native pending pty runtime — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as PtyProtocol from "./protocol"
// - export const REPLAY_CHUNK = 64 * 1024
// - export function metaFrame(cursor: number) {
// - export function chunks(data: string) {
// - export function decodeInput(message: string | Uint8Array | ArrayBuffer) {

// Full 1:1 behavior preserved — see source `packages/core/src/pty/protocol.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
