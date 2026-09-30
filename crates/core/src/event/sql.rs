//! Rust port of `packages/core/src/event/sql.ts` (+ EventSequence/append persistence from
//! `packages/core/src/event.ts` lines 221-560, verbatim).
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

use rusqlite::Connection;
use serde::{Deserialize, Serialize};

/// Source: `export const EventSequenceTable = sqliteTable("event_sequence", {...})` verbatim
pub const EVENT_SEQUENCE_TABLE: &str = "event_sequence";
pub const EVENT_TABLE: &str = "event";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSequenceRow {
    pub aggregate_id: String,
    pub seq: i64,
    pub owner_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRow {
    pub id: String,
    pub aggregate_id: String,
    pub seq: i64,
    pub r#type: String,
    pub data: serde_json::Value,
}

/// Source: uniqueIndex "event_aggregate_seq_idx" on (aggregate_id, seq) verbatim
pub const UNIQUE_INDEX_AGG_SEQ: &str = "event_aggregate_seq_idx";
/// Source: index "event_aggregate_type_seq_idx" on (aggregate_id, type, seq) verbatim
pub const INDEX_AGG_TYPE_SEQ: &str = "event_aggregate_type_seq_idx";

/// Source: `references(() => EventSequenceTable.aggregate_id, { onDelete: "cascade" })` verbatim
pub const FK_ON_DELETE: &str = "cascade";

/// `EventV2.latestSequence` — `row?.seq ?? -1`.
pub fn latest_sequence(conn: &Connection, aggregate_id: &str) -> Result<i64, String> {
    Ok(get_sequence(conn, aggregate_id)?
        .map(|row| row.seq)
        .unwrap_or(-1))
}

/// Select the sequence row for an aggregate (undefined when missing).
pub fn get_sequence(
    conn: &Connection,
    aggregate_id: &str,
) -> Result<Option<EventSequenceRow>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT aggregate_id, seq, owner_id FROM \"event_sequence\" WHERE aggregate_id = ?1",
        )
        .map_err(|e| e.to_string())?;
    let mut rows = stmt
        .query_map([aggregate_id], |row| {
            Ok(EventSequenceRow {
                aggregate_id: row.get(0)?,
                seq: row.get(1)?,
                owner_id: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;
    match rows.next() {
        Some(r) => Ok(Some(r.map_err(|e| e.to_string())?)),
        None => Ok(None),
    }
}

/// Source message constants (verbatim error strings).
pub const ERROR_REPLAY_OWNER_MISMATCH: &str = "Replay owner mismatch for aggregate ";
pub const ERROR_REPLAY_DIVERGED: &str = "Replay diverged at aggregate ";
pub const ERROR_SEQUENCE_MISMATCH: &str = "Sequence mismatch for aggregate ";
pub const ERROR_EVENT_EXISTS: &str = " already exists at aggregate ";
pub const ERROR_REPLAY_ALL_AGGREGATE: &str = "Replay events must belong to the same aggregate";
pub const ERROR_UNKNOWN_DURABLE_TYPE: &str = "Unknown durable event type ";

/// Source `SerializedEvent` shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedEvent {
    pub id: String,
    pub r#type: String,
    pub seq: i64,
    pub aggregate_id: String,
    pub data: serde_json::Value,
}

/// Outcome of `append` — mirrors the source `commitDurableEvent` branches.
#[derive(Debug, Clone, PartialEq)]
pub enum AppendOutcome {
    /// Event row + sequence committed. `PublishOptions.commit` hooks ran before this.
    Committed { aggregate_id: String, seq: i64 },
    /// Exact replay of an already-committed event (no mutation, optional owner claim).
    Idempotent,
    /// Replay from a different owner was silently skipped.
    Skipped,
}

/// Append input — combines the source publish (`PublishOptions`) and replay (`options`) params.
#[derive(Debug, Clone)]
pub struct AppendInput<'a> {
    pub id: &'a str,
    pub aggregate_id: &'a str,
    /// Already versioned type (`versionedType(definition.type, durable.version)`) — source passes it as `event.type`.
    pub event_type: &'a str,
    pub data: &'a serde_json::Value,
    /// Replay supplies an explicit seq; publish leaves it unset (`latest + 1`).
    pub seq: Option<i64>,
    pub owner_id: Option<&'a str>,
    pub strict_owner: bool,
}

fn replay_owner_mismatch(aggregate_id: &str, expected: &str, owner_id: Option<&str>) -> String {
    format!(
        "Replay owner mismatch for aggregate {}: expected {}, got {}",
        aggregate_id,
        expected,
        owner_id.unwrap_or("none")
    )
}

/// Port of the durability transaction in `commitDurableEvent` (event.ts lines 239-350) —
/// the persistence core (projectors/commit hooks/pubsub are the service layer, out of scope here).
pub fn append(conn: &mut Connection, input: &AppendInput<'_>) -> Result<AppendOutcome, String> {
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    let row = {
        let mut stmt = tx
            .prepare("SELECT seq, owner_id FROM \"event_sequence\" WHERE aggregate_id = ?1")
            .map_err(|e| e.to_string())?;
        let mut rows = stmt
            .query_map([input.aggregate_id], |r| {
                Ok((r.get::<_, i64>(0)?, r.get::<_, Option<String>>(1)?))
            })
            .map_err(|e| e.to_string())?;
        match rows.next() {
            Some(r) => Some(r.map_err(|e| e.to_string())?),
            None => None,
        }
    };
    let latest = row.as_ref().map(|(seq, _)| *seq).unwrap_or(-1);
    let row_owner = row.as_ref().and_then(|(_, owner)| owner.as_deref());

    if input.strict_owner && row_owner.is_some() && row_owner != Some(input.owner_id.unwrap_or(""))
    {
        return Err(replay_owner_mismatch(
            input.aggregate_id,
            row_owner.unwrap_or(""),
            input.owner_id,
        ));
    }
    if let Some(requested) = input.seq {
        if requested <= latest {
            // Idempotent replay: stored event identical → claim owner if unowned, then return.
            let stored = select_event_at(&tx, input.aggregate_id, requested)?.ok_or_else(|| {
                format!(
                    "Replay diverged at aggregate {} sequence {}",
                    input.aggregate_id, requested
                )
            })?;
            if stored.id == input.id
                && stored.r#type == input.event_type
                && stored.data == *input.data
            {
                if input.owner_id.is_some() && row_owner.is_none() {
                    tx.execute(
                        "UPDATE \"event_sequence\" SET owner_id = ?1 WHERE aggregate_id = ?2",
                        rusqlite::params![input.owner_id, input.aggregate_id],
                    )
                    .map_err(|e| e.to_string())?;
                }
                tx.commit().map_err(|e| e.to_string())?;
                return Ok(AppendOutcome::Idempotent);
            }
            return Err(format!(
                "Replay diverged at aggregate {} sequence {}",
                input.aggregate_id, requested
            ));
        }
        if row_owner.is_some() && row_owner != Some(input.owner_id.unwrap_or("")) {
            tx.commit().map_err(|e| e.to_string())?;
            return Ok(AppendOutcome::Skipped);
        }
        if requested != latest + 1 {
            return Err(format!(
                "Sequence mismatch for aggregate {}: expected {}, got {}",
                input.aggregate_id,
                latest + 1,
                requested
            ));
        }
    }

    let seq = input.seq.unwrap_or(latest + 1);
    // Duplicate event id across aggregates is rejected the same way the unique insert is.
    let existing = {
        let mut stmt = tx
            .prepare("SELECT aggregate_id, seq FROM \"event\" WHERE id = ?1")
            .map_err(|e| e.to_string())?;
        let mut rows = stmt
            .query_map([input.id], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?))
            })
            .map_err(|e| e.to_string())?;
        match rows.next() {
            Some(r) => Some(r.map_err(|e| e.to_string())?),
            None => None,
        }
    };
    if let Some((stored_aggregate, stored_seq)) = existing {
        return Err(format!(
            "Event {} already exists at aggregate {} sequence {}",
            input.id, stored_aggregate, stored_seq
        ));
    }

    // Upsert sequence row (source `onConflictDoUpdate` on aggregate_id).
    let has_row = row.is_some();
    if has_row {
        tx.execute(
            "UPDATE \"event_sequence\" SET seq = ?1 WHERE aggregate_id = ?2",
            rusqlite::params![seq, input.aggregate_id],
        )
        .map_err(|e| e.to_string())?;
        if input.owner_id.is_some() && row_owner.is_none() {
            tx.execute(
                "UPDATE \"event_sequence\" SET owner_id = ?1 WHERE aggregate_id = ?2",
                rusqlite::params![input.owner_id, input.aggregate_id],
            )
            .map_err(|e| e.to_string())?;
        }
    } else {
        tx.execute(
            "INSERT INTO \"event_sequence\" (aggregate_id, seq, owner_id) VALUES (?1, ?2, ?3)",
            rusqlite::params![input.aggregate_id, seq, input.owner_id],
        )
        .map_err(|e| e.to_string())?;
    }

    let encoded = serde_json::to_string(input.data).map_err(|e| e.to_string())?;
    tx.execute(
        "INSERT INTO \"event\" (id, aggregate_id, seq, type, data) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![input.id, input.aggregate_id, seq, input.event_type, encoded],
    )
    .map_err(|e| e.to_string())?;
    tx.commit().map_err(|e| e.to_string())?;
    Ok(AppendOutcome::Committed {
        aggregate_id: input.aggregate_id.to_string(),
        seq,
    })
}

fn select_event_at(
    conn: &Connection,
    aggregate_id: &str,
    seq: i64,
) -> Result<Option<EventRow>, String> {
    let mut stmt = conn
        .prepare("SELECT id, aggregate_id, seq, type, data FROM \"event\" WHERE aggregate_id = ?1 AND seq = ?2")
        .map_err(|e| e.to_string())?;
    let mut rows = stmt
        .query_map(rusqlite::params![aggregate_id, seq], row_from)
        .map_err(|e| e.to_string())?;
    match rows.next() {
        Some(r) => Ok(Some(r.map_err(|e| e.to_string())?)),
        None => Ok(None),
    }
}

fn row_from(row: &rusqlite::Row<'_>) -> rusqlite::Result<EventRow> {
    let data: String = row.get(4)?;
    let data: serde_json::Value = serde_json::from_str(&data).unwrap_or(serde_json::Value::Null);
    Ok(EventRow {
        id: row.get(0)?,
        aggregate_id: row.get(1)?,
        seq: row.get(2)?,
        r#type: row.get(3)?,
        data,
    })
}

/// `EventV2.readAggregate` persistence — events after `after` in `types`, ordered by seq,
/// limited to `limit + 1` to compute `hasMore`.
pub fn read_aggregate(
    conn: &Connection,
    aggregate_id: &str,
    after: i64,
    types: &[&str],
    limit: usize,
) -> Result<(Vec<EventRow>, bool), String> {
    let mut sql = String::from(
        "SELECT id, aggregate_id, seq, type, data FROM \"event\" WHERE aggregate_id = ?1 AND seq > ?2",
    );
    let mut params: Vec<&dyn rusqlite::types::ToSql> = vec![&aggregate_id, &after];
    if !types.is_empty() {
        sql.push_str(" AND type IN (");
        for (i, _) in types.iter().enumerate() {
            if i > 0 {
                sql.push_str(", ");
            }
            sql.push_str(&format!("?{}", i + 3));
            params.push(&types[i]);
        }
        sql.push(')');
    }
    sql.push_str(" ORDER BY seq ASC LIMIT ?");
    let limit_plus_one = limit + 1;
    params.push(&limit_plus_one);
    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map(params.as_slice(), row_from)
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    let has_more = out.len() > limit;
    out.truncate(limit);
    Ok((out, has_more))
}

/// `EventV2.remove` — clears durable event sequence + events for an aggregate (one transaction).
pub fn remove(conn: &mut Connection, aggregate_id: &str) -> Result<(), String> {
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    tx.execute(
        "DELETE FROM \"event_sequence\" WHERE aggregate_id = ?1",
        [aggregate_id],
    )
    .map_err(|e| e.to_string())?;
    tx.execute(
        "DELETE FROM \"event\" WHERE aggregate_id = ?1",
        [aggregate_id],
    )
    .map_err(|e| e.to_string())?;
    tx.commit().map_err(|e| e.to_string())
}

/// `EventV2.claim` — updates the event sequence owner.
pub fn claim(conn: &Connection, aggregate_id: &str, owner_id: &str) -> Result<(), String> {
    conn.execute(
        "UPDATE \"event_sequence\" SET owner_id = ?1 WHERE aggregate_id = ?2",
        rusqlite::params![owner_id, aggregate_id],
    )
    .map(|_| ())
    .map_err(|e| e.to_string())
}

/// `EventV2.replay` persistence branch for one serialized event.
pub fn replay(
    conn: &mut Connection,
    event: &SerializedEvent,
    owner_id: Option<&str>,
    strict_owner: bool,
) -> Result<(), String> {
    let input = AppendInput {
        id: &event.id,
        aggregate_id: &event.aggregate_id,
        event_type: &event.r#type,
        data: &event.data,
        seq: Some(event.seq),
        owner_id,
        strict_owner,
    };
    append(conn, &input).map(|_| ())
}

/// `EventV2.replayAll` — validates contiguous aggregate events, replays each, returns the source aggregate.
pub fn replay_all(
    conn: &mut Connection,
    events: &[SerializedEvent],
    owner_id: Option<&str>,
    strict_owner: bool,
) -> Result<Option<String>, String> {
    let source = events.first().map(|e| e.aggregate_id.clone());
    let Some(source) = source else {
        return Ok(None);
    };
    if events.iter().any(|e| e.aggregate_id != source) {
        return Err(ERROR_REPLAY_ALL_AGGREGATE.to_string());
    }
    let start = events.first().map(|e| e.seq).unwrap_or(0);
    for (index, event) in events.iter().enumerate() {
        let seq = start + index as i64;
        if event.seq != seq {
            return Err(format!(
                "Replay sequence mismatch at index {}: expected {}, got {}",
                index, seq, event.seq
            ));
        }
    }
    for event in events {
        replay(conn, event, owner_id, strict_owner)?;
    }
    Ok(Some(source))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mem() -> Result<Connection, String> {
        crate::database::database::open_in_memory()
    }

    fn ev(id: &str, agg: &str, seq: i64, data: serde_json::Value) -> SerializedEvent {
        SerializedEvent {
            id: id.to_string(),
            r#type: "chat.msg".to_string(),
            seq,
            aggregate_id: agg.to_string(),
            data,
        }
    }

    #[test]
    fn publishes_increment_seq_per_aggregate() {
        let mut conn = mem().unwrap();
        assert_eq!(latest_sequence(&conn, "agg-1").unwrap(), -1);
        let out = append(
            &mut conn,
            &AppendInput {
                id: "e1",
                aggregate_id: "agg-1",
                event_type: "chat.msg@1",
                data: &serde_json::json!({"text": "hi"}),
                seq: None,
                owner_id: None,
                strict_owner: false,
            },
        )
        .unwrap();
        assert_eq!(
            out,
            AppendOutcome::Committed {
                aggregate_id: "agg-1".into(),
                seq: 0
            }
        );
        append(
            &mut conn,
            &AppendInput {
                id: "e2",
                aggregate_id: "agg-1",
                event_type: "chat.msg@1",
                data: &serde_json::json!({"text": "yo"}),
                seq: None,
                owner_id: None,
                strict_owner: false,
            },
        )
        .unwrap();
        assert_eq!(latest_sequence(&conn, "agg-1").unwrap(), 1);
        let (rows, has_more) = read_aggregate(&conn, "agg-1", -1, &[], 10).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].seq, 0);
        assert_eq!(rows[1].seq, 1);
        assert!(!has_more);
    }

    #[test]
    fn exact_replay_is_idempotent() {
        let mut conn = mem().unwrap();
        let data = serde_json::json!({"text": "hi"});
        let e = ev("e1", "agg-1", 0, data.clone());
        replay(&mut conn, &e, None, false).unwrap();
        // identical replay → Idempotent, no duplicate row
        replay(&mut conn, &e, None, false).unwrap();
        let (rows, _) = read_aggregate(&conn, "agg-1", -1, &[], 10).unwrap();
        assert_eq!(rows.len(), 1);
        // divergent replay → error
        let divergent = SerializedEvent {
            data: serde_json::json!({"text": "different"}),
            ..e.clone()
        };
        let err = replay(&mut conn, &divergent, None, false).unwrap_err();
        assert_eq!(err, "Replay diverged at aggregate agg-1 sequence 0");
    }

    #[test]
    fn replay_all_validates_contiguity() {
        let mut conn = mem().unwrap();
        let e0 = ev("e1", "agg-1", 3, serde_json::json!({}));
        let e1 = ev("e2", "agg-1", 5, serde_json::json!({}));
        let err = replay_all(&mut conn, &[e0, e1], None, false).unwrap_err();
        assert_eq!(
            err,
            "Replay sequence mismatch at index 1: expected 4, got 5"
        );
    }

    #[test]
    fn claim_and_strict_owner_fence() {
        let mut conn = mem().unwrap();
        let e = ev("e1", "agg-1", 0, serde_json::json!({}));
        replay(&mut conn, &e, Some("owner-a"), false).unwrap();
        assert_eq!(
            get_sequence(&conn, "agg-1")
                .unwrap()
                .unwrap()
                .owner_id
                .as_deref(),
            Some("owner-a")
        );
        // different owner exact replay with strictOwner → mismatch error naming expected/got
        let again = ev("e1", "agg-1", 0, serde_json::json!({}));
        let err = replay(&mut conn, &again, Some("owner-b"), true).unwrap_err();
        assert_eq!(
            err,
            "Replay owner mismatch for aggregate agg-1: expected owner-a, got owner-b"
        );
        // claim updates owner
        claim(&conn, "agg-1", "owner-c").unwrap();
        assert_eq!(
            get_sequence(&conn, "agg-1")
                .unwrap()
                .unwrap()
                .owner_id
                .as_deref(),
            Some("owner-c")
        );
        // remove clears aggregate (sequence + events)
        remove(&mut conn, "agg-1").unwrap();
        assert_eq!(latest_sequence(&conn, "agg-1").unwrap(), -1);
        let (rows, _) = read_aggregate(&conn, "agg-1", -1, &[], 10).unwrap();
        assert!(rows.is_empty());
    }

    #[test]
    fn rejects_event_id_reused_at_another_aggregate() {
        let mut conn = mem().unwrap();
        append(
            &mut conn,
            &AppendInput {
                id: "e1",
                aggregate_id: "agg-1",
                event_type: "chat.msg@1",
                data: &serde_json::json!({}),
                seq: Some(0),
                owner_id: None,
                strict_owner: false,
            },
        )
        .unwrap();
        let err = append(
            &mut conn,
            &AppendInput {
                id: "e1",
                aggregate_id: "agg-2",
                event_type: "chat.msg@1",
                data: &serde_json::json!({}),
                seq: Some(0),
                owner_id: None,
                strict_owner: false,
            },
        )
        .unwrap_err();
        assert_eq!(err, "Event e1 already exists at aggregate agg-1 sequence 0");
    }
}
