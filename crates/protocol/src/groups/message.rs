//! Rust port of `packages/protocol/src/groups/message.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — `SessionMessagesQuery` (`NumberFromString`
//! `limit` checked int `>= 1` and `<= 200`, optional `order`
//! `"asc" | "desc"`, optional opaque `cursor`) and `session.messages`
//! (`GET /api/session/:sessionID/message`, identifier
//! `v2.session.messages`).

use serde::{Deserialize, Serialize};

use crate::api::{Group, GroupAnnotation, HttpMethod, Operation};

/// Port of the `SessionMessagesQuery` `order` literals (`"asc" | "desc"`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageOrder {
    Asc,
    Desc,
}

fn deserialize_optional_limit_from_string<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let value = Option::<String>::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(s) => {
            let n: u64 = s.parse().map_err(|_| {
                Error::custom("Expected an integer between 1 and 200 encoded as a string")
            })?;
            if !(1..=200).contains(&n) {
                return Err(Error::custom(
                    "Expected an integer between 1 and 200 encoded as a string",
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

/// Port of `SessionMessagesQuery` (identifier `"SessionMessagesQuery"`).
///
/// Field descriptions verbatim from source:
/// - `limit`: "Maximum number of messages to return. When omitted, the
///   endpoint returns its default page size."
/// - `order`: "Message order for the first page. Use desc for newest first
///   or asc for oldest first."
/// - `cursor`: "Opaque pagination cursor returned as cursor.previous or
///   cursor.next in the previous response. Do not combine with order."
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionMessagesQuery {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_limit_from_string",
        serialize_with = "serialize_optional_limit_as_string"
    )]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<MessageOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Port of the `session.messages` params (`{ sessionID: Session.ID }`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionMessageParams {
    pub sessionID: schema::session_id::SessionID,
}

/// Endpoint descriptors for `server.message`, in source order.
pub const MESSAGE_OPERATIONS: &[Operation] = &[Operation {
  operation_id: "session.messages",
  openapi_identifier: "v2.session.messages",
  path: "/api/session/:sessionID/message",
  method: HttpMethod::GET,
  summary: Some("Get session messages"),
  description: Some(
    "Retrieve projected messages for a session. Items keep the requested order across pages; use cursor.next or cursor.previous to move through the ordered timeline.",
  ),
  errors: &["InvalidCursorError", "SessionNotFoundError", "UnknownError"],
}];

/// Port of `MessageGroup` (`HttpApiGroup.make("server.message")`).
#[allow(non_upper_case_globals)]
pub const MessageGroup: Group = Group {
    name: "server.message",
    annotations: &[GroupAnnotation {
        title: Some("messages"),
        description: Some("Experimental message routes."),
    }],
    operations: MESSAGE_OPERATIONS,
};
