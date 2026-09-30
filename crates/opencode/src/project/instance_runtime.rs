// source: src/project/instance-runtime.ts — exports: load,
// disposeInstance, disposeAllInstances, reloadInstance, InstanceRuntime
// (Promise/ALS bridge over InstanceStore via AppRuntime.runPromise; deletion
// note preserved in source comment).

/// source: bridge fns — verbatim names/order.
pub const BRIDGE_FNS: &[&str] = &[
    "load",
    "disposeInstance",
    "disposeAllInstances",
    "reloadInstance",
];
