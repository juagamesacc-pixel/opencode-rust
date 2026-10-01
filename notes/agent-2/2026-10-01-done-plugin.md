# Verify — Assignment 01 local gate (system rust 1.98.1)

branch: agent-2/plugin-api
task: Assignment 01 (plugin/api, slots, command_shim, adapters, runtime)
status: done (lane green; one pre-existing out-of-lane failure logged below)
files changed: crates/tui/src/plugin/{api,slots,command_shim,runtime,adapters}.rs (stubs → full 1:1 port, ~1620 lines)
verify: cargo fmt --check CLEAN; cargo clippy --all-targets -- -D warnings CLEAN (workspace); cargo test -p tui GREEN (75 unit + 31 integration, 0 failed); workspace 280 suites ok
needs leader: merge (--no-ff) after review; http-recorder doctest decision (leave to owner lane)

## Parity self-audit (spot-checks, all verbatim)
- api: last-wins get (.at(-1)), key-scoped unregister, name deleted when empty, revision bump on register/unregister; lifecycle signal starts false, onDispose stores + returns noop.
- slots: non-host register → noop unregister; `[tui.slot] plugin error` tag + plugin/slot/phase/source/message order; dispose/clear reset view.
- command-shim: COMMAND_PALETTE_SHOW="command.palette.show"; 3 warnOnce pairs verbatim; `namespace:"palette"` + field order; fallback `{key,cmd,desc:title}` vs mapped `{...binding,cmd,desc:binding.desc??title}`; trigger/show dispatch verbatim.
- runtime: emptyCommands false/false/false + install `{ok:false,"Plugin runtime is not available."}`; update applies present fields only; usePluginRuntime outside provider → "usePluginRuntime must be used within PluginRuntimeProvider".
- adapters: home/session/plugin navigate branches (bad sessionID → return); routeCurrent 3 shapes; session.diff drops file-undefined items; mcp sorted + error-only-when-failed; slots.register/theme.install/ plugins.install error strings verbatim; toast variant ?? "info".

## Pre-existing failure (NOT this lane, NOT touched)
- `cargo test --workspace`: exactly 1 failing suite — Doc-tests http_recorder (8 failed, TS-flavored snippets in doc comments: "expected item, found keyword `return`").
- Evidence: `git diff origin/master --stat` shows ONLY crates/tui/src/plugin/* + notes/agent-2/*; crates/http-recorder/Cargo.toml has NO tui dependency (independent crate). Untouched, so failure pre-exists on master. Left for owner lane per guidelines §2/§7 (stop + report instead of editing others' files).
