// source: src/effect/bootstrap-runtime.ts — exports: BootstrapLayer,
// BootstrapRuntime (7-node group verbatim order + Observability.provide).

/// source: bootstrap group — verbatim node order.
pub const BOOTSTRAP_NODES: &[&str] = &[
    "@/config/config.Config",
    "@/plugin.Plugin",
    "@/share/share-next.ShareNext",
    "@/format.Format",
    "@/lsp/lsp.LSP",
    "@/project/vcs.Vcs",
    "@/snapshot.Snapshot",
];
