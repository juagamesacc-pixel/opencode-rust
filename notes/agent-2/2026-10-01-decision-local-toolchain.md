# Decision — local toolchain override (leader order)

branch: agent-2/plugin-api
task: Assignment 01 verification method
status: in-progress
files changed: (none — env setup)
verify: (pending — local gate after install)
needs leader: (none — order received)

Leader ordered (2026-10-01 chat): agent-2 must NOT use the CI workflow;
install Rust on this system and build/test with system rust.

Accepted as explicit download consent (covers AGENTS.md rule outside the
12–6 AM IST free window). Deviation from PORTING_MAP "no local toolchain,
verify in CI" policy and from the lane plan §5 (CI-driven gate) is by
leader order, not silent: verify gate is now the guidelines §1/§5 full
gate run locally (`cargo fmt --check && cargo clippy --all-targets --
-D warnings && cargo test`). CI run 36799229130 (sha 309f4fe) is
superseded for verification but left untouched (append-only, no cancel).
