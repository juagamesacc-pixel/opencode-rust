//! Rust port of `packages/protocol/src/groups/question.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — the `makeQuestionGroup` factory endpoint list
//! (operation IDs, `/api/…` paths, methods, OpenApi identifiers/summaries/
//! descriptions, and error channels byte-identical, in source order).
//!
//! Source note preserved: Effect applies group middleware only to endpoints
//! already added, so the first endpoint carries the location middleware while
//! the session endpoints carry the session-location middleware. Both
//! middleware keys are server-side placement with no descriptor
//! representation here. The `session.question.reply` payload IS
//! `Question.Reply` (referenced as `schema::question::Reply`, not redefined).
//!
//! Both group-level annotation merges are preserved in order (`questions`,
//! then `session questions`).

use serde::{Deserialize, Serialize};

use crate::api::{Group, GroupAnnotation, HttpMethod, Operation};

/// Port of the session question params
/// (`{ sessionID: Session.ID, requestID: Question.ID }`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionQuestionParams {
    pub sessionID: schema::session_id::SessionID,
    pub requestID: schema::question::ID,
}

/// Endpoint descriptors for `server.question`, in source order.
pub const QUESTION_OPERATIONS: &[Operation] = &[
    Operation {
        operation_id: "question.request.list",
        openapi_identifier: "v2.question.request.list",
        path: "/api/question/request",
        method: HttpMethod::GET,
        summary: Some("List pending question requests"),
        description: Some("Retrieve pending question requests for a location."),
        errors: &[],
    },
    Operation {
        operation_id: "session.question.list",
        openapi_identifier: "v2.session.question.list",
        path: "/api/session/:sessionID/question",
        method: HttpMethod::GET,
        summary: Some("List session question requests"),
        description: Some("Retrieve pending question requests owned by a session."),
        errors: &["SessionNotFoundError"],
    },
    Operation {
        operation_id: "session.question.reply",
        openapi_identifier: "v2.session.question.reply",
        path: "/api/session/:sessionID/question/:requestID/reply",
        method: HttpMethod::POST,
        summary: Some("Reply to pending question request"),
        description: Some("Answer a pending question request owned by a session."),
        errors: &["SessionNotFoundError", "QuestionNotFoundError"],
    },
    Operation {
        operation_id: "session.question.reject",
        openapi_identifier: "v2.session.question.reject",
        path: "/api/session/:sessionID/question/:requestID/reject",
        method: HttpMethod::POST,
        summary: Some("Reject pending question request"),
        description: Some("Reject a pending question request owned by a session."),
        errors: &["SessionNotFoundError", "QuestionNotFoundError"],
    },
];

/// Group name verbatim from source (`HttpApiGroup.make("server.question")`).
pub const QUESTION_GROUP_NAME: &str = "server.question";

/// Port of `makeQuestionGroup(locationMiddleware,
/// sessionLocationMiddleware)`. Middleware keys are server-side placement
/// (no descriptor representation); endpoint middleware assignment follows the
/// source note above. Returns the `server.question` group descriptor.
pub fn make_question_group() -> Group {
    Group {
        name: QUESTION_GROUP_NAME,
        annotations: &[
            GroupAnnotation {
                title: Some("questions"),
                description: Some("Experimental question routes."),
            },
            GroupAnnotation {
                title: Some("session questions"),
                description: Some("Experimental session question routes."),
            },
        ],
        operations: QUESTION_OPERATIONS,
    }
}
