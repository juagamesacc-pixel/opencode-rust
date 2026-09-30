//! Port of `packages/schema/test/event-manifest.test.ts`.
//!
//! Case names/order mirror the source (`toBe` → `assert_eq!` on values,
//! `toEqual` → `assert_eq!`, `length`/`size` → `len()`, `has` →
//! `contains_key`). Cross-lane paths use plan §4 module names; the assembly
//! lane wires `lib.rs`. Assumed Lane B conventions (same marker pattern as
//! this lane): `Event::Definitions: &[&'static str]` per module and
//! `session_event::step::Ended::TYPE` for the step-ended marker.

use schema::event_manifest::{Definitions, Durable, Latest, ServerDefinitions};

#[test]
fn owns_the_complete_public_event_surface() {
    // FLAG: test-fix — session.next.revert.* are real current events (session-event.ts:434-446 define() + 448/479 inventory membership, durable + live). Previous 55/85/85/32 counts predated them; +3 for the Revert namespace.
    assert_eq!(ServerDefinitions.len(), 58);
    assert_eq!(Definitions.len(), 88);
    assert_eq!(
        schema::session_v1::Event::Definitions,
        &[
            schema::session_v1::Event::Created::TYPE,
            schema::session_v1::Event::Updated::TYPE,
            schema::session_v1::Event::Deleted::TYPE,
            schema::session_v1::Event::MessageUpdated::TYPE,
            schema::session_v1::Event::MessageRemoved::TYPE,
            schema::session_v1::Event::PartUpdated::TYPE,
            schema::session_v1::Event::PartRemoved::TYPE,
            schema::session_v1::Event::PartDelta::TYPE,
            schema::session_v1::Event::Diff::TYPE,
            schema::session_v1::Event::Error::TYPE,
        ]
    );
    assert_eq!(Latest.len(), 88);
    assert_eq!(Durable.len(), 35);
}

#[test]
fn uses_canonical_definitions_for_current_public_events() {
    assert_eq!(
        schema::session::Event::Definitions,
        schema::session_event::Event::Definitions
    );
    assert_eq!(
        schema::workspace::Event::Definitions,
        schema::workspace_event::Event::Definitions
    );
    assert_eq!(
        Latest
            .get("session.next.step.ended")
            .map(|definition| definition.r#type),
        Some(schema::session_event::step::Ended::TYPE)
    );
    assert_eq!(
        Latest
            .get("todo.updated")
            .map(|definition| definition.r#type),
        Some(schema::session_todo::Event::Updated::TYPE)
    );
    assert_eq!(
        Latest
            .get("project.updated")
            .map(|definition| definition.r#type),
        Some(schema::project::Event::Updated::TYPE)
    );
    assert_eq!(
        schema::project::Event::Definitions
            .iter()
            .map(|d| d.r#type)
            .collect::<Vec<_>>(),
        vec![schema::project::Event::Updated::TYPE]
    );
    assert_eq!(
        schema::filesystem::Event::Definitions,
        &[schema::filesystem::Event::Edited::TYPE]
    );
    assert_eq!(
        schema::integration::Event::Definitions,
        &[
            schema::integration::Event::Updated::TYPE,
            schema::integration::Event::ConnectionUpdated::TYPE
        ]
    );
    assert_eq!(
        schema::permission::Event::Definitions,
        &[
            schema::permission::Event::Asked::TYPE,
            schema::permission::Event::Replied::TYPE
        ]
    );
    assert_eq!(
        schema::reference::Event::Definitions,
        &[schema::reference::Event::Updated::TYPE]
    );
    assert!(!Latest.contains_key("ide.installed"));
    assert_eq!(
        schema::ide_event::Definitions,
        &[schema::ide_event::Installed::TYPE]
    );
    // FLAG: test-fix — prior 40..43 assumed foundation=40 (pre-revert). With RevertEvent (3) in SessionEvent Definitions (session-event.ts:509-511) foundation=43, so the V1 live window shifts to 43..46. The V1 trio (v1/session.ts:633,644,652) remain in full Definitions (sessionV1LiveDefinitions), not in ServerDefinitions (AGENTS.md: keep V1-only like message.part.* out of the current surface is for Server/Latest-current, not the full manifest).
    assert_eq!(
        Definitions
            .iter()
            .map(|definition| definition.r#type)
            .collect::<Vec<_>>()[43..46],
        [
            schema::session_v1::Event::PartDelta::TYPE,
            schema::session_v1::Event::Diff::TYPE,
            schema::session_v1::Event::Error::TYPE,
        ]
    );
    assert!(!Durable.contains_key("session.next.step.ended.1"));
    assert_eq!(
        Durable
            .get("session.next.step.ended.2")
            .map(|definition| definition.r#type),
        Some(schema::session_event::step::Ended::TYPE)
    );
}
