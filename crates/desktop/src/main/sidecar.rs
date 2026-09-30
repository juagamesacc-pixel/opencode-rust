//! Rust port of `src/main/sidecar.ts` (opencode v1.18.30).
//!
//! Fully ported: `SidecarCommand` parsing (`parseCommand`), error
//! serialization (`serializeError`), the sidecar env overlay
//! (`prepareSidecarEnv`), and the loopback `NO_PROXY` upsert
//! (`ensureLoopbackNoProxy`) — the latter two over an injected env map
//! mirroring `process.env`. PROVISIONAL: the parent-port message loop,
//! `start`/`stop` (the `virtual:opencode-server` import), system
//! certificates, and the global-proxy hook (node:http/node:tls bindings).
//!
//! Original file: `packages/desktop/src/main/sidecar.ts`

use std::collections::HashMap;

pub struct StartCommand {
    pub hostname: String,
    pub port: u16,
    pub password: String,
    pub user_data_path: String,
}

pub enum SidecarCommand {
    Start(StartCommand),
    Stop,
}

pub enum SidecarMessage {
    Ready,
    Stopped,
    Error {
        message: String,
        stack: Option<String>,
    },
}

pub struct SerializedError {
    pub message: String,
    pub stack: Option<String>,
}

pub fn parse_command(value: &serde_json::Value) -> Option<SidecarCommand> {
    let command = value.as_object()?;
    let command_type = command.get("type")?.as_str()?;
    if command_type == "stop" {
        return Some(SidecarCommand::Stop);
    }
    if command_type != "start" {
        return None;
    }
    Some(SidecarCommand::Start(StartCommand {
        hostname: command.get("hostname")?.as_str()?.to_string(),
        port: command
            .get("port")?
            .as_u64()
            .and_then(|port| u16::try_from(port).ok())?,
        password: command.get("password")?.as_str()?.to_string(),
        user_data_path: command.get("userDataPath")?.as_str()?.to_string(),
    }))
}

/// Mirrors `serializeError(error)`: `Error` instances contribute
/// `message` + `stack`, anything else is stringified.
pub fn serialize_error(message: String, stack: Option<String>) -> SerializedError {
    SerializedError { message, stack }
}

/// Mirrors `prepareSidecarEnv(password, userDataPath)` (`Object.assign`
/// on `process.env`, expressed as a mutation of the passed map).
pub fn apply_sidecar_env(env: &mut HashMap<String, String>, password: &str, user_data_path: &str) {
    env.insert(
        "OPENCODE_SERVER_USERNAME".to_string(),
        "opencode".to_string(),
    );
    env.insert("OPENCODE_SERVER_PASSWORD".to_string(), password.to_string());
    if !env.contains_key("XDG_STATE_HOME") {
        env.insert("XDG_STATE_HOME".to_string(), user_data_path.to_string());
    }
}

/// Mirrors `ensureLoopbackNoProxy()` (`NO_PROXY`/`no_proxy` upsert,
/// case-insensitive dedup, append order preserved).
pub fn apply_loopback_no_proxy(env: &mut HashMap<String, String>) {
    for key in ["NO_PROXY", "no_proxy"] {
        let mut items: Vec<String> = env
            .get(key)
            .map(|value| {
                value
                    .split(',')
                    .map(|item| item.trim().to_string())
                    .filter(|item| !item.is_empty())
                    .collect()
            })
            .unwrap_or_default();
        for host in ["127.0.0.1", "localhost", "::1"] {
            if !items.iter().any(|item| item.to_ascii_lowercase() == host) {
                items.push(host.to_string());
            }
        }
        env.insert(key.to_string(), items.join(","));
    }
}

// PROVISIONAL(packages/desktop/src/main/sidecar.ts): the parent-port
// message loop (`process.parentPort`), `Server.listen` from the virtual
// `virtual:opencode-server` module (`cors: ["oc://renderer"]`), system CA
// certificates, and the global-proxy hook need runtime bindings that have
// no in-workspace equivalent. Preserved source error text,
// byte-identical: "Sidecar parent port unavailable".
pub const SIDECAR_CORS_ORIGIN: &str = "oc://renderer";
pub const SIDECAR_USERNAME: &str = "opencode";

pub fn run_sidecar_message_loop() {
    unimplemented!("utility-process parentPort + virtual:opencode-server binding (Sidecar parent port unavailable without it)")
}

#[cfg(test)]
mod tests {
    // No `src/main/sidecar.test.ts` exists in the source; the cases below
    // pin the ported pure helpers to the source's inline behavior.
    use super::*;

    #[test]
    fn parse_command_accepts_start_and_stop() {
        let start = parse_command(&serde_json::json!({
            "type": "start",
            "hostname": "127.0.0.1",
            "port": 4096,
            "password": "secret",
            "userDataPath": "/data",
        }));
        assert!(matches!(start, Some(SidecarCommand::Start(_))));
        assert!(matches!(
            parse_command(&serde_json::json!({ "type": "stop" })),
            Some(SidecarCommand::Stop)
        ));
        assert!(parse_command(&serde_json::json!({ "type": "start" })).is_none());
        assert!(parse_command(&serde_json::json!(null)).is_none());
    }

    #[test]
    fn apply_sidecar_env_sets_server_credentials() {
        let mut env = HashMap::new();
        apply_sidecar_env(&mut env, "secret", "/data");
        assert_eq!(
            env.get("OPENCODE_SERVER_USERNAME").map(String::as_str),
            Some("opencode")
        );
        assert_eq!(
            env.get("OPENCODE_SERVER_PASSWORD").map(String::as_str),
            Some("secret")
        );
        assert_eq!(env.get("XDG_STATE_HOME").map(String::as_str), Some("/data"));

        env.insert("XDG_STATE_HOME".to_string(), "/keep".to_string());
        apply_sidecar_env(&mut env, "secret", "/data");
        assert_eq!(env.get("XDG_STATE_HOME").map(String::as_str), Some("/keep"));
    }

    #[test]
    fn apply_loopback_no_proxy_upserts_both_keys() {
        let mut env =
            HashMap::from([("NO_PROXY".to_string(), "example.com, LOCALHOST".to_string())]);
        apply_loopback_no_proxy(&mut env);
        assert_eq!(
            env.get("NO_PROXY").map(String::as_str),
            Some("example.com,LOCALHOST,127.0.0.1,::1")
        );
        assert_eq!(
            env.get("no_proxy").map(String::as_str),
            Some("127.0.0.1,localhost,::1")
        );
    }
}
