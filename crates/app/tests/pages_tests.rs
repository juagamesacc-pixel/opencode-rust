use app::pages::error_description::error_description_key;
use app::pages::home_session_open::{should_open_session_in_background, ShouldOpenInput};
use app::pages::layout::deep_links::{
    collect_open_project_deep_links, drain_pending_deep_links, parse_deep_link,
    parse_new_session_deep_link,
};
use app::pages::new_session::new_session_workspace_controller::{
    normalize_new_session_worktree, resolve_new_session_branch, resolve_new_session_worktree,
    ResolveBranchInput, ResolveWorktreeInput,
};
use app::pages::session::composer::session_composer_state::{todo_dock_at_boundary, todo_state};
use app::pages::session::composer::session_request_tree::{
    session_permission_request, PermissionRequest, Session,
};
use app::pages::session::file_tab_scroll::{next_tab_list_scroll_left, TabScrollInput};
use app::pages::session::helpers::{get_session_key, get_tab_reorder_index, should_show_file_tree};
use app::pages::session::message_gesture::{
    normalize_wheel_delta, should_mark_boundary_gesture, BoundaryInput, WheelInput,
};
use app::pages::session::message_id_from_hash::message_id_from_hash;
use app::pages::session::new_session_layout::NEW_SESSION_CONTENT_WIDTH;
use app::pages::session::session_panel_layout::{session_panel_layout, SessionPanelLayoutInput};
use app::pages::session::session_panel_width::{
    clamp_session_panel_width, session_panel_width_max, REVIEW_PANE_WIDTH_MIN,
    REVIEW_PANE_WIDTH_MIN_SPLIT, SESSION_PANEL_WIDTH_MIN,
};
use app::pages::session::timeline::summary_diffs::{unique_summary_diffs, SummaryDiff};
use app::pages::session::v2::review_diff_kinds::{
    filter_review_files, review_diff_kinds, review_diff_needs_load, review_root_directory,
};

#[test]
fn error_description_key_local_startup() {
    let err = serde_json::json!({ "localServerStartup": true });
    assert_eq!(
        error_description_key(&err),
        "error.page.description.localServerStartup"
    );
    let err2 = serde_json::json!({ "localServerStartup": false });
    assert_eq!(error_description_key(&err2), "error.page.description");
    let err3 = serde_json::json!({});
    assert_eq!(error_description_key(&err3), "error.page.description");
}

#[test]
fn should_open_session_in_background_middle_and_modifiers() {
    assert!(should_open_session_in_background(ShouldOpenInput {
        button: 1,
        mac: true,
        meta: false,
        ctrl: false,
        shift: false,
        alt: false
    }));
    assert!(!should_open_session_in_background(ShouldOpenInput {
        button: 2,
        mac: true,
        meta: false,
        ctrl: false,
        shift: false,
        alt: false
    }));
    assert!(should_open_session_in_background(ShouldOpenInput {
        button: 0,
        mac: true,
        meta: true,
        ctrl: false,
        shift: false,
        alt: false
    }));
    assert!(should_open_session_in_background(ShouldOpenInput {
        button: 0,
        mac: false,
        meta: false,
        ctrl: true,
        shift: false,
        alt: false
    }));
    assert!(!should_open_session_in_background(ShouldOpenInput {
        button: 0,
        mac: true,
        meta: true,
        ctrl: false,
        shift: true,
        alt: false
    }));
    assert!(!should_open_session_in_background(ShouldOpenInput {
        button: 0,
        mac: false,
        meta: true,
        ctrl: false,
        shift: false,
        alt: false
    }));
}

#[test]
fn next_tab_list_scroll_left_cases() {
    assert_eq!(
        next_tab_list_scroll_left(TabScrollInput {
            prev_scroll_width: 500,
            scroll_width: 420,
            client_width: 300,
            prev_context_open: false,
            context_open: false
        }),
        None
    );
    assert_eq!(
        next_tab_list_scroll_left(TabScrollInput {
            prev_scroll_width: 400,
            scroll_width: 500,
            client_width: 320,
            prev_context_open: false,
            context_open: true
        }),
        Some(0)
    );
    assert_eq!(
        next_tab_list_scroll_left(TabScrollInput {
            prev_scroll_width: 500,
            scroll_width: 780,
            client_width: 300,
            prev_context_open: true,
            context_open: true
        }),
        Some(480)
    );
}

#[test]
fn normalize_wheel_delta_cases() {
    assert_eq!(
        normalize_wheel_delta(WheelInput {
            delta_y: 3.0,
            delta_mode: 1,
            root_height: 500.0
        }),
        120.0
    );
    assert_eq!(
        normalize_wheel_delta(WheelInput {
            delta_y: -1.0,
            delta_mode: 2,
            root_height: 600.0
        }),
        -600.0
    );
    assert_eq!(
        normalize_wheel_delta(WheelInput {
            delta_y: 16.0,
            delta_mode: 0,
            root_height: 600.0
        }),
        16.0
    );
}

#[test]
fn should_mark_boundary_gesture_cases() {
    assert!(should_mark_boundary_gesture(BoundaryInput {
        delta: 20.0,
        scroll_top: 0.0,
        scroll_height: 300.0,
        client_height: 300.0
    }));
    assert!(should_mark_boundary_gesture(BoundaryInput {
        delta: -40.0,
        scroll_top: 10.0,
        scroll_height: 1000.0,
        client_height: 400.0
    }));
    assert!(should_mark_boundary_gesture(BoundaryInput {
        delta: 50.0,
        scroll_top: 580.0,
        scroll_height: 1000.0,
        client_height: 400.0
    }));
    assert!(!should_mark_boundary_gesture(BoundaryInput {
        delta: 20.0,
        scroll_top: 200.0,
        scroll_height: 1000.0,
        client_height: 400.0
    }));
}

#[test]
fn message_id_from_hash_cases() {
    assert_eq!(
        message_id_from_hash("#message-abc123"),
        Some("abc123".to_string())
    );
    assert_eq!(message_id_from_hash("message-xyz"), Some("xyz".to_string()));
    assert_eq!(message_id_from_hash("#other-123"), None);
    assert_eq!(message_id_from_hash(""), None);
}

#[test]
fn session_panel_layout_cases() {
    let r = session_panel_layout(SessionPanelLayoutInput {
        review: false,
        terminal: false,
        files: false,
    });
    assert!(!r.visible && !r.stacked);
    let r = session_panel_layout(SessionPanelLayoutInput {
        review: false,
        terminal: true,
        files: false,
    });
    assert!(r.visible && !r.stacked);
    let r = session_panel_layout(SessionPanelLayoutInput {
        review: true,
        terminal: true,
        files: false,
    });
    assert!(r.visible && r.stacked);
}

#[test]
fn session_panel_width_cases() {
    assert_eq!(
        session_panel_width_max(1700, false),
        1700 - REVIEW_PANE_WIDTH_MIN
    );
    assert_eq!(
        session_panel_width_max(1700, true),
        1700 - REVIEW_PANE_WIDTH_MIN_SPLIT
    );
    const { assert!(REVIEW_PANE_WIDTH_MIN_SPLIT > REVIEW_PANE_WIDTH_MIN) };
    assert!(session_panel_width_max(3440, false) > (3440_f64 * 0.45) as i32);
    assert_eq!(session_panel_width_max(600, true), SESSION_PANEL_WIDTH_MIN);
    assert_eq!(clamp_session_panel_width(800, Some(1700), false), 800);
    assert_eq!(
        clamp_session_panel_width(1600, Some(1700), false),
        1700 - REVIEW_PANE_WIDTH_MIN
    );
    assert_eq!(clamp_session_panel_width(1600, None, false), 1600);
}

#[test]
fn unique_summary_diffs_cases() {
    assert!(unique_summary_diffs(None).is_empty());
    assert!(unique_summary_diffs(Some(&[])).is_empty());
    let alpha = SummaryDiff {
        file: "alpha.ts".to_string(),
    };
    let beta = SummaryDiff {
        file: "beta.ts".to_string(),
    };
    let invalid = SummaryDiff {
        file: "".to_string(),
    };
    let r = unique_summary_diffs(Some(std::slice::from_ref(&invalid)));
    assert!(r.is_empty());
    let r = unique_summary_diffs(Some(&[alpha.clone(), invalid.clone(), beta.clone()]));
    assert_eq!(r, vec![alpha.clone(), beta.clone()]);

    let old_alpha = SummaryDiff {
        file: "alpha.ts".to_string(),
    };
    let old_beta = SummaryDiff {
        file: "beta.ts".to_string(),
    };
    let new_alpha = SummaryDiff {
        file: "alpha.ts".to_string(),
    };
    let charlie = SummaryDiff {
        file: "charlie.ts".to_string(),
    };
    let new_beta = SummaryDiff {
        file: "beta.ts".to_string(),
    };
    let r = unique_summary_diffs(Some(&[
        old_alpha.clone(),
        old_beta.clone(),
        new_alpha.clone(),
        charlie.clone(),
        new_beta.clone(),
    ]));
    assert_eq!(r, vec![new_alpha, charlie, new_beta]);
}

#[test]
fn review_diff_kinds_cases() {
    let kinds = review_diff_kinds(&[
        ("src/a.ts".to_string(), "added".to_string()),
        ("src/b.ts".to_string(), "deleted".to_string()),
    ]);
    assert_eq!(kinds.get("src/a.ts").map(|s| s.as_str()), Some("add"));
    assert_eq!(kinds.get("src/b.ts").map(|s| s.as_str()), Some("del"));
    assert_eq!(kinds.get("src").map(|s| s.as_str()), Some("mix"));
}

#[test]
fn filter_review_files_cases() {
    let files = vec![
        "src/a.ts".to_string(),
        "src/b.ts".to_string(),
        "lib/c.ts".to_string(),
    ];
    assert_eq!(
        filter_review_files(&files, "b.ts"),
        vec!["src/b.ts".to_string()]
    );
    assert_eq!(filter_review_files(&files, ""), files);
}

#[test]
fn review_diff_needs_load_cases() {
    assert!(review_diff_needs_load(
        1,
        0,
        Some("diff --git a/src/a.ts b/src/a.ts\n--- a/src/a.ts\n+++ b/src/a.ts")
    ));
    assert!(!review_diff_needs_load(1, 0, Some("@@ -0,0 +1 @@\n+value")));
    assert!(!review_diff_needs_load(0, 0, None));
}

#[test]
fn review_root_directory_cases() {
    assert_eq!(review_root_directory("/repo/"), "/repo");
    assert_eq!(review_root_directory("/"), "/");
    assert_eq!(review_root_directory("C:\\"), "C:\\");
}

#[test]
fn todo_state_cases() {
    assert_eq!(todo_state(0, false, true), "hide");
    assert_eq!(todo_state(2, false, false), "clear");
    assert_eq!(todo_state(2, false, true), "open");
    assert_eq!(todo_state(2, true, true), "close");
    assert!(todo_dock_at_boundary("open"));
    assert!(!todo_dock_at_boundary("close"));
}

#[test]
fn session_request_tree_cases() {
    let sessions = vec![
        Session {
            id: "root".to_string(),
            parent_id: None,
        },
        Session {
            id: "child".to_string(),
            parent_id: Some("root".to_string()),
        },
    ];
    let mut perms = std::collections::HashMap::new();
    perms.insert(
        "root".to_string(),
        vec![PermissionRequest {
            id: "perm-root".to_string(),
            session_id: "root".to_string(),
        }],
    );
    perms.insert(
        "child".to_string(),
        vec![PermissionRequest {
            id: "perm-child".to_string(),
            session_id: "child".to_string(),
        }],
    );
    assert_eq!(
        session_permission_request(&sessions, &perms, Some("root"), None).map(|p| p.id),
        Some("perm-root".to_string())
    );
    let sessions2 = vec![
        Session {
            id: "root".to_string(),
            parent_id: None,
        },
        Session {
            id: "child".to_string(),
            parent_id: Some("root".to_string()),
        },
        Session {
            id: "grand".to_string(),
            parent_id: Some("child".to_string()),
        },
    ];
    let mut perms2 = std::collections::HashMap::new();
    perms2.insert(
        "grand".to_string(),
        vec![PermissionRequest {
            id: "perm-grand".to_string(),
            session_id: "grand".to_string(),
        }],
    );
    assert_eq!(
        session_permission_request(&sessions2, &perms2, Some("root"), None).map(|p| p.id),
        Some("perm-grand".to_string())
    );
}

#[test]
fn new_session_workspace_cases() {
    assert_eq!(
        resolve_new_session_worktree(ResolveWorktreeInput {
            enabled: false,
            selected: Some("/project/feature".to_string()),
            directory: "/project/feature".to_string(),
            project_worktree: Some("/project".to_string())
        }),
        "main"
    );
    assert_eq!(
        resolve_new_session_worktree(ResolveWorktreeInput {
            enabled: true,
            selected: None,
            directory: "/project/feature".to_string(),
            project_worktree: Some("/project".to_string())
        }),
        "/project/feature"
    );
    assert_eq!(
        resolve_new_session_worktree(ResolveWorktreeInput {
            enabled: true,
            selected: None,
            directory: "/project".to_string(),
            project_worktree: Some("/project".to_string())
        }),
        "main"
    );
    assert_eq!(
        normalize_new_session_worktree("main", "/project/feature", Some("/project")),
        "/project"
    );
    assert_eq!(
        normalize_new_session_worktree("main", "/project", Some("/project")),
        "main"
    );
    let branch = |w: &str| {
        if w == "/project/feature" {
            Some("feature".to_string())
        } else {
            None
        }
    };
    assert_eq!(
        resolve_new_session_branch(
            ResolveBranchInput {
                worktree: "main".to_string(),
                local: Some("dev".to_string())
            },
            branch
        ),
        Some("dev".to_string())
    );
    assert_eq!(
        resolve_new_session_branch(
            ResolveBranchInput {
                worktree: "/project/feature".to_string(),
                local: Some("dev".to_string())
            },
            branch
        ),
        Some("feature".to_string())
    );
}

#[test]
fn deep_links_cases() {
    assert_eq!(
        parse_deep_link("opencode://open-project?directory=/tmp/demo"),
        Some("/tmp/demo".to_string())
    );
    assert_eq!(
        parse_deep_link("opencode://other?directory=/tmp/demo"),
        None
    );
    assert_eq!(parse_deep_link("opencode://open-project"), None);
    assert_eq!(
        parse_new_session_deep_link("opencode://new-session?directory=/tmp/demo"),
        Some(("/tmp/demo".to_string(), None))
    );
    assert_eq!(
        parse_new_session_deep_link(
            "opencode://new-session?directory=/tmp/demo&prompt=hello%20world"
        ),
        Some(("/tmp/demo".to_string(), Some("hello world".to_string())))
    );
    let r = collect_open_project_deep_links(&[
        "opencode://open-project?directory=/a".to_string(),
        "opencode://other?directory=/b".to_string(),
        "opencode://open-project?directory=/c".to_string(),
    ]);
    assert_eq!(r, vec!["/a".to_string(), "/c".to_string()]);
    let mut pending = vec!["a".to_string(), "b".to_string()];
    let drained = drain_pending_deep_links(&mut pending);
    assert_eq!(drained, vec!["a".to_string(), "b".to_string()]);
    assert!(pending.is_empty());
}

#[test]
fn helpers_cases() {
    assert_eq!(get_session_key(Some("/tmp"), Some("123")), "/tmp/123");
    assert_eq!(get_session_key(None, None), "");
    assert!(should_show_file_tree(true, true));
    assert!(!should_show_file_tree(false, true));
    let tabs = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    assert_eq!(get_tab_reorder_index(&tabs, "a", "c"), Some(2));
    assert_eq!(get_tab_reorder_index(&tabs, "a", "a"), None);
    assert_eq!(NEW_SESSION_CONTENT_WIDTH, "w-full max-w-[720px] px-0");
}
