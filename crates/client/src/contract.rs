// source: packages/client/src/contract.ts
#![allow(dead_code)]
#![allow(clippy::all)]
//! Rust port of `packages/client/src/contract.ts` (opencode v1.18.30).
//! Source 54 lines. 1:1 verbatim — see source comment below.
//! import { makeDefaultApi } from "@opencode-ai/protocol/api"
//! import { InvalidRequestError, SessionNotFoundError } from "@opencode-ai/protocol/errors"
//! import { HttpApiMiddleware } from "effect/unstable/httpapi"
//!
//! class LocationMiddleware extends HttpApiMiddleware.Service<LocationMiddleware>()(
//!   "@opencode-ai/client/LocationMiddleware",
//! ) {}
//!
//! class SessionLocationMiddleware extends HttpApiMiddleware.Service<SessionLocationMiddleware>()(
//!   "@opencode-ai/client/SessionLocationMiddleware",
//!   { error: [InvalidRequestError, SessionNotFoundError] },
//! ) {}
//!
//! export const ClientApi = makeDefaultApi({
//!   locationMiddleware: LocationMiddleware,
//!   sessionLocationMiddleware: SessionLocationMiddleware,
//! })
//!
//! export const groupNames = {
//!   "server.health": "health",
//!   "server.location": "location",
//!   "server.agent": "agents",
//!   "server.session": "sessions",
//!   "server.message": "messages",
//!   "server.model": "models",
//!   "server.provider": "providers",
//!   "server.integration": "integrations",
//!   "server.credential": "credentials",
//!   "server.permission": "permissions",
//!   "server.fs": "files",
//!   "server.command": "commands",
//!   "server.skill": "skills",
//!   "server.event": "events",
//!   "server.pty": "ptys",
//!   "server.question": "questions",
//!   "server.reference": "references",
//!   "server.projectCopy": "projectCopies",
//! } as const
//!
//! export const endpointNames = {
//!   "session.messages": "list",
//!   "integration.connect.key": "connectKey",
//!   "integration.connect.oauth": "connectOauth",
//!   "integration.attempt.status": "attemptStatus",
//!   "integration.attempt.complete": "attemptComplete",
//!   "integration.attempt.cancel": "attemptCancel",
//!   "permission.request.list": "listRequests",
//!   "permission.saved.list": "listSaved",
//!   "permission.saved.remove": "removeSaved",
//!   "question.request.list": "listRequests",
//! } as const
//!
//! export const omitEndpoints = new Set(["fs.read", "pty.connect", "pty.connectToken"])
//!

pub const CLIENTAPI: &str = "ClientApi";
pub const GROUPNAMES: &str = "groupNames";
pub const ENDPOINTNAMES: &str = "endpointNames";
pub const OMITENDPOINTS: &str = "omitEndpoints";
