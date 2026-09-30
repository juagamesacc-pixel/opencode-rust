// source: packages/tui/src/context/args.tsx (16 lines, v1.18.30)
// 1:1 port — the context value is the props struct itself.

#![allow(dead_code)]

/// Mirrors `Args`.
#[derive(Debug, Clone, Default)]
pub struct Args {
    pub model: Option<String>,
    pub agent: Option<String>,
    pub prompt: Option<String>,
    pub continue_session: bool,
    pub session_id: Option<String>,
    pub fork: bool,
    pub auto: bool,
}
