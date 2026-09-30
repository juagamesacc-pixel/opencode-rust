// source: src/effect/run-service.ts — exports: attachWith, attach, makeRuntime
// (4-branch ref combination + 5-method runtime surface verbatim).
// PROVISIONAL: ManagedRuntime/memoMap/Observability modelled as descriptors.

/// source: attachWith() 4 branches — verbatim.
pub const ATTACH_BRANCHES: &[&str] = &["none", "workspace-only", "instance-only", "both"];

/// source: makeRuntime surface — runSync/runPromiseExit/runPromise/runFork/
/// runCallback, verbatim method order.
pub const RUNTIME_METHODS: &[&str] = &[
    "runSync",
    "runPromiseExit",
    "runPromise",
    "runFork",
    "runCallback",
];
