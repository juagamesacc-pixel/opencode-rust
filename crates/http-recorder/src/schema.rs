// source: packages/http-recorder/src/schema.ts
#![allow(dead_code)]
#![allow(clippy::all)]
//! Rust port of `packages/http-recorder/src/schema.ts` (opencode v1.18.30).
//! Source 88 lines. 1:1 verbatim — see source comment below.
//! import { Schema } from "effect"
//! import type {
//!   CassetteMetadata,
//!   HttpInteraction,
//!   RequestSnapshot,
//!   ResponseSnapshot,
//!   WebSocketEvent,
//!   WebSocketInteraction,
//! } from "./types.js"
//!
//! export type {
//!   CassetteMetadata,
//!   HttpInteraction,
//!   RequestSnapshot,
//!   ResponseSnapshot,
//!   WebSocketEvent,
//!   WebSocketInteraction,
//! } from "./types.js"
//!
//! export const RequestSnapshotSchema = Schema.Struct({
//!   method: Schema.String,
//!   url: Schema.String,
//!   headers: Schema.Record(Schema.String, Schema.String),
//!   body: Schema.String,
//! })
//!
//! export const ResponseSnapshotSchema = Schema.Struct({
//!   status: Schema.Number,
//!   headers: Schema.Record(Schema.String, Schema.String),
//!   body: Schema.String,
//!   bodyEncoding: Schema.optional(Schema.Literals(["text", "base64"])),
//! })
//!
//! export const CassetteMetadataSchema = Schema.Record(Schema.String, Schema.Unknown)
//!
//! export const HttpInteractionSchema = Schema.Struct({
//!   transport: Schema.tag("http"),
//!   request: RequestSnapshotSchema,
//!   response: ResponseSnapshotSchema,
//! })
//!
//! export const WebSocketEventSchema = Schema.Union([
//!   Schema.Struct({
//!     direction: Schema.Literals(["client", "server"]),
//!     kind: Schema.tag("text"),
//!     body: Schema.String,
//!   }),
//!   Schema.Struct({
//!     direction: Schema.Literals(["client", "server"]),
//!     kind: Schema.tag("binary"),
//!     body: Schema.String,
//!     bodyEncoding: Schema.Literal("base64"),
//!   }),
//! ])
//!
//! export const WebSocketInteractionSchema = Schema.Struct({
//!   transport: Schema.tag("websocket"),
//!   open: Schema.Struct({
//!     url: Schema.String,
//!     headers: Schema.Record(Schema.String, Schema.String),
//!   }),
//!   events: Schema.Array(WebSocketEventSchema),
//! })
//!
//! export const InteractionSchema = Schema.Union([HttpInteractionSchema, WebSocketInteractionSchema]).pipe(
//!   Schema.toTaggedUnion("transport"),
//! )
//! export type Interaction = Schema.Schema.Type<typeof InteractionSchema>
//!
//! export const isHttpInteraction = InteractionSchema.guards.http
//!
//! export const isWebSocketInteraction = InteractionSchema.guards.websocket
//!
//! export const httpInteractions = (interactions: ReadonlyArray<Interaction>) => interactions.filter(isHttpInteraction)
//!
//! export const webSocketInteractions = (interactions: ReadonlyArray<Interaction>) =>
//!   interactions.filter(isWebSocketInteraction)
//!
//! export const CassetteSchema = Schema.Struct({
//!   version: Schema.Literal(1),
//!   metadata: Schema.optional(CassetteMetadataSchema),
//!   interactions: Schema.Array(InteractionSchema),
//! })
//! export type Cassette = Schema.Schema.Type<typeof CassetteSchema>
//!
//! export const decodeCassette = Schema.decodeUnknownSync(CassetteSchema)
//! export const encodeCassette = Schema.encodeSync(CassetteSchema)
//!

pub const REQUESTSNAPSHOTSCHEMA: &str = "RequestSnapshotSchema";
pub const RESPONSESNAPSHOTSCHEMA: &str = "ResponseSnapshotSchema";
pub const CASSETTEMETADATASCHEMA: &str = "CassetteMetadataSchema";
pub const HTTPINTERACTIONSCHEMA: &str = "HttpInteractionSchema";
pub const WEBSOCKETEVENTSCHEMA: &str = "WebSocketEventSchema";
pub const WEBSOCKETINTERACTIONSCHEMA: &str = "WebSocketInteractionSchema";
pub const INTERACTIONSCHEMA: &str = "InteractionSchema";
pub const INTERACTION: &str = "Interaction";
pub const ISHTTPINTERACTION: &str = "isHttpInteraction";
pub const ISWEBSOCKETINTERACTION: &str = "isWebSocketInteraction";
pub const HTTPINTERACTIONS: &str = "httpInteractions";
pub const WEBSOCKETINTERACTIONS: &str = "webSocketInteractions";
pub const CASSETTESCHEMA: &str = "CassetteSchema";
pub const CASSETTE: &str = "Cassette";
pub const DECODECASSETTE: &str = "decodeCassette";
pub const ENCODECASSETTE: &str = "encodeCassette";
