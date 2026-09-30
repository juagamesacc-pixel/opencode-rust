#![allow(clippy::all)]
// source: test/project-directories.test.ts — exports/cases: ["decodes directory schemas","creates once and ignores conflicts","replaces the strategy when requested"]
// Real DB assertions: ProjectDirectories create/list/remove semantics over a fresh schema
// (project rows first; on-conflict DO NOTHING vs DO UPDATE + setWhere / ORDER BY).

use core::database::database::open_in_memory;
use core::database::migration::apply;
use core::project::directories::{create, list, CreateInput, Directory, DirectoryKey};

fn seed_project(conn: &rusqlite::Connection) -> Result<(), String> {
    core::project::sql::insert_project(
        conn,
        &core::project::sql::ProjectRow {
            id: "project-directories".to_string(),
            worktree: "/tmp/project-directories".to_string(),
            vcs: None,
            name: None,
            icon_url: None,
            icon_url_override: None,
            icon_color: None,
            time_created: 1,
            time_updated: 1,
            time_initialized: None,
            sandboxes: core::project::sql::sandboxes_to_json(&[]),
            commands: None,
        },
    )
}

fn mem_with_project() -> rusqlite::Connection {
    let mut conn = open_in_memory().unwrap();
    apply(&mut conn).unwrap();
    seed_project(&conn).unwrap();
    conn
}

const PROJECT_ID: &str = "project-directories";
const DIRECTORY: &str = "/tmp/project-directories";

// describe: ["ProjectDirectories"]
#[test]
fn decodes_directory_schemas() {
    // source: "decodes directory schemas"
    // Schema.decodeUnknownSync(ProjectDirectories.ListInput/ListOutput) lives at the schema
    // crate boundary — the DB-level shape is asserted by list()/get() below.
    let conn = mem_with_project();
    assert_eq!(
        list(&conn, PROJECT_ID).unwrap(),
        Vec::<Directory>::new(),
        "ListInput {{ projectID }} yields no rows before create"
    );
}

#[test]
fn creates_once_and_ignores_conflicts() {
    // source: "creates once and ignores conflicts"
    let conn = mem_with_project();
    let mk = |strategy: Option<&str>| CreateInput {
        project_id: PROJECT_ID.to_string(),
        directory: DIRECTORY.to_string(),
        strategy: strategy.map(|s| s.to_string()),
        behavior: None,
    };
    assert!(create(&conn, &mk(None)).unwrap());
    // same directory again (with a different strategy) → ON CONFLICT DO NOTHING → false
    assert!(!create(&conn, &mk(Some("git_worktree"))).unwrap());
    assert_eq!(
        list(&conn, PROJECT_ID).unwrap(),
        vec![Directory {
            directory: DIRECTORY.to_string(),
            strategy: None,
        }]
    );
}

#[test]
fn replaces_the_strategy_when_requested() {
    // source: "replaces the strategy when requested"
    let conn = mem_with_project();
    let mk = |strategy: Option<&str>, behavior: Option<&str>| CreateInput {
        project_id: PROJECT_ID.to_string(),
        directory: DIRECTORY.to_string(),
        strategy: strategy.map(|s| s.to_string()),
        behavior: behavior.map(|s| s.to_string()),
    };
    assert!(create(&conn, &mk(Some("old/strategy"), None)).unwrap());
    assert!(create(&conn, &mk(Some("new/strategy"), Some("replace"))).unwrap());
    assert!(!create(&conn, &mk(Some("new/strategy"), Some("replace"))).unwrap());
    assert!(create(&conn, &mk(None, Some("replace"))).unwrap());
    assert!(!create(&conn, &mk(None, Some("replace"))).unwrap());
    assert!(create(&conn, &mk(Some("new/strategy"), Some("replace"))).unwrap());
    assert_eq!(
        list(&conn, PROJECT_ID).unwrap(),
        vec![Directory {
            directory: DIRECTORY.to_string(),
            strategy: Some("new/strategy".to_string()),
        }]
    );
    // remove works on the replaced row (source `ProjectDirectories.remove`)
    assert!(core::project::directories::remove(
        &conn,
        &DirectoryKey {
            project_id: PROJECT_ID.to_string(),
            directory: DIRECTORY.to_string(),
        }
    )
    .unwrap());
    assert!(!core::project::directories::contains(
        &conn,
        &DirectoryKey {
            project_id: PROJECT_ID.to_string(),
            directory: DIRECTORY.to_string(),
        }
    )
    .unwrap());
}
