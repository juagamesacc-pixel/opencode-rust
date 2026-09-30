//! Rust port of `packages/protocol/src/groups/location.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — `LocationQuery` shape (`location` with optional
//! `directory`/`workspace`), the `locationQueryOpenApi` deepObject/explode
//! transform descriptor, and the `location.get` endpoint
//! (`GET /api/location`, identifier `v2.location.get`).

use serde::{Deserialize, Serialize};

use crate::api::{Group, HttpMethod, Operation};

/// The nested `location` query object. Note this is NOT `Location.Ref`
/// (`directory` is optional here).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LocationQueryLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}

/// Port of `LocationQuery` (identifier `"LocationQuery"`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LocationQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationQueryLocation>,
}

/// Query parameter name targeted by `locationQueryOpenApi`.
pub const LOCATION_QUERY_PARAM_NAME: &str = "location";

/// Port of `locationQueryOpenApi`: the `location` query parameter is encoded
/// with OpenAPI `style: "deepObject"`, `explode: true`.
pub const LOCATION_QUERY_PARAM_STYLE: &str = "deepObject";

/// Port of `locationQueryOpenApi`: `explode: true` for the `location` query
/// parameter.
pub const LOCATION_QUERY_PARAM_EXPLODE: bool = true;

/// Port of `locationQueryOpenApi` as a descriptor: maps the `location`
/// query parameter (`in: "query"`) to deepObject/explode encoding.
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct locationQueryOpenApi;

impl locationQueryOpenApi {
    /// The query parameter name this transform applies to (`"location"`).
    pub const fn param_name() -> &'static str {
        LOCATION_QUERY_PARAM_NAME
    }

    /// The OpenAPI style applied (`"deepObject"`).
    pub const fn param_style() -> &'static str {
        LOCATION_QUERY_PARAM_STYLE
    }

    /// Whether explode is applied (`true`).
    pub const fn param_explode() -> bool {
        LOCATION_QUERY_PARAM_EXPLODE
    }
}

/// Endpoint descriptors for `server.location`, in source order.
pub const LOCATION_OPERATIONS: &[Operation] = &[Operation {
    operation_id: "location.get",
    openapi_identifier: "v2.location.get",
    path: "/api/location",
    method: HttpMethod::GET,
    summary: Some("Get location"),
    description: Some("Resolve the requested location or the server default location."),
    errors: &[],
}];

/// Port of `LocationGroup` (`HttpApiGroup.make("server.location")`).
#[allow(non_upper_case_globals)]
pub const LocationGroup: Group = Group {
    name: "server.location",
    annotations: &[],
    operations: LOCATION_OPERATIONS,
};
