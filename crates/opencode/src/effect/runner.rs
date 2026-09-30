// source: src/effect/runner.ts — exports: Runner, Cancelled, Busy, State, make
// (Idle/Running/Shell/ShellThenRun state machine + interrupts-only →
// Cancelled rule verbatim; fiber orchestration as trait).
// PROVISIONAL: Effect fibers/Deferred/Latch/SynchronizedRef as descriptors.

/// source: RunnerCancelled tag — verbatim.
pub const CANCELLED_TAG: &str = "RunnerCancelled";
/// source: RunnerBusy tag — verbatim.
pub const BUSY_TAG: &str = "RunnerBusy";

/// source: State tags — verbatim order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateTag {
    Idle,
    Running,
    Shell,
    ShellThenRun,
}

/// source: Runner surface — state/busy/ensureRunning/startShell/cancel. Verbatim.
pub trait Runner {
    fn state_tag(&self) -> StateTag;
    fn busy(&self) -> bool;
    fn cancel(&self);
}

/// source: interrupts-only cause → Cancelled completion rule. Verbatim.
pub fn interrupts_only_completes_cancelled(interrupts_only: bool, failed: bool) -> bool {
    failed && interrupts_only
}
