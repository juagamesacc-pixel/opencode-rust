//! Rust port of `packages/app/src/utils/solid-dnd.tsx` (opencode v1.18.30).
//!
//! Source 49 lines: `getDraggableId`, `ConstrainDragXAxis`,
//! `ConstrainDragYAxis` (+ private transformer/root wiring).
//! `@thisbeyond/solid-dnd` has no Rust equivalent — PROVISIONAL stub with
//! verbatim transformer ids.
//! Original file: `packages/app/src/utils/solid-dnd.tsx`

#![allow(dead_code)]

// PROVISIONAL: pending solid-dnd crate — mirrors `packages/app/src/utils/solid-dnd.tsx`.
/// Mirrors the `constrain-x-axis` transformer id (verbatim).
pub const CONSTRAIN_X_AXIS_ID: &str = "constrain-x-axis";

/// Mirrors the `constrain-y-axis` transformer id (verbatim).
pub const CONSTRAIN_Y_AXIS_ID: &str = "constrain-y-axis";

/// Mirrors the transformer `order` (verbatim `100`).
pub const DRAG_TRANSFORMER_ORDER: u32 = 100;

/// Mirrors `getDraggableId(event)`.
pub fn draggable_id(value: &serde_json::Value) -> Option<String> {
    value
        .get("draggable")
        .and_then(|draggable| draggable.get("id"))
        .and_then(|id| id.as_str())
        .map(str::to_string)
}

/// Mirrors the axis-constraint descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AxisConstraint {
    pub id: &'static str,
    pub axis: &'static str,
}

impl AxisConstraint {
    pub fn constrain_x() -> Self {
        Self {
            id: CONSTRAIN_X_AXIS_ID,
            axis: "x",
        }
    }

    pub fn constrain_y() -> Self {
        Self {
            id: CONSTRAIN_Y_AXIS_ID,
            axis: "y",
        }
    }
}
