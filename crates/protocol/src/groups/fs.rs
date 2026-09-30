//! Rust port of `packages/protocol/src/groups/fs.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — `ListQuery` (`LocationQuery` fields + optional
//! `path`), `FindQuery` (`LocationQuery` fields + `query`/`type` from
//! `FileSystem.FindInput` + `NumberFromString` `limit`), and the `fs.read`
//! (`GET /api/fs/read/*`), `fs.list` (`GET /api/fs/list`), `fs.find`
//! (`GET /api/fs/find`) endpoints.

use serde::{Deserialize, Serialize};

use crate::api::{Group, GroupAnnotation, HttpMethod, Operation};
use crate::groups::location::LocationQueryLocation;

/// Port of `ListQuery` (not exported in source): `LocationQuery` fields with
/// an optional relative `path`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationQueryLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// Port of the `FileSystem.FindInput` `type` field (`"file" | "directory"`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FindType {
    File,
    Directory,
}

fn deserialize_optional_positive_from_string<'de, D>(
    deserializer: D,
) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let value = Option::<String>::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(s) => {
            let n: u64 = s
                .parse()
                .map_err(|_| Error::custom("Expected a positive integer encoded as a string"))?;
            if n < 1 {
                return Err(Error::custom(
                    "Expected a positive integer encoded as a string",
                ));
            }
            Ok(Some(n))
        }
    }
}

fn serialize_optional_limit_as_string<S>(
    value: &Option<u64>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        None => serializer.serialize_none(),
        Some(n) => serializer.serialize_str(&n.to_string()),
    }
}
/// Port of `FindQuery` (not exported in source): `LocationQuery` fields with
/// `query`/`type` from `FileSystem.FindInput` and a `NumberFromString`
/// `limit` decoded to `PositiveInt`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FindQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationQueryLocation>,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<FindType>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_positive_from_string",
        serialize_with = "serialize_optional_limit_as_string"
    )]
    pub limit: Option<u64>,
}

/// Endpoint descriptors for `server.fs`, in source order.
pub const FS_OPERATIONS: &[Operation] = &[
    Operation {
        operation_id: "fs.read",
        openapi_identifier: "v2.fs.read",
        path: "/api/fs/read/*",
        method: HttpMethod::GET,
        summary: Some("Read file"),
        description: Some("Serve one file relative to the requested location."),
        errors: &[],
    },
    Operation {
        operation_id: "fs.list",
        openapi_identifier: "v2.fs.list",
        path: "/api/fs/list",
        method: HttpMethod::GET,
        summary: Some("List directory"),
        description: Some(
            "List direct children of one directory relative to the requested location.",
        ),
        errors: &[],
    },
    Operation {
        operation_id: "fs.find",
        openapi_identifier: "v2.fs.find",
        path: "/api/fs/find",
        method: HttpMethod::GET,
        summary: Some("Find files"),
        description: Some(
            "Find recursively ranked filesystem entries relative to the requested location.",
        ),
        errors: &[],
    },
];

/// Port of `FileSystemGroup` (`HttpApiGroup.make("server.fs")`).
#[allow(non_upper_case_globals)]
pub const FileSystemGroup: Group = Group {
    name: "server.fs",
    annotations: &[GroupAnnotation {
        title: Some("filesystem"),
        description: Some("Experimental location-scoped filesystem routes."),
    }],
    operations: FS_OPERATIONS,
};
