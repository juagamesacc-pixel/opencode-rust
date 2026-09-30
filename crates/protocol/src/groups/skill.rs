//! Rust port of `packages/protocol/src/groups/skill.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — `skill.list` (`GET /api/skill`).

use crate::api::{Group, GroupAnnotation, HttpMethod, Operation};

/// Endpoint descriptors for `server.skill`, in source order.
pub const SKILL_OPERATIONS: &[Operation] = &[Operation {
    operation_id: "skill.list",
    openapi_identifier: "v2.skill.list",
    path: "/api/skill",
    method: HttpMethod::GET,
    summary: Some("List skills"),
    description: Some("Retrieve currently registered skills."),
    errors: &[],
}];

/// Port of `SkillGroup` (`HttpApiGroup.make("server.skill")`).
#[allow(non_upper_case_globals)]
pub const SkillGroup: Group = Group {
    name: "server.skill",
    annotations: &[GroupAnnotation {
        title: Some("skills"),
        description: Some("Experimental skill routes."),
    }],
    operations: SKILL_OPERATIONS,
};
