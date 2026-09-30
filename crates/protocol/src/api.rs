//! Rust port of `packages/protocol/src/api.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — `make_api` + `make_default_api` as route-table
//! descriptors (operation list with exact operation IDs, `/api/…` paths, and
//! HTTP methods). Group insertion order matches `makeApiFromGroup` source
//! order exactly.
//!
//! The TS signatures take `Location`/`SessionLocation` middleware keys and an
//! event `definitions` array. Middleware placement is server-side (no runtime
//! representation here); `make_api` therefore takes the already-built event
//! group descriptor, mirroring `makeApiFromGroup(eventGroup, …)`, while
//! `make_default_api` uses the default `EventGroup`, mirroring
//! `makeDefaultApi`.

use std::fmt;

/// HTTP method of an endpoint, verbatim from source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl HttpMethod {
    /// Method string verbatim (`"GET"`, `"POST"`, `"PUT"`, `"PATCH"`, `"DELETE"`).
    pub const fn as_str(self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::DELETE => "DELETE",
        }
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Route-table descriptor for one endpoint: exact operation ID, OpenApi
/// identifier, `/api/…` path, HTTP method, summary/description, and the
/// `_tag` strings of the endpoint error channel (source order).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Operation {
    pub operation_id: &'static str,
    pub openapi_identifier: &'static str,
    pub path: &'static str,
    pub method: HttpMethod,
    pub summary: Option<&'static str>,
    pub description: Option<&'static str>,
    pub errors: &'static [&'static str],
}

/// One group-level `OpenApi.annotations` merge (title/description), in source
/// order. Groups without a group-level merge carry an empty slice.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GroupAnnotation {
    pub title: Option<&'static str>,
    pub description: Option<&'static str>,
}

/// Route-table descriptor for one `HttpApiGroup`: exact group name plus its
/// endpoints in source order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Group {
    pub name: &'static str,
    pub annotations: &'static [GroupAnnotation],
    pub operations: &'static [Operation],
}

/// The assembled `"server"` HttpApi descriptor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Api {
    pub name: &'static str,
    pub title: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub groups: Vec<Group>,
}

/// Api name verbatim from source: `HttpApi.make("server")`.
pub const API_NAME: &str = "server";

/// OpenApi title verbatim from source.
pub const API_TITLE: &str = "opencode HttpApi";

/// OpenApi version verbatim from source.
pub const API_VERSION: &str = "0.0.1";

/// OpenApi description verbatim from source.
pub const API_DESCRIPTION: &str = "Experimental HttpApi surface for selected instance routes.";

/// Group names in `makeApiFromGroup` insertion order.
pub const GROUP_ORDER: &[&str] = &[
    "server.health",
    "server.location",
    "server.agent",
    "server.session",
    "server.message",
    "server.model",
    "server.provider",
    "server.integration",
    "server.credential",
    "server.permission",
    "server.fs",
    "server.command",
    "server.skill",
    "server.event",
    "server.pty",
    "server.question",
    "server.reference",
    "server.projectCopy",
];

/// Port of `makeApi(options)`: assemble the `"server"` Api around the given
/// event group (built from the caller's `definitions`, mirroring
/// `makeEventGroup(options.definitions)`).
pub fn make_api(event_group: Group) -> Api {
    use crate::groups::agent::AgentGroup;
    use crate::groups::command::CommandGroup;
    use crate::groups::credential::CredentialGroup;
    use crate::groups::fs::FileSystemGroup;
    use crate::groups::health::HealthGroup;
    use crate::groups::integration::IntegrationGroup;
    use crate::groups::location::LocationGroup;
    use crate::groups::message::MessageGroup;
    use crate::groups::model::ModelGroup;
    use crate::groups::permission::make_permission_group;
    use crate::groups::project_copy::ProjectCopyGroup;
    use crate::groups::provider::ProviderGroup;
    use crate::groups::pty::PtyGroup;
    use crate::groups::question::make_question_group;
    use crate::groups::reference::ReferenceGroup;
    use crate::groups::session::make_session_group;
    use crate::groups::skill::SkillGroup;
    Api {
        name: API_NAME,
        title: API_TITLE,
        version: API_VERSION,
        description: API_DESCRIPTION,
        groups: vec![
            HealthGroup,
            LocationGroup,
            AgentGroup,
            make_session_group(),
            MessageGroup,
            ModelGroup,
            ProviderGroup,
            IntegrationGroup,
            CredentialGroup,
            make_permission_group(),
            FileSystemGroup,
            CommandGroup,
            SkillGroup,
            event_group,
            PtyGroup,
            make_question_group(),
            ReferenceGroup,
            ProjectCopyGroup,
        ],
    }
}

/// Port of `makeDefaultApi(options)`: assemble the `"server"` Api around the
/// default `EventGroup` (built from `EventManifest.ServerDefinitions`).
pub fn make_default_api() -> Api {
    use crate::groups::event::EventGroup;
    make_api(EventGroup)
}
