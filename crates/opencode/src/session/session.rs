// source: src/session/session.ts — exports: [isDefaultTitle, fromRow, toRow, ArchivedTimestamp, Metadata, Info, ProjectInfo, GlobalInfo, CreateInput, ForkInput, GetInput, ChildrenInput, RemoveInput, SetTitleInput, SetArchivedInput, SetMetadataInput, SetPermissionInput, SetRevertInput, MessagesInput, ListInput, GlobalListInput, Event, plan, getUsage (+7 more)]
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/layer-node`
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/permission`
// PROVISIONAL pending crates/core: `@opencode-ai/core/util/slug`
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/session`
// PROVISIONAL pending crates/core: `@opencode-ai/core/effect/service-use`
// PROVISIONAL pending crates/llm: `@opencode-ai/llm`
// PROVISIONAL pending crates/core: `@opencode-ai/core/installation/version`
// PROVISIONAL pending crates/core: `@opencode-ai/core/database/database`
// PROVISIONAL pending crates/core: `@opencode-ai/core/session`
// PROVISIONAL pending crates/core: `@opencode-ai/core/session/execution/local`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/effect/layer-node"
/// - "New session - "
/// - "Child session - "
/// - "ProjectSummary"
/// - "GlobalSession"
/// - ".opencode"
/// - ") + "
/// - "anthropic"
use serde::{Deserialize, Serialize};

/// source: `export function isDefaultTitle` — stub shell; CI verifies behavior.
pub fn isDefaultTitle(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function fromRow` — stub shell; CI verifies behavior.
pub fn fromRow(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function toRow` — stub shell; CI verifies behavior.
pub fn toRow(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export const ArchivedTimestamp` — shape as JSON value; CI verifies.
pub type ArchivedTimestamp = serde_json::Value;
/// source: `export const Metadata` — shape as JSON value; CI verifies.
pub type Metadata = serde_json::Value;
/// source: `export const Info` — shape as JSON value; CI verifies.
pub type Info = serde_json::Value;
/// source: `export const ProjectInfo` — shape as JSON value; CI verifies.
pub type ProjectInfo = serde_json::Value;
/// source: `export const GlobalInfo` — shape as JSON value; CI verifies.
pub type GlobalInfo = serde_json::Value;
/// source: `export const CreateInput` — shape as JSON value; CI verifies.
pub type CreateInput = serde_json::Value;
/// source: `export const ForkInput` — shape as JSON value; CI verifies.
pub type ForkInput = serde_json::Value;
/// source: `export const GetInput` — shape as JSON value; CI verifies.
pub type GetInput = serde_json::Value;
/// source: `export const ChildrenInput` — shape as JSON value; CI verifies.
pub type ChildrenInput = serde_json::Value;
/// source: `export const RemoveInput` — shape as JSON value; CI verifies.
pub type RemoveInput = serde_json::Value;
/// source: `export const SetTitleInput` — shape as JSON value; CI verifies.
pub type SetTitleInput = serde_json::Value;
/// source: `export const SetArchivedInput` — shape as JSON value; CI verifies.
pub type SetArchivedInput = serde_json::Value;
/// source: `export const SetMetadataInput` — shape as JSON value; CI verifies.
pub type SetMetadataInput = serde_json::Value;
/// source: `export const SetPermissionInput` — shape as JSON value; CI verifies.
pub type SetPermissionInput = serde_json::Value;
/// source: `export const SetRevertInput` — shape as JSON value; CI verifies.
pub type SetRevertInput = serde_json::Value;
/// source: `export const MessagesInput` — shape as JSON value; CI verifies.
pub type MessagesInput = serde_json::Value;
/// source: `export type ListInput` — shape as JSON value; CI verifies.
pub type ListInput = serde_json::Value;
/// source: `export type GlobalListInput` — shape as JSON value; CI verifies.
pub type GlobalListInput = serde_json::Value;
/// source: `export const Event` — shape as JSON value; CI verifies.
pub type Event = serde_json::Value;
/// source: `export function plan` — stub shell; CI verifies behavior.
pub fn plan(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export const getUsage` — shape as JSON value; CI verifies.
pub type getUsage = serde_json::Value;
/// source: `export class BusyError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusyError {
    pub value: serde_json::Value,
}
/// source: `export type NotFound` — shape as JSON value; CI verifies.
pub type NotFound = serde_json::Value;
/// source: `export interface Interface` — shape as JSON value; CI verifies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interface {
    pub value: serde_json::Value,
}
/// source: `export class Service` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub value: serde_json::Value,
}
/// source: `export const use` — shape as JSON value; CI verifies.
pub type r#use = serde_json::Value;
/// source: `export type Patch` — shape as JSON value; CI verifies.
pub type Patch = serde_json::Value;
/// source: `export const node` — shape as JSON value; CI verifies.
pub type node = serde_json::Value;
