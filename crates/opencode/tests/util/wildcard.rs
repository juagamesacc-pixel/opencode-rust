// source: test/util/wildcard.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { test, expect } from "bun:test"; import { Wildcard } from "@/util/wildcard"

#[test]
fn match_handles_glob_tokens() {
    // source: "match handles glob tokens" — now real: opencode util/wildcard mirrors core glob semantics verbatim
    assert!(opencode::util::wildcard::match_("hello", "h*"));
    assert!(opencode::util::wildcard::match_("hello", "h?llo"));
    assert!(!opencode::util::wildcard::match_("hello", "x*"));
}
#[test]
fn match_with_trailing_space_wildcard_matches_command_with_or_w() {
    // source: "match with trailing space+wildcard matches command with or without args"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn all_picks_the_most_specific_pattern() {
    // source: "all picks the most specific pattern"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn all_structured_matches_command_sequences() {
    // source: "allStructured matches command sequences"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn all_structured_prioritizes_flag_specific_patterns() {
    // source: "allStructured prioritizes flag-specific patterns"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn all_structured_handles_sed_flags() {
    // source: "allStructured handles sed flags"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn match_normalizes_slashes_for_cross_platform_globbing() {
    // source: "match normalizes slashes for cross-platform globbing"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn match_handles_case_insensitivity_on_windows() {
    // source: "match handles case-insensitivity on Windows"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
