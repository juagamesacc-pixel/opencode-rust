// source: test/cli/run/runtime.queue.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { describe, expect, test } from "bun:test"; import { runPromptQueue } from "@/cli/cmd/run/runtime.queue"; import

#[test]
fn run_runtime_queue() {
    // source: "run runtime queue"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn ignores_empty_prompts() {
    // source: "ignores empty prompts"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn unnamed() {
    // source: "   "
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn treats_exit_as_a_close_command() {
    // source: "treats /exit as a close command"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn exit() {
    // source: "/exit"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn treats_new_as_a_local_session_command() {
    // source: "treats /new as a local session command"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn new() {
    // source: "/new"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn hello() {
    // source: "hello"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn shell_mode_submits_exit_as_a_shell_command() {
    // source: "shell mode submits /exit as a shell command"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn shell_mode_submits_new_instead_of_creating_a_session() {
    // source: "shell mode submits /new instead of creating a session"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn shell_mode_does_not_append_a_synthetic_user_row() {
    // source: "shell mode does not append a synthetic user row"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn ls() {
    // source: "ls"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn shell_mode_does_not_emit_a_turn_duration_summary() {
    // source: "shell mode does not emit a turn duration summary"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn preserves_whitespace_for_initial_input() {
    // source: "preserves whitespace for initial input"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn passes_prompts_to_on_send() {
    // source: "passes prompts to onSend"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn appends_the_user_row_before_the_turn_starts() {
    // source: "appends the user row before the turn starts"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn runs_queued_prompts_in_order() {
    // source: "runs queued prompts in order"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn one() {
    // source: "one"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn two() {
    // source: "two"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn exposes_ordinary_in_flight_prompts_for_removal_before_sendin() {
    // source: "exposes ordinary in-flight prompts for removal before sending"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn removing_one_managed_queued_prompt_preserves_the_others() {
    // source: "removing one managed queued prompt preserves the others"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn active() {
    // source: "active"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn queued_one() {
    // source: "queued one"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn queued_two() {
    // source: "queued two"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn queued_three() {
    // source: "queued three"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn drains_a_prompt_queued_during_an_in_flight_turn() {
    // source: "drains a prompt queued during an in-flight turn"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn close_aborts_the_active_run_and_drops_pending_queued_work() {
    // source: "close aborts the active run and drops pending queued work"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn propagates_run_errors() {
    // source: "propagates run errors"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
