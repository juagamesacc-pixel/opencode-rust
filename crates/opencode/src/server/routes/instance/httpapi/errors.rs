// source: src/server/routes/instance/httpapi/errors.ts — exports: [InvalidRequestError, UnauthorizedError, ForbiddenError, ConflictError, UpstreamError, ServiceUnavailableError, TimeoutError, UnknownError, ProviderNotFoundError, ModelNotFoundError, SessionNotFoundError, MessageNotFoundError, InvalidCursorError, SessionBusyError, QuestionNotFoundError, PermissionNotFoundError, McpServerNotFoundError, PtyNotFoundError, PtyForbiddenError, ProjectNotFoundError, ApiNotFoundError, notFound]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "InvalidRequestError"
/// - "UnauthorizedError"
/// - "ForbiddenError"
/// - "ConflictError"
/// - "UpstreamError"
/// - "ServiceUnavailableError"
/// - "TimeoutError"
/// - "UnknownError"
use serde::{Deserialize, Serialize};

/// source: `export class InvalidRequestError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvalidRequestError {
    pub value: serde_json::Value,
}
/// source: `export class UnauthorizedError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnauthorizedError {
    pub value: serde_json::Value,
}
/// source: `export class ForbiddenError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForbiddenError {
    pub value: serde_json::Value,
}
/// source: `export class ConflictError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictError {
    pub value: serde_json::Value,
}
/// source: `export class UpstreamError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamError {
    pub value: serde_json::Value,
}
/// source: `export class ServiceUnavailableError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceUnavailableError {
    pub value: serde_json::Value,
}
/// source: `export class TimeoutError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutError {
    pub value: serde_json::Value,
}
/// source: `export class UnknownError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnknownError {
    pub value: serde_json::Value,
}
/// source: `export class ProviderNotFoundError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderNotFoundError {
    pub value: serde_json::Value,
}
/// source: `export class ModelNotFoundError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelNotFoundError {
    pub value: serde_json::Value,
}
/// source: `export class SessionNotFoundError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionNotFoundError {
    pub value: serde_json::Value,
}
/// source: `export class MessageNotFoundError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageNotFoundError {
    pub value: serde_json::Value,
}
/// source: `export class InvalidCursorError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvalidCursorError {
    pub value: serde_json::Value,
}
/// source: `export class SessionBusyError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionBusyError {
    pub value: serde_json::Value,
}
/// source: `export class QuestionNotFoundError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionNotFoundError {
    pub value: serde_json::Value,
}
/// source: `export class PermissionNotFoundError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionNotFoundError {
    pub value: serde_json::Value,
}
/// source: `export class McpServerNotFoundError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerNotFoundError {
    pub value: serde_json::Value,
}
/// source: `export class PtyNotFoundError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtyNotFoundError {
    pub value: serde_json::Value,
}
/// source: `export class PtyForbiddenError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtyForbiddenError {
    pub value: serde_json::Value,
}
/// source: `export class ProjectNotFoundError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectNotFoundError {
    pub value: serde_json::Value,
}
/// source: `export class ApiNotFoundError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiNotFoundError {
    pub value: serde_json::Value,
}
/// source: `export function notFound` — stub shell; CI verifies behavior.
pub fn notFound(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
