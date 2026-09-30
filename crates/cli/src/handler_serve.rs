//! Port of `packages/cli/src/commands/handlers/serve.ts` (v1.18.30 @3104c14).
//!
//! Source exports: default handler for `Commands.commands.serve`
//! (`Runtime.handler(...)`, lines 15-28); internal `listen` (lines 30-37)
//! and `bind` (lines 39-46).
//!
//! 1:1 notes:
//! - Handler flow: `Effect.scoped` -> `Daemon.Service` -> `listen(hostname,
//!   port, password)` -> if `input.register` then `daemon.register(address)`
//!   -> `console.log("server listening on {address}")` -> `Effect.never`
//!   (runs forever).
//! - `listen`: when `port` is Some -> `bind` once; when None -> try 4096
//!   upward, on failure `next(port + 1)` unless port is 65535 (then fail).
//! - `bind` layers (recorded for the server-crate integration):
//!   `HttpRouter.serve(createRoutes(password), { disableListenLog: true,
//!   disableLogger: true })` + `NodeHttpServer.layer(port, host)` +
//!   `AppNodeBuilder.build(LayerNode.group([Credential.node,
//!   PermissionSaved.node]))`, returning the bound `HttpServer` address.
//! - Log line verbatim: `"server listening on {address}"`.

/// First port tried when `--port` is absent (source line 36: `next(4096)`).
pub const FIRST_PORT: u16 = 4096;
/// Last port tried (source line 34: `port === 65_535 ? fail : next`).
pub const LAST_PORT: u16 = 65535;

/// Port of the log line (line 23).
pub fn listening_message(address: &str) -> String {
    format!("server listening on {address}")
}

/// Port of `listen` port-selection (lines 30-37): the sequence of ports to
/// try. `Some(port)` -> exactly one attempt; `None` -> 4096..=65535.
pub fn ports_to_try(port: Option<u16>) -> Vec<u16> {
    match port {
        Some(p) => vec![p],
        None => (FIRST_PORT..=LAST_PORT).collect(),
    }
}
