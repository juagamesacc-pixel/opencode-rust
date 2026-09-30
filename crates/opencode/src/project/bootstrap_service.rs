// source: src/project/bootstrap-service.ts — exports: Interface, Service,
// InstanceBootstrap ("@opencode/InstanceBootstrap", run only).
pub const SERVICE_ID: &str = "@opencode/InstanceBootstrap";

/// source: Interface — run only, verbatim.
pub trait Interface {
    fn run(&self);
}
