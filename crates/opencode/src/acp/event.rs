// source: src/acp/event.ts — exports: start, Subscription, ACPEvent
// PROVISIONAL pending ACP sdk + sdk/v2 + ./session + ./permission +
// ./content + ./tool: event-type branches (session.status idle→idle(),
// permission.asked→handle, message.part.updated/delta), replay role gate
// (assistant|user only), start-once/stop-disconnect rules verbatim.

/// source: handle() event branches — verbatim types.
pub const EVENT_STATUS: &str = "session.status";
pub const EVENT_PERMISSION_ASKED: &str = "permission.asked";
pub const EVENT_PART_UPDATED: &str = "message.part.updated";
pub const EVENT_PART_DELTA: &str = "message.part.delta";

/// source: replayMessage() role gate — assistant|user only. Verbatim.
pub fn replayable_role(role: &str) -> bool {
    matches!(role, "assistant" | "user")
}

/// source: status idle marker — verbatim.
pub const STATUS_IDLE: &str = "idle";
