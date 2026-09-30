// source: src/acp/agent.ts — exports: init, Agent, ACP
// PROVISIONAL pending @agentclientprotocol/sdk + @opencode-ai/sdk/v2 +
// ./service + ./error: 14-method ACPAgent surface + run() defect-fallback
// rule verbatim.

/// source: ACPAgent method table — verbatim names/order.
pub const AGENT_METHODS: &[&str] = &[
    "initialize",
    "authenticate",
    "newSession",
    "loadSession",
    "listSessions",
    "resumeSession",
    "closeSession",
    "unstable_forkSession",
    "setSessionConfigOption",
    "setSessionMode",
    "unstable_setSessionModel",
    "prompt",
    "cancel",
];

/// source: run() — mapError(toRequestError); defect → RequestError passthrough
/// else toRequestError(fromUnknownDefect). Verbatim rule.
pub const RUN_MAP_ERROR: &str = "toRequestError";
pub const RUN_DEFECT_FALLBACK: &str = "fromUnknownDefect";
