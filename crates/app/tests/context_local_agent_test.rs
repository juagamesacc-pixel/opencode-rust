//! Rust port of `packages/app/src/context/local-agent.test.ts` (opencode v1.18.30).
//!
//! Source 29 lines, 5 tests across `hasCustomAgent` + `resolveAgent`.
//! 1:1 parity — same inputs/assertions.

use app::context::local_agent::{hasCustomAgent, resolveAgent, AgentItem, NamedAgent};

fn native(native: Option<bool>, name: &str) -> AgentItem {
    AgentItem {
        native,
        name: Some(name.to_string()),
    }
}

fn named(name: &str) -> NamedAgent {
    NamedAgent {
        name: name.to_string(),
    }
}

#[test]
fn detects_explicitly_custom_agents() {
    assert!(hasCustomAgent(&[
        native(Some(true), "a"),
        native(Some(false), "b")
    ]));
}

#[test]
fn ignores_built_in_and_unclassified_agents() {
    assert!(!hasCustomAgent(&[
        native(Some(true), "a"),
        native(None, "b")
    ]));
}

#[test]
fn uses_the_requested_available_agent() {
    let agents = [named("plan"), named("build"), named("custom")];
    assert_eq!(
        resolveAgent(&agents, Some("custom"))
            .as_ref()
            .map(|a| a.name.as_str()),
        Some("custom")
    );
}

#[test]
fn defaults_to_build() {
    let agents = [named("plan"), named("build"), named("custom")];
    assert_eq!(
        resolveAgent(&agents, None)
            .as_ref()
            .map(|a| a.name.as_str()),
        Some("build")
    );
    assert_eq!(
        resolveAgent(&agents, Some("missing"))
            .as_ref()
            .map(|a| a.name.as_str()),
        Some("build")
    );
}

#[test]
fn uses_the_first_agent_when_build_is_unavailable() {
    let agents = [named("custom")];
    assert_eq!(
        resolveAgent(&agents, Some("missing"))
            .as_ref()
            .map(|a| a.name.as_str()),
        Some("custom")
    );
}
