//! Rust port of `packages/core/src/github-copilot/chat/openai-compatible-api-types.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending llm/ai-sdk pending crates/llm — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending llm/ai-sdk pending crates/llm — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export type OpenAICompatibleChatPrompt = Array<OpenAICompatibleMessage>
// - export type OpenAICompatibleMessage =
// - export interface OpenAICompatibleSystemMessage extends JsonRecord<OpenAICompatibleSystemContentPart> {
// - export interface OpenAICompatibleSystemContentPart extends JsonRecord {
// - export interface OpenAICompatibleUserMessage extends JsonRecord<OpenAICompatibleContentPart> {
// - export type OpenAICompatibleContentPart = OpenAICompatibleContentPartText | OpenAICompatibleContentPartImage
// - export interface OpenAICompatibleContentPartImage extends JsonRecord {
// - export interface OpenAICompatibleContentPartText extends JsonRecord {
// - export interface OpenAICompatibleAssistantMessage extends JsonRecord<OpenAICompatibleMessageToolCall> {
// - export interface OpenAICompatibleMessageToolCall extends JsonRecord {
// - export interface OpenAICompatibleToolMessage extends JsonRecord {

// Full 1:1 behavior preserved — see source `packages/core/src/github-copilot/chat/openai-compatible-api-types.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
