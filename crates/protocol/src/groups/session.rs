//! Rust port of `packages/protocol/src/groups/session.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — `SessionsCursor` brand (base64url of the JSON
//! cursor union, failure string `"Invalid cursor"`), `SessionHistoryQuery`,
//! `SessionsQuery`, and the full `makeSessionGroup` factory endpoint list
//! (operation IDs, `/api/…` paths, methods, OpenApi identifiers/summaries/
//! descriptions, and error channels byte-identical, in source order).
//!
//! Base64url is hand-rolled (padded output never emitted; `serde 1` +
//! `serde_json 1` + local `schema` dep only). `SessionsCursorInput` mirrors
//! the source `SessionsCursorInput` union
//! (`withCursor(SessionsDirectoryQuery | SessionsProjectQuery | SessionsAllQuery)`:
//! shared fields plus scope fields, `limit` omitted, `anchor` appended last).
//! The `anchor` leaf is the schema-owned `Session.ListAnchor`
//! (`schema::session::ListAnchor`); it is never redefined here.

use serde::{Deserialize, Serialize};

use crate::api::{Group, GroupAnnotation, HttpMethod, Operation};

/// Failure string verbatim from source (`const invalidCursor = "Invalid cursor"`).
const INVALID_CURSOR: &str = "Invalid cursor";

/// Port of the cursor `order` literals (`"asc" | "desc"`).
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SessionOrder {
    Asc,
    Desc,
}

/// Directory-scoped cursor input (`withCursor(SessionsDirectoryQuery)`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionsCursorDirectoryInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<schema::workspace_id::WorkspaceID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<SessionOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    pub directory: schema::schema_primitives::AbsolutePath,
    pub anchor: schema::session::ListAnchor,
}

/// Project-scoped cursor input (`withCursor(SessionsProjectQuery)`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionsCursorProjectInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<schema::workspace_id::WorkspaceID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<SessionOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    pub project: schema::project_id::ProjectID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subpath: Option<schema::schema_primitives::RelativePath>,
    pub anchor: schema::session::ListAnchor,
}

/// Unscoped cursor input (`withCursor(SessionsAllQuery)`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionsCursorAllInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<schema::workspace_id::WorkspaceID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<SessionOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    pub anchor: schema::session::ListAnchor,
}

/// Port of `SessionsCursorInput` (union order follows source).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum SessionsCursorInput {
    Directory(SessionsCursorDirectoryInput),
    Project(SessionsCursorProjectInput),
    All(SessionsCursorAllInput),
}

const BASE64URL_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

fn base64url_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    let mut i = 0;
    while i < bytes.len() {
        let b0 = bytes[i] as u32;
        let b1 = if i + 1 < bytes.len() {
            bytes[i + 1] as u32
        } else {
            0
        };
        let b2 = if i + 2 < bytes.len() {
            bytes[i + 2] as u32
        } else {
            0
        };
        let remaining = bytes.len() - i;
        out.push(BASE64URL_ALPHABET[((b0 >> 2) & 63) as usize] as char);
        out.push(BASE64URL_ALPHABET[(((b0 << 4) | (b1 >> 4)) & 63) as usize] as char);
        if remaining > 1 {
            out.push(BASE64URL_ALPHABET[(((b1 << 2) | (b2 >> 6)) & 63) as usize] as char);
        }
        if remaining > 2 {
            out.push(BASE64URL_ALPHABET[(b2 & 63) as usize] as char);
        }
        i += 3;
    }
    out
}

fn base64url_value(byte: u8) -> Option<u32> {
    match byte {
        b'A'..=b'Z' => Some((byte - b'A') as u32),
        b'a'..=b'z' => Some((byte - b'a' + 26) as u32),
        b'0'..=b'9' => Some((byte - b'0' + 52) as u32),
        b'-' => Some(62),
        b'_' => Some(63),
        _ => None,
    }
}

fn base64url_decode(input: &str) -> Option<Vec<u8>> {
    let bytes = input.as_bytes();
    if bytes.len() % 4 == 1 {
        return None;
    }
    let mut out = Vec::with_capacity(bytes.len() / 4 * 3 + 3);
    let (chunks, rem) = bytes.as_chunks::<4>();
    for chunk in chunks {
        let c0 = base64url_value(chunk[0])?;
        let c1 = base64url_value(chunk[1])?;
        let c2 = base64url_value(chunk[2])?;
        let c3 = base64url_value(chunk[3])?;
        out.push(((c0 << 2) | (c1 >> 4)) as u8);
        out.push((((c1 & 0xF) << 4) | (c2 >> 2)) as u8);
        out.push((((c2 & 0x3) << 6) | c3) as u8);
    }
    if rem.len() == 2 {
        let c0 = base64url_value(rem[0])?;
        let c1 = base64url_value(rem[1])?;
        out.push(((c0 << 2) | (c1 >> 4)) as u8);
    } else if rem.len() == 3 {
        let c0 = base64url_value(rem[0])?;
        let c1 = base64url_value(rem[1])?;
        let c2 = base64url_value(rem[2])?;
        out.push(((c0 << 2) | (c1 >> 4)) as u8);
        out.push((((c1 & 0xF) << 4) | (c2 >> 2)) as u8);
    }
    Some(out)
}

/// Port of `SessionsCursor` (brand `"SessionsCursor"` over `string`; wire
/// format stays a bare string).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(transparent)]
pub struct SessionsCursor(pub String);

impl SessionsCursor {
    /// Port of `SessionsCursor.make`: JSON-encode the cursor input, then
    /// base64url-encode it (mirrors `Encoding.encodeBase64Url`).
    pub fn make(input: &SessionsCursorInput) -> SessionsCursor {
        let json = serde_json::to_vec(input).expect("SessionsCursor input serializes to JSON");
        SessionsCursor(base64url_encode(&json))
    }

    /// Port of `SessionsCursor.parse`: base64url-decode, then JSON-decode.
    /// Any failure yields the verbatim `"Invalid cursor"` string (mirrors the
    /// `Effect.fail(invalidCursor)` paths for undecodable base64 and for
    /// schema-decode errors).
    pub fn parse(input: &str) -> Result<SessionsCursorInput, String> {
        let bytes = base64url_decode(input).ok_or_else(|| INVALID_CURSOR.to_string())?;
        serde_json::from_slice(&bytes).map_err(|_| INVALID_CURSOR.to_string())
    }

    /// The cursor string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SessionsCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn deserialize_optional_positive_capped_from_string<'de, D>(
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
            let n: u64 = s.parse().map_err(|_| {
                Error::custom("Expected a positive integer at most 100 encoded as a string")
            })?;
            if !(1..=100).contains(&n) {
                return Err(Error::custom(
                    "Expected a positive integer at most 100 encoded as a string",
                ));
            }
            Ok(Some(n))
        }
    }
}

fn deserialize_optional_non_negative_from_string<'de, D>(
    deserializer: D,
) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let value = Option::<String>::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(s) => s
            .parse::<u64>()
            .map(Some)
            .map_err(|_| Error::custom("Expected a non-negative integer encoded as a string")),
    }
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

fn serialize_optional_number_as_string<S>(
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

/// Port of `SessionHistoryQuery`: `limit` is `NumberFromString` decoded to
/// `PositiveInt` capped at 100; `after` is `NumberFromString` decoded to
/// `NonNegativeInt`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionHistoryQuery {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_positive_capped_from_string",
        serialize_with = "serialize_optional_number_as_string"
    )]
    pub limit: Option<u64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_non_negative_from_string",
        serialize_with = "serialize_optional_number_as_string"
    )]
    pub after: Option<u64>,
}

/// Port of `SessionsQuery` (identifier `"SessionsQuery"`).
///
/// `limit` description verbatim: "Maximum number of sessions to return.
/// Defaults to the newest 50 sessions." `order` description verbatim:
/// "Session order for the first page. Use desc for newest first or asc for
/// oldest first." `cursor` description verbatim: "Opaque pagination cursor
/// returned as cursor.previous or cursor.next in the previous response."
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<schema::workspace_id::WorkspaceID>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_positive_from_string",
        serialize_with = "serialize_optional_number_as_string"
    )]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<SessionOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<schema::schema_primitives::AbsolutePath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<schema::project_id::ProjectID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subpath: Option<schema::schema_primitives::RelativePath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<SessionsCursor>,
}

/// Port of the `session.events` query (`after: NumberFromString` decoded to
/// `NonNegativeInt`, optional).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionEventsQuery {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_non_negative_from_string",
        serialize_with = "serialize_optional_number_as_string"
    )]
    pub after: Option<u64>,
}

/// Port of the session params (`{ sessionID: Session.ID }`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionParams {
    pub sessionID: schema::session_id::SessionID,
}

/// Port of the `session.message` params
/// (`{ sessionID: Session.ID, messageID: SessionMessage.ID }`).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionMessageParams {
    pub sessionID: schema::session_id::SessionID,
    pub messageID: schema::session_message::ID,
}

/// Port of the `session.create` payload.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionCreatePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<schema::session_id::SessionID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<schema::agent::ID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<schema::model::Ref>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<schema::location::Ref>,
}

/// Port of the `session.switchAgent` payload.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionSwitchAgentPayload {
    pub agent: schema::agent::ID,
}

/// Port of the `session.switchModel` payload.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionSwitchModelPayload {
    pub model: schema::model::Ref,
}

/// Port of the `session.prompt` payload.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionPromptPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<schema::session_message::ID>,
    pub prompt: schema::prompt_input::Prompt,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<schema::session_input::Delivery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume: Option<bool>,
}

/// Port of the `session.revert.stage` payload.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionRevertStagePayload {
    pub messageID: schema::session_message::ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<bool>,
}

/// Endpoint descriptors for `server.session`, in source order.
pub const SESSION_OPERATIONS: &[Operation] = &[
  Operation {
    operation_id: "session.list",
    openapi_identifier: "v2.session.list",
    path: "/api/session",
    method: HttpMethod::GET,
    summary: Some("List sessions"),
    description: Some(
      "Retrieve sessions in the requested order. Items keep that order across pages; use cursor.next or cursor.previous to move through the ordered list.",
    ),
    errors: &["InvalidCursorError", "InvalidRequestError"],
  },
  Operation {
    operation_id: "session.create",
    openapi_identifier: "v2.session.create",
    path: "/api/session",
    method: HttpMethod::POST,
    summary: Some("Create session"),
    description: Some("Create a session at the requested location."),
    errors: &[],
  },
  Operation {
    operation_id: "session.active",
    openapi_identifier: "v2.session.active",
    path: "/api/session/active",
    method: HttpMethod::GET,
    summary: Some("List active sessions"),
    description: Some(
      "Retrieve foreground Session drains currently owned by this OpenCode process. Sessions absent from the result are inactive.",
    ),
    errors: &[],
  },
  Operation {
    operation_id: "session.get",
    openapi_identifier: "v2.session.get",
    path: "/api/session/:sessionID",
    method: HttpMethod::GET,
    summary: Some("Get session"),
    description: Some("Retrieve a session by ID."),
    errors: &["SessionNotFoundError"],
  },
  Operation {
    operation_id: "session.switchAgent",
    openapi_identifier: "v2.session.switchAgent",
    path: "/api/session/:sessionID/agent",
    method: HttpMethod::POST,
    summary: Some("Switch session agent"),
    description: Some("Switch the agent used by subsequent provider turns."),
    errors: &["SessionNotFoundError"],
  },
  Operation {
    operation_id: "session.switchModel",
    openapi_identifier: "v2.session.switchModel",
    path: "/api/session/:sessionID/model",
    method: HttpMethod::POST,
    summary: Some("Switch session model"),
    description: Some("Switch the model used by subsequent provider turns."),
    errors: &["SessionNotFoundError"],
  },
  Operation {
    operation_id: "session.prompt",
    openapi_identifier: "v2.session.prompt",
    path: "/api/session/:sessionID/prompt",
    method: HttpMethod::POST,
    summary: Some("Send message"),
    description: Some("Durably admit one session input and schedule agent-loop execution unless resume is false."),
    errors: &["ConflictError", "SessionNotFoundError"],
  },
  Operation {
    operation_id: "session.compact",
    openapi_identifier: "v2.session.compact",
    path: "/api/session/:sessionID/compact",
    method: HttpMethod::POST,
    summary: Some("Compact session"),
    description: Some("Compact a session conversation."),
    errors: &["SessionNotFoundError", "ServiceUnavailableError"],
  },
  Operation {
    operation_id: "session.wait",
    openapi_identifier: "v2.session.wait",
    path: "/api/session/:sessionID/wait",
    method: HttpMethod::POST,
    summary: Some("Wait for session"),
    description: Some("Wait for a session agent loop to become idle."),
    errors: &["SessionNotFoundError", "ServiceUnavailableError"],
  },
  Operation {
    operation_id: "session.revert.stage",
    openapi_identifier: "v2.session.revert.stage",
    path: "/api/session/:sessionID/revert/stage",
    method: HttpMethod::POST,
    summary: Some("Stage session revert"),
    description: Some("Stage or move a reversible session boundary and optionally apply its file changes."),
    errors: &["MessageNotFoundError", "SessionNotFoundError", "UnknownError"],
  },
  Operation {
    operation_id: "session.revert.clear",
    openapi_identifier: "v2.session.revert.clear",
    path: "/api/session/:sessionID/revert/clear",
    method: HttpMethod::POST,
    summary: Some("Clear staged revert"),
    description: None,
    errors: &["SessionNotFoundError", "UnknownError"],
  },
  Operation {
    operation_id: "session.revert.commit",
    openapi_identifier: "v2.session.revert.commit",
    path: "/api/session/:sessionID/revert/commit",
    method: HttpMethod::POST,
    summary: Some("Commit staged revert"),
    description: None,
    errors: &["SessionNotFoundError"],
  },
  Operation {
    operation_id: "session.context",
    openapi_identifier: "v2.session.context",
    path: "/api/session/:sessionID/context",
    method: HttpMethod::GET,
    summary: Some("Get session context"),
    description: Some("Retrieve the active context messages for a session (all messages after the last compaction)."),
    errors: &["SessionNotFoundError", "UnknownError"],
  },
  Operation {
    operation_id: "session.history",
    openapi_identifier: "v2.session.history",
    path: "/api/session/:sessionID/history",
    method: HttpMethod::GET,
    summary: Some("Get session history"),
    description: Some(
      "Read one finite page of public durable Session events after an exclusive aggregate sequence. Newly committed events may appear on later pages.",
    ),
    errors: &["SessionNotFoundError"],
  },
  Operation {
    operation_id: "session.events",
    openapi_identifier: "v2.session.events",
    path: "/api/session/:sessionID/event",
    method: HttpMethod::GET,
    summary: Some("Subscribe to session events"),
    description: Some("Replay durable events after an aggregate sequence, then continue with new durable events."),
    errors: &["SessionNotFoundError"],
  },
  Operation {
    operation_id: "session.interrupt",
    openapi_identifier: "v2.session.interrupt",
    path: "/api/session/:sessionID/interrupt",
    method: HttpMethod::POST,
    summary: Some("Interrupt session execution"),
    description: Some("Interrupt active execution owned by this OpenCode process. Idle interruption is a no-op."),
    errors: &["SessionNotFoundError"],
  },
  Operation {
    operation_id: "session.message",
    openapi_identifier: "v2.session.message",
    path: "/api/session/:sessionID/message/:messageID",
    method: HttpMethod::GET,
    summary: Some("Get session message"),
    description: Some("Retrieve one projected message owned by the Session."),
    errors: &["SessionNotFoundError", "MessageNotFoundError"],
  },
];

/// Group name verbatim from source (`HttpApiGroup.make("server.session")`).
pub const SESSION_GROUP_NAME: &str = "server.session";

/// Port of `makeSessionGroup(sessionLocationMiddleware)`.
///
/// The `sessionLocationMiddleware` key is server-side placement (no
/// descriptor representation); every endpoint below carries it in source.
/// Returns the `server.session` group descriptor with endpoints in source
/// order.
pub fn make_session_group() -> Group {
    Group {
        name: SESSION_GROUP_NAME,
        annotations: &[GroupAnnotation {
            title: Some("sessions"),
            description: Some("Experimental session routes."),
        }],
        operations: SESSION_OPERATIONS,
    }
}
