// source: packages/enterprise/src/core/share.ts — exports: Share namespace (Info/Data/key/merge/readSnapshot/writeSnapshot/legacy/create/get/remove/removeAdmin/sync/data/syncOld/Errors)
//! 1:1 port — key derivation, merge ordering, snapshot/compaction/event flow, and all error
//! messages preserved verbatim. Async storage is modeled on the sync `Store` trait
//! (`super::storage::Store`); secret RNG is caller-supplied (`crypto.randomUUID` PROVISIONAL).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::storage::{list_prefix, resolve, split_key, Store};

/// source: `Share.Info` — `{ id, secret, sessionID }` verbatim
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Info {
    pub id: String,
    #[serde(rename = "secret")]
    pub secret: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
}

/// source: minimal payload shapes carrying the fields `key()` reads verbatim
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionPayload {
    pub id: String,
    #[serde(default)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessagePayload {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(default)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartPayload {
    pub id: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(default)]
    pub extra: serde_json::Value,
}

/// source: `Share.Data` discriminated union on `"type"` verbatim
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Data {
    #[serde(rename = "session")]
    Session { data: SessionPayload },
    #[serde(rename = "message")]
    Message { data: MessagePayload },
    #[serde(rename = "part")]
    Part { data: PartPayload },
    #[serde(rename = "session_diff")]
    SessionDiff { data: Vec<serde_json::Value> },
    #[serde(rename = "model")]
    Model { data: Vec<serde_json::Value> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Snapshot {
    data: Vec<Data>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Compaction {
    event: Option<String>,
    data: Vec<Data>,
}

/// source: `key()` — `"session"` / `message/${id}` / `part/${messageID}/${id}` /
/// `"session_diff"` / `"model"` verbatim
pub fn key(item: &Data) -> String {
    match item {
        Data::Session { .. } => "session".to_string(),
        Data::Message { data } => format!("message/{}", data.id),
        Data::Part { data } => format!("part/{}/{}", data.message_id, data.id),
        Data::SessionDiff { .. } => "session_diff".to_string(),
        Data::Model { .. } => "model".to_string(),
    }
}

/// source: `merge()` — last-writer-wins by key, sorted by key (`localeCompare`; keys are
/// ASCII so byte order matches) verbatim
pub fn merge(lists: Vec<Vec<Data>>) -> Vec<Data> {
    let mut map: BTreeMap<String, Data> = BTreeMap::new();
    for list in lists {
        for item in list {
            map.insert(key(&item), item);
        }
    }
    map.into_values().collect()
}

fn read_snapshot(store: &dyn Store, share_id: &str) -> Result<Option<Vec<Data>>, String> {
    let raw = store.read(&resolve(&["share_snapshot", share_id]))?;
    match raw {
        None => Ok(None),
        Some(text) => serde_json::from_str::<Snapshot>(&text)
            .map(|s| Some(s.data))
            .map_err(|e| e.to_string()),
    }
}

fn write_snapshot(store: &mut dyn Store, share_id: &str, data: &[Data]) -> Result<(), String> {
    store.write(
        &resolve(&["share_snapshot", share_id]),
        serde_json::to_string(&Snapshot {
            data: data.to_vec(),
        })
        .map_err(|e| e.to_string())?,
    )
}

/// source: `Share.Errors` — messages `Share not found: {id}` / `Share secret invalid: {id}` /
/// `Share already exists: {id}` verbatim
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShareError {
    NotFound(String),
    InvalidSecret(String),
    AlreadyExists(String),
    Storage(String),
}

impl std::fmt::Display for ShareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShareError::NotFound(id) => write!(f, "Share not found: {id}"),
            ShareError::InvalidSecret(id) => write!(f, "Share secret invalid: {id}"),
            ShareError::AlreadyExists(id) => write!(f, "Share already exists: {id}"),
            ShareError::Storage(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ShareError {}

/// source: `create` — `test_` prefix rule (`NODE_ENV === "test"` or `sessionID` starts with
/// `test_`), id = last 8 of sessionID, AlreadyExists guard, writes `share` + snapshot verbatim.
/// `secret` is caller-supplied (source: `crypto.randomUUID()` — PROVISIONAL, no RNG dep here).
pub fn create(
    store: &mut dyn Store,
    session_id: &str,
    secret: &str,
    node_env_test: bool,
) -> Result<Info, ShareError> {
    let is_test = node_env_test || session_id.starts_with("test_");
    let tail: String = session_id
        .chars()
        .rev()
        .take(8)
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    let info = Info {
        id: format!("{}{}", if is_test { "test_" } else { "" }, tail),
        session_id: session_id.to_string(),
        secret: secret.to_string(),
    };
    if get(store, &info.id)?.is_some() {
        return Err(ShareError::AlreadyExists(info.id));
    }
    store
        .write(
            &resolve(&["share", &info.id]),
            serde_json::to_string(&info).map_err(|e| ShareError::Storage(e.to_string()))?,
        )
        .map_err(ShareError::Storage)?;
    write_snapshot(store, &info.id, &[]).map_err(ShareError::Storage)?;
    Ok(info)
}

/// source: `get` — read `["share", id]` verbatim
pub fn get(store: &dyn Store, id: &str) -> Result<Option<Info>, ShareError> {
    let raw = store
        .read(&resolve(&["share", id]))
        .map_err(ShareError::Storage)?;
    match raw {
        None => Ok(None),
        Some(text) => serde_json::from_str::<Info>(&text)
            .map(Some)
            .map_err(|e| ShareError::Storage(e.to_string())),
    }
}

/// source: `remove` — NotFound/InvalidSecret guards, removes `share` + every key under
/// `share_snapshot` / `share_compaction` / `share_event` / `share_data` prefixes verbatim
pub fn remove(store: &mut dyn Store, id: &str, secret: &str) -> Result<(), ShareError> {
    let share = get(store, id)?.ok_or_else(|| ShareError::NotFound(id.to_string()))?;
    if share.secret != secret {
        return Err(ShareError::InvalidSecret(id.to_string()));
    }
    store
        .remove(&resolve(&["share", id]))
        .map_err(ShareError::Storage)?;
    for group in [
        "share_snapshot",
        "share_compaction",
        "share_event",
        "share_data",
    ] {
        let keys = store
            .list(&list_prefix(&[group, id]), None, None, None)
            .map_err(ShareError::Storage)?;
        for k in keys {
            store.remove(&k).map_err(ShareError::Storage)?;
        }
    }
    Ok(())
}

/// source: `removeAdmin` — NotFound guard then `remove` with stored secret verbatim
pub fn remove_admin(store: &mut dyn Store, id: &str) -> Result<(), ShareError> {
    let share = get(store, id)?.ok_or_else(|| ShareError::NotFound(id.to_string()))?;
    remove(store, &share.id, &share.secret)
}

/// source: `legacy` — compaction read with `{ data: [], event: undefined }` default, reversed
/// event list bounded by `compaction.event`, merge, write compaction + snapshot verbatim
pub fn legacy(store: &mut dyn Store, share_id: &str) -> Result<Vec<Data>, ShareError> {
    let compaction: Compaction = match store
        .read(&resolve(&["share_compaction", share_id]))
        .map_err(ShareError::Storage)?
    {
        None => Compaction {
            event: None,
            data: vec![],
        },
        Some(text) => serde_json::from_str(&text)
            .map_err(|e: serde_json::Error| ShareError::Storage(e.to_string()))?,
    };
    let mut keys = store
        .list(
            &list_prefix(&["share_event", share_id]),
            None,
            None,
            compaction.event.as_deref(),
        )
        .map_err(ShareError::Storage)?;
    keys.reverse();
    if keys.is_empty() {
        if !compaction.data.is_empty() {
            write_snapshot(store, share_id, &compaction.data).map_err(ShareError::Storage)?;
        }
        return Ok(compaction.data);
    }
    let mut event_data: Vec<Vec<Data>> = Vec::new();
    for key in &keys {
        let raw = store.read(key).map_err(ShareError::Storage)?;
        if let Some(text) = raw {
            let items: Vec<Data> = serde_json::from_str(&text)
                .map_err(|e: serde_json::Error| ShareError::Storage(e.to_string()))?;
            event_data.push(items);
        }
    }
    let mut lists = vec![compaction.data];
    lists.extend(event_data);
    let next = merge(lists);
    let last_event = keys
        .last()
        .map(|k| split_key(k))
        .and_then(|segs| segs.last().cloned());
    store
        .write(
            &resolve(&["share_compaction", share_id]),
            serde_json::to_string(&Compaction {
                event: last_event,
                data: next.clone(),
            })
            .map_err(|e| ShareError::Storage(e.to_string()))?,
        )
        .map_err(ShareError::Storage)?;
    write_snapshot(store, share_id, &next).map_err(ShareError::Storage)?;
    Ok(next)
}

/// source: `sync` — NotFound/InvalidSecret guards, snapshot ?? legacy, merge + write verbatim
pub fn sync(
    store: &mut dyn Store,
    id: &str,
    secret: &str,
    data: Vec<Data>,
) -> Result<(), ShareError> {
    let share = get(store, id)?.ok_or_else(|| ShareError::NotFound(id.to_string()))?;
    if share.secret != secret {
        return Err(ShareError::InvalidSecret(id.to_string()));
    }
    let current = match read_snapshot(store, id).map_err(ShareError::Storage)? {
        Some(d) => d,
        None => legacy(store, id)?,
    };
    let next = merge(vec![current, data]);
    write_snapshot(store, id, &next).map_err(ShareError::Storage)?;
    Ok(())
}

/// source: `data` — `readSnapshot(shareID) ?? legacy(shareID)` verbatim
pub fn data(store: &mut dyn Store, share_id: &str) -> Result<Vec<Data>, ShareError> {
    match read_snapshot(store, share_id).map_err(ShareError::Storage)? {
        Some(d) => Ok(d),
        None => legacy(store, share_id),
    }
}

/// source: `syncOld` — per-type writes under `share_data/{id}/...` verbatim
/// (`session` / `message/{id}` / `part/{messageID}/{id}` / `session_diff` / `model`)
pub fn sync_old(
    store: &mut dyn Store,
    id: &str,
    secret: &str,
    data: Vec<Data>,
) -> Result<(), ShareError> {
    let share = get(store, id)?.ok_or_else(|| ShareError::NotFound(id.to_string()))?;
    if share.secret != secret {
        return Err(ShareError::InvalidSecret(id.to_string()));
    }
    for item in &data {
        let (path, payload): (String, serde_json::Value) = match item {
            Data::Session { data } => (
                resolve(&["share_data", id, "session"]),
                serde_json::to_value(data).map_err(|e| ShareError::Storage(e.to_string()))?,
            ),
            Data::Message { data } => (
                resolve(&["share_data", id, "message", &data.id]),
                serde_json::to_value(data).map_err(|e| ShareError::Storage(e.to_string()))?,
            ),
            Data::Part { data } => (
                resolve(&["share_data", id, "part", &data.message_id, &data.id]),
                serde_json::to_value(data).map_err(|e| ShareError::Storage(e.to_string()))?,
            ),
            Data::SessionDiff { data } => (
                resolve(&["share_data", id, "session_diff"]),
                serde_json::to_value(data).map_err(|e| ShareError::Storage(e.to_string()))?,
            ),
            Data::Model { data } => (
                resolve(&["share_data", id, "model"]),
                serde_json::to_value(data).map_err(|e| ShareError::Storage(e.to_string()))?,
            ),
        };
        store
            .write(
                &path,
                serde_json::to_string(&payload).map_err(|e| ShareError::Storage(e.to_string()))?,
            )
            .map_err(ShareError::Storage)?;
    }
    Ok(())
}
