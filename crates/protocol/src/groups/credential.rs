//! Rust port of `packages/protocol/src/groups/credential.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — `credential.update`
//! (`PATCH /api/credential/:credentialID`) and `credential.remove`
//! (`DELETE /api/credential/:credentialID`).

use serde::{Deserialize, Serialize};

use crate::api::{Group, HttpMethod, Operation};

/// Port of the credential params (`{ credentialID: Credential.ID }`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CredentialParams {
    pub credentialID: schema::credential::ID,
}

/// Port of the `credential.update` payload (`{ label: string }`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CredentialUpdatePayload {
    pub label: String,
}

/// Endpoint descriptors for `server.credential`, in source order.
pub const CREDENTIAL_OPERATIONS: &[Operation] = &[
    Operation {
        operation_id: "credential.update",
        openapi_identifier: "v2.credential.update",
        path: "/api/credential/:credentialID",
        method: HttpMethod::PATCH,
        summary: Some("Update credential"),
        description: Some("Update a stored credential label."),
        errors: &[],
    },
    Operation {
        operation_id: "credential.remove",
        openapi_identifier: "v2.credential.remove",
        path: "/api/credential/:credentialID",
        method: HttpMethod::DELETE,
        summary: Some("Remove credential"),
        description: Some("Remove a stored integration credential."),
        errors: &[],
    },
];

/// Port of `CredentialGroup` (`HttpApiGroup.make("server.credential")`).
#[allow(non_upper_case_globals)]
pub const CredentialGroup: Group = Group {
    name: "server.credential",
    annotations: &[],
    operations: CREDENTIAL_OPERATIONS,
};
