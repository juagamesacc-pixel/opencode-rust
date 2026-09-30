//! Rust port of `packages/protocol/src/groups/provider.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — `provider.list` (`GET /api/provider`) and
//! `provider.get` (`GET /api/provider/:providerID`).

use serde::{Deserialize, Serialize};

use crate::api::{Group, GroupAnnotation, HttpMethod, Operation};

/// Port of the `provider.get` params (`{ providerID: Provider.ID }`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProviderParams {
    pub providerID: schema::provider::ID,
}

/// Endpoint descriptors for `server.provider`, in source order.
pub const PROVIDER_OPERATIONS: &[Operation] = &[
  Operation {
    operation_id: "provider.list",
    openapi_identifier: "v2.provider.list",
    path: "/api/provider",
    method: HttpMethod::GET,
    summary: Some("List providers"),
    description: Some("Retrieve active AI providers so clients can show provider availability and configuration."),
    errors: &["ServiceUnavailableError"],
  },
  Operation {
    operation_id: "provider.get",
    openapi_identifier: "v2.provider.get",
    path: "/api/provider/:providerID",
    method: HttpMethod::GET,
    summary: Some("Get provider"),
    description: Some("Retrieve a single AI provider so clients can inspect its availability and endpoint settings."),
    errors: &["ProviderNotFoundError", "ServiceUnavailableError"],
  },
];

/// Port of `ProviderGroup` (`HttpApiGroup.make("server.provider")`).
#[allow(non_upper_case_globals)]
pub const ProviderGroup: Group = Group {
    name: "server.provider",
    annotations: &[GroupAnnotation {
        title: Some("providers"),
        description: Some("Experimental provider routes."),
    }],
    operations: PROVIDER_OPERATIONS,
};
