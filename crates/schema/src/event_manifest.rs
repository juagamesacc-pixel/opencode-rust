//! Port of `packages/schema/src/event-manifest.ts` (public event manifest).
//!
//! Source exports: `ServerDefinitions`, `Definitions`, `Latest`, and
//! re-exported `Durable` (from `durable-event-manifest.ts`). Assembly order is
//! verbatim from source: `coreDefinitions` (SessionV1 durable + SessionEvent),
//! `foundationDefinitions` (ModelsDev + Integration + Catalog + core),
//! `featureDefinitions` (FileSystem + Reference + Permission + Plugin +
//! ProjectDirectories + FileSystemWatcher + Pty + Question),
//! `ServerDefinitions` (foundation + feature + SessionTodo), `Definitions`
//! (foundation + SessionV1 live + Installation + feature + SessionTodo + Lsp +
//! PermissionV1 + Tui + Mcp + Legacy + Project + SessionStatus + QuestionV1 +
//! SessionCompaction + Vcs + Workspace + Worktree + ServerEvent).
//!
//! Cross-lane assumptions (plan §4; assembly lane reconciles, CI verifies
//! counts 55 / 85 / 85 / 32):
//! - Every domain module exposes `Event::Definitions: &[&'static str]` in
//!   source declaration order (same convention as this lane's modules).
//! - `crate::session_event::Event::DurableDefinitions: &[DurableEntry]`
//!   lists the SessionEvent durable defs with verbatim aggregate/version;
//!   SessionV1 markers (via the flat `crate::session_v1` shim, this lane)
//!   carry their own durable consts.
//! - All other Lane A/B defs are transient (no durable meta).
//!
//! `Latest`/`Durable` maps mirror `Event.latest` / `Event.durable`
//! (`event.ts`): latest keeps the highest durable version per type and panics
//! on conflicting duplicates with the source messages verbatim.

#![allow(non_snake_case, non_upper_case_globals)]

use std::collections::BTreeMap;
use std::sync::LazyLock;

pub use crate::event::{Definition, Durable};

/// Durable entry shape assumed for SessionEvent durable definitions (manifest row view).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DurableEntry {
    pub r#type: &'static str,
    pub aggregate: &'static str,
    pub version: i64,
}

fn live(r#type: &'static str) -> Definition {
    Definition {
        r#type,
        durable: None,
        data: "",
    }
}

fn durable(r#type: &'static str, aggregate: &'static str, version: i64) -> Definition {
    Definition {
        r#type,
        durable: Some(Durable { aggregate, version }),
        data: "",
    }
}

fn push_all(out: &mut Vec<Definition>, types: &[&'static str]) {
    out.extend(types.iter().map(|t| live(t)));
}

/// `type.version` key (ports `Event.versionedType`; crate-private: the shared
/// helper lives in Lane A `crate::event` once the assembly lane wires it).
fn versioned_type(r#type: &str, version: i64) -> String {
    format!("{}.{}", r#type, version)
}

fn build_latest(definitions: &[Definition]) -> BTreeMap<String, Definition> {
    let mut result = BTreeMap::new();
    for definition in definitions {
        match result.get(definition.r#type) {
            None => {
                result.insert(definition.r#type.to_string(), definition.clone());
            }
            Some(existing) => {
                match (
                    existing.durable.as_ref().map(|d| d.version),
                    definition.durable.as_ref().map(|d| d.version),
                ) {
                    (Some(a), Some(b)) if a != b => {
                        if b > a {
                            result.insert(definition.r#type.to_string(), definition.clone());
                        }
                    }
                    _ => {
                        if existing != definition {
                            panic!(
                                "Duplicate latest event definition for {}",
                                definition.r#type
                            );
                        }
                    }
                }
            }
        }
    }
    result
}

fn build_durable(definitions: &[Definition]) -> BTreeMap<String, Definition> {
    let mut result = BTreeMap::new();
    for definition in definitions {
        if let Some(durable) = definition.durable.as_ref() {
            let version = durable.version;
            let key = versioned_type(definition.r#type, version);
            if result.contains_key(&key) {
                panic!("Duplicate durable event definition for {key}");
            }
            result.insert(key, definition.clone());
        }
    }
    result
}

fn session_v1_durable() -> Vec<Definition> {
    use crate::session_v1::Event as V1;
    vec![
        durable(
            V1::Created::TYPE,
            V1::Created::DURABLE_AGGREGATE.unwrap(),
            V1::Created::DURABLE_VERSION.unwrap(),
        ),
        durable(
            V1::Updated::TYPE,
            V1::Updated::DURABLE_AGGREGATE.unwrap(),
            V1::Updated::DURABLE_VERSION.unwrap(),
        ),
        durable(
            V1::Deleted::TYPE,
            V1::Deleted::DURABLE_AGGREGATE.unwrap(),
            V1::Deleted::DURABLE_VERSION.unwrap(),
        ),
        durable(
            V1::MessageUpdated::TYPE,
            V1::MessageUpdated::DURABLE_AGGREGATE.unwrap(),
            V1::MessageUpdated::DURABLE_VERSION.unwrap(),
        ),
        durable(
            V1::MessageRemoved::TYPE,
            V1::MessageRemoved::DURABLE_AGGREGATE.unwrap(),
            V1::MessageRemoved::DURABLE_VERSION.unwrap(),
        ),
        durable(
            V1::PartUpdated::TYPE,
            V1::PartUpdated::DURABLE_AGGREGATE.unwrap(),
            V1::PartUpdated::DURABLE_VERSION.unwrap(),
        ),
        durable(
            V1::PartRemoved::TYPE,
            V1::PartRemoved::DURABLE_AGGREGATE.unwrap(),
            V1::PartRemoved::DURABLE_VERSION.unwrap(),
        ),
    ]
}

fn session_v1_live() -> Vec<Definition> {
    use crate::session_v1::Event as V1;
    vec![
        live(V1::PartDelta::TYPE),
        live(V1::Diff::TYPE),
        live(V1::Error::TYPE),
    ]
}

fn session_event_definitions() -> Vec<Definition> {
    crate::session_event::Event::Definitions
        .iter()
        .map(|t| live(t))
        .collect()
}

fn session_event_durable() -> Vec<Definition> {
    crate::session_event::Event::DurableDefinitions
        .iter()
        .map(|entry: &DurableEntry| durable(entry.r#type, entry.aggregate, entry.version))
        .collect()
}

fn foundation_definitions() -> Vec<Definition> {
    let mut out = Vec::new();
    push_all(&mut out, crate::models_dev::Event::Definitions);
    push_all(&mut out, crate::integration::Event::Definitions);
    push_all(&mut out, crate::catalog::Event::Definitions);
    out.extend(session_v1_durable());
    out.extend(session_event_definitions());
    out
}

fn feature_definitions() -> Vec<Definition> {
    let mut out = Vec::new();
    push_all(&mut out, crate::filesystem::Event::Definitions);
    push_all(&mut out, crate::reference::Event::Definitions);
    push_all(&mut out, crate::permission::Event::Definitions);
    out.extend(crate::plugin::Event::Definitions.iter().cloned());
    out.extend(
        crate::project_directories::Event::Definitions
            .iter()
            .cloned(),
    );
    push_all(&mut out, crate::filesystem_watcher::Event::Definitions);
    push_all(&mut out, crate::pty::Event::Definitions);
    push_all(&mut out, crate::question::Event::Definitions);
    out
}

fn server_definitions_inner() -> Vec<Definition> {
    let mut out = foundation_definitions();
    out.extend(feature_definitions());
    push_all(&mut out, crate::session_todo::Event::Definitions);
    out
}

fn definitions_inner() -> Vec<Definition> {
    let mut out = foundation_definitions();
    out.extend(session_v1_live());
    push_all(&mut out, crate::installation_event::Definitions);
    out.extend(feature_definitions());
    push_all(&mut out, crate::session_todo::Event::Definitions);
    push_all(&mut out, crate::lsp_event::Definitions);
    push_all(&mut out, crate::permission_v1::Event::Definitions);
    push_all(&mut out, crate::tui_event::Definitions);
    push_all(&mut out, crate::mcp_event::Definitions);
    push_all(&mut out, crate::legacy_event::Definitions);
    out.extend(crate::project::Event::Definitions.iter().cloned());
    push_all(&mut out, crate::session_status_event::Event::Definitions);
    push_all(&mut out, crate::question_v1::Event::Definitions);
    push_all(
        &mut out,
        crate::session_compaction_event::Event::Definitions,
    );
    push_all(&mut out, crate::vcs_event::Definitions);
    out.extend(crate::workspace_event::Definitions.iter().cloned());
    push_all(&mut out, crate::worktree_event::Definitions);
    push_all(&mut out, crate::server_event::Definitions);
    out
}

/// `EventManifest.ServerDefinitions` (source expects length 55).
pub static ServerDefinitions: LazyLock<Vec<Definition>> = LazyLock::new(server_definitions_inner);

/// `EventManifest.Definitions` (source expects length 85).
pub static Definitions: LazyLock<Vec<Definition>> = LazyLock::new(definitions_inner);

/// `EventManifest.Latest` (source expects size 85).
pub static Latest: LazyLock<BTreeMap<String, Definition>> =
    LazyLock::new(|| build_latest(&definitions_inner()));

/// `EventManifest.Durable` — re-exported from `durable-event-manifest`
/// in source (`export { Durable }`).
pub use crate::durable_event_manifest::Durable;
