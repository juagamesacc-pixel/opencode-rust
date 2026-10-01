# Blocked — Assignment 02 missing module decls

branch: agent-2/feature-plugins
task: Assignment 02 (builtins, home/*, sidebar/*, system/*)
status: blocked
files changed: (none yet — implementation not started)
verify: (not run yet)
needs leader: add 3 decls to crates/tui/src/feature_plugins/mod.rs (or grant permission)

## What I found

`crates/tui/src/feature_plugins/mod.rs` declares ONLY `builtins`. The
`home/`, `sidebar/`, `system/` subdirectories exist with complete inner
barrels (`home/mod.rs`, `sidebar/mod.rs`, `system/mod.rs` all declare
their files), but without top-level `pub mod home; pub mod sidebar;
pub mod system;` the 16 files under them are orphaned — never compiled,
never clippy/test-checked.

Per Assignment 02 ("do NOT edit mod.rs; if a decl is missing, stop and
report"), I am NOT editing it. Request: leader adds the 3 decls (+3
re-export lines matching the existing barrel style), pushes to master,
and I rebase — or explicitly grants a one-time exception and I add
exactly those 6 lines myself with nothing else touched.

Implementation of the 17 files is ready to start the moment the decls
land (sources inventoried; Group A read). No other blocker: all seams
are my lane-1 port + listed master seams.
