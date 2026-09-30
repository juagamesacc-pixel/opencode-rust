//! Port of `packages/schema/test/legacy-event.test.ts`.
//!
//! Case names/order mirror the source. Verbatim `type` strings and the
//! durable filter (`aggregate == "sessionID"`, `version == 1`, length 7) are
//! asserted exactly; `Project.Event.Updated` is Lane B (plan §4).

#[test]
fn owns_all_sessionv1_definitions() {
    use schema::session_v1::Event as SessionV1Event;

    let types: Vec<&'static str> = SessionV1Event::Definitions.to_vec();
    assert_eq!(
        types,
        vec![
            "session.created",
            "session.updated",
            "session.deleted",
            "message.updated",
            "message.removed",
            "message.part.updated",
            "message.part.removed",
            "message.part.delta",
            "session.diff",
            "session.error",
        ]
    );

    let durable: Vec<(&'static str, Option<&'static str>, Option<i64>)> = vec![
        (
            SessionV1Event::Created::TYPE,
            SessionV1Event::Created::DURABLE_AGGREGATE,
            SessionV1Event::Created::DURABLE_VERSION,
        ),
        (
            SessionV1Event::Updated::TYPE,
            SessionV1Event::Updated::DURABLE_AGGREGATE,
            SessionV1Event::Updated::DURABLE_VERSION,
        ),
        (
            SessionV1Event::Deleted::TYPE,
            SessionV1Event::Deleted::DURABLE_AGGREGATE,
            SessionV1Event::Deleted::DURABLE_VERSION,
        ),
        (
            SessionV1Event::MessageUpdated::TYPE,
            SessionV1Event::MessageUpdated::DURABLE_AGGREGATE,
            SessionV1Event::MessageUpdated::DURABLE_VERSION,
        ),
        (
            SessionV1Event::MessageRemoved::TYPE,
            SessionV1Event::MessageRemoved::DURABLE_AGGREGATE,
            SessionV1Event::MessageRemoved::DURABLE_VERSION,
        ),
        (
            SessionV1Event::PartUpdated::TYPE,
            SessionV1Event::PartUpdated::DURABLE_AGGREGATE,
            SessionV1Event::PartUpdated::DURABLE_VERSION,
        ),
        (
            SessionV1Event::PartRemoved::TYPE,
            SessionV1Event::PartRemoved::DURABLE_AGGREGATE,
            SessionV1Event::PartRemoved::DURABLE_VERSION,
        ),
    ];
    assert_eq!(durable.len(), 7);
    assert!(durable
        .iter()
        .all(|(_, aggregate, _)| *aggregate == Some("sessionID")));
    assert!(durable.iter().all(|(_, _, version)| *version == Some(1)));
}

#[test]
fn owns_the_legacy_transient_public_definitions() {
    assert_eq!(
        vec![
            schema::session_v1::Event::PartDelta::TYPE,
            schema::session_v1::Event::Diff::TYPE,
            schema::session_v1::Event::Error::TYPE,
            schema::permission_v1::Event::Asked::TYPE,
            schema::permission_v1::Event::Replied::TYPE,
            schema::question_v1::Event::Asked::TYPE,
            schema::question_v1::Event::Replied::TYPE,
            schema::question_v1::Event::Rejected::TYPE,
            schema::project::Event::Updated::TYPE,
            schema::legacy_event::CommandExecuted::TYPE,
        ],
        vec![
            "message.part.delta",
            "session.diff",
            "session.error",
            "permission.asked",
            "permission.replied",
            "question.asked",
            "question.replied",
            "question.rejected",
            "project.updated",
            "command.executed",
        ]
    );
}
