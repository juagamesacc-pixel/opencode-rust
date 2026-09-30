// source: src/project/bootstrap.ts — exports: Service, Interface
// (re-exported), node, InstanceBootstrap
// (yield-deps-at-init pattern + run order config→plugin→[lsp, shareNext,
// format, vcs, snapshot, project] verbatim; spans verbatim).

/// source: run order — config.get, plugin.init, then unbounded forEach over
/// [lsp, shareNext, format, vcs, snapshot, project] with init-failure →
/// logWarning "init failed". Verbatim.
pub const INIT_ORDER: &[&str] = &["lsp", "shareNext", "format", "vcs", "snapshot", "project"];

/// source: "bootstrapping" { directory } — verbatim.
pub const LOG_BOOTSTRAPPING: &str = "bootstrapping";
/// source: "init failed" — verbatim.
pub const LOG_INIT_FAILED: &str = "init failed";
/// source: spans "InstanceBootstrap" / "InstanceBootstrap.init" — verbatim.
pub const SPAN: &str = "InstanceBootstrap";
pub const SPAN_INIT: &str = "InstanceBootstrap.init";

/// source: node deps — verbatim order.
pub const NODE_DEPS: &[&str] = &[
    "@/config/config.Config",
    "@/format.Format",
    "@/lsp/lsp.LSP",
    "@/plugin.Plugin",
    "@/project/project.Project",
    "@/share/share-next.ShareNext",
    "@/snapshot.Snapshot",
    "@/project/vcs.Vcs",
];
