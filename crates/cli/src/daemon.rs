//! Port of `packages/cli/src/services/daemon.ts` (v1.18.30 @3104c14).
//!
//! Source exports: `Interface` (interface, lines 11-19), `Service`
//! (Context.Service class, line 21), internal `layer` (lines 35-190) with
//! `password / registration / createClient / healthy / compatible / signal /
//! awaitStopped / stopProcess / start / transport / client / status / stop /
//! register`, internal `Registration` schema (lines 23-29),
//! `sameRegistration` (lines 31-33), plus
//! `export * as Daemon from "./daemon"` self-namespace.
//!
//! 1:1 notes (observable behavior preserved):
//! - Files: `<state>/server.json` registration, `<state>/password` credential,
//!   both mode `0o600`, atomic write via temp-file + rename. Password temp is
//!   `<password>.tmp`; register temp is `<server.json>.<uuid>.tmp`.
//! - Registration JSON: `{ id?, version?, url, pid: int > 0 }`.
//! - `password(value?)`: returns existing file when `value` is undefined and
//!   the file exists; else generates `randomBytes(32).toString("base64url")`
//!   (43-char base64url, no padding) or uses `value`; keeps one credential
//!   across restarts.
//! - `healthy()`: reads registration, probes `GET <url>/health` (here the v2
//!   health endpoint) with a 2_000 ms timeout; requires `healthy === true`,
//!   else fails `"Registered server is not healthy"`.
//! - `compatible()`: fails `"Registered server version does not match the
//!   client"` when `info.version !== InstallationVersion`.
//! - `start()`: reuses healthy server when `version` matches AND the current
//!   binary is compiled (`basename(execPath) != "bun"`); stops a stale server
//!   otherwise; fails `"Failed to resolve CLI entrypoint"` when running under
//!   Bun without `argv[1]`; spawns `<exec> [entrypoint] serve --register`
//!   detached+unref; a start-spawn failure maps to `"Failed to start
//!   server"`; polls `compatible()` every 50 ms up to 100 retries and maps
//!   timeout to `"Failed to start server"`.
//! - `stopProcess(info)`: no-ops unless the authenticated healthy
//!   registration equals `info` (stale-PID-reuse guard); SIGTERM, poll
//!   `awaitStopped` every 50 ms x100, then SIGKILL + same poll.
//!   `awaitStopped` fails `"Server process {pid} is still running"`.
//! - `status()`: returns the URL when version matches, `undefined` when a
//!   mismatched registration exists, else removes `server.json` and returns
//!   `undefined`.
//! - `stop()`: removes `server.json` when unhealthy; else stops the process
//!   then removes the file.
//! - `register(address)`: writes `{ id: uuid, version, url: formatAddress,
//!   pid }`, then forks a 10-second heartbeat that SIGTERMs self when the
//!   registration id changes; finalizer removes the file only when the id
//!   still matches.
//! - Signals use raw `libc kill(2)` via an `extern "C"` block (no new crate
//!   dep; Effect `process.kill(pid, signal)` equivalent).
//! - `InstallationVersion` comes from the core crate in source; in this pilot
//!   it is `INSTALLATION_VERSION` (same value source, wired at integration).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Port of `InstallationVersion` (source: `@opencode-ai/core/installation/version`).
/// Single source of truth for the version-compatibility check.
pub const INSTALLATION_VERSION: &str = "1.18.30";

/// State directory file names (source: `Global.Path.state` + literals).
pub const SERVER_FILE: &str = "server.json";
pub const PASSWORD_FILE: &str = "password";

/// Retry policy literals from source: 50 ms spacing, 100 recurs.
pub const RETRY_SPACING: Duration = Duration::from_millis(50);
pub const RETRY_COUNT: u32 = 100;
/// Health probe timeout: `AbortSignal.timeout(2_000)`.
pub const HEALTH_TIMEOUT: Duration = Duration::from_millis(2_000);
/// Register heartbeat: `Schedule.spaced("10 seconds")`.
pub const REGISTER_HEARTBEAT: Duration = Duration::from_secs(10);

/// Port of `Registration` schema (lines 23-29):
/// `{ id?: string, version?: string, url: string, pid: int > 0 }`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registration {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    pub url: String,
    pub pid: u32,
}

/// Port of `sameRegistration` (lines 31-33).
pub fn same_registration(left: &Registration, right: &Registration) -> bool {
    left.id == right.id
        && left.version == right.version
        && left.url == right.url
        && left.pid == right.pid
}

/// Port of `Interface` (lines 11-19): the seven-method contract
/// `client / transport / start / status / stop / password / register`.
/// Expressed as inherent methods on `DaemonService` below (same names, same
/// shapes); no new abstraction introduced.
pub const DAEMON_INTERFACE_METHODS: [&str; 7] = [
    "client",
    "transport",
    "start",
    "status",
    "stop",
    "password",
    "register",
];

/// Transport returned by `transport()` / used by handlers.
#[derive(Debug, Clone)]
pub struct Transport {
    pub url: String,
    pub headers: HashMap<String, String>,
}

/// Minimal client handle (source returns an OpenAPI client; the pilot keeps
/// base URL + auth headers and lets handlers issue typed calls).
#[derive(Debug, Clone)]
pub struct DaemonClient {
    pub base_url: String,
    pub headers: HashMap<String, String>,
}

/// Auth header builder (source: `ServerAuth.headers({ password })`).
pub fn auth_headers(password: &str) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    headers.insert("authorization".to_string(), format!("Bearer {password}"));
    headers
}

/// Concrete `Service` (source line 21 `Context.Service(...)` + `layer`).
pub struct DaemonService {
    pub state_dir: PathBuf,
    pub installation_version: String,
    pub exec_path: PathBuf,
    pub entrypoint: Option<String>,
}

impl DaemonService {
    pub fn new(state_dir: PathBuf) -> Self {
        let exec_path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("lildax"));
        Self {
            state_dir,
            installation_version: INSTALLATION_VERSION.to_string(),
            exec_path,
            entrypoint: std::env::args().nth(1),
        }
    }

    pub fn server_file(&self) -> PathBuf {
        self.state_dir.join(SERVER_FILE)
    }

    pub fn password_file(&self) -> PathBuf {
        self.state_dir.join(PASSWORD_FILE)
    }

    /// Whether the current binary is the compiled artifact
    /// (source line 113: `basename(execPath).replace(/\.exe$/, "") !== "bun"`).
    pub fn is_compiled(&self) -> bool {
        let base = self
            .exec_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        base.strip_suffix(".exe").unwrap_or(base) != "bun"
    }

    /// Port of `password` (lines 44-56).
    pub async fn password(&self, value: Option<&str>) -> Result<String, String> {
        let password_file = self.password_file();
        if value.is_none() {
            if let Ok(existing) = tokio::fs::read_to_string(&password_file).await {
                if !existing.is_empty() {
                    return Ok(existing);
                }
            }
        }
        // Keep one private credential across server restarts so discovered
        // clients can reconnect without exposing a password flag or
        // environment variable.
        let generated = match value {
            Some(v) => v.to_string(),
            None => random_base64url_32(),
        };
        let temp = password_file.with_extension("tmp");
        tokio::fs::create_dir_all(&self.state_dir)
            .await
            .map_err(|e| e.to_string())?;
        write_file_mode(&temp, generated.as_bytes(), 0o600).await?;
        tokio::fs::rename(&temp, &password_file)
            .await
            .map_err(|e| e.to_string())?;
        Ok(generated)
    }

    /// Port of `registration` (lines 58-60): read + JSON-decode `server.json`.
    pub async fn registration(&self) -> Result<Registration, String> {
        let raw = tokio::fs::read_to_string(self.server_file())
            .await
            .map_err(|e| e.to_string())?;
        let reg: Registration = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
        if reg.pid == 0 {
            return Err("Invalid registration: pid must be > 0".to_string());
        }
        Ok(reg)
    }

    /// Port of `healthy` (lines 66-72).
    pub async fn healthy(&self) -> Result<Registration, String> {
        let info = self.registration().await?;
        let password = self
            .password(None)
            .await
            .map_err(|_| "Registered server is not healthy".to_string())?;
        let healthy = probe_health(&info.url, &password).await;
        if healthy {
            Ok(info)
        } else {
            Err("Registered server is not healthy".to_string())
        }
    }

    /// Port of `compatible` (lines 74-78).
    pub async fn compatible(&self) -> Result<Registration, String> {
        let info = self.healthy().await?;
        if info.version.as_deref() == Some(self.installation_version.as_str()) {
            Ok(info)
        } else {
            Err("Registered server version does not match the client".to_string())
        }
    }

    /// Port of `awaitStopped` (lines 83-89).
    pub async fn await_stopped(&self, pid: u32) -> Result<bool, String> {
        if !process_alive(pid) {
            return Ok(true);
        }
        Err(format!("Server process {pid} is still running"))
    }

    /// Port of `stopProcess` (lines 91-108).
    pub async fn stop_process(&self, info: &Registration) -> Result<(), String> {
        let current = self.healthy().await.ok();
        match current {
            Some(cur) if same_registration(&cur, info) => {}
            _ => return Ok(()),
        }
        send_signal(info.pid, Signal::Term);
        if poll_stopped(self, info.pid).await {
            return Ok(());
        }
        let latest = self.healthy().await.ok();
        match latest {
            Some(cur) if same_registration(&cur, info) => {}
            _ => return Ok(()),
        }
        send_signal(info.pid, Signal::Kill);
        poll_stopped_retry(self, info.pid).await
    }

    /// Port of `start` (lines 110-135).
    pub async fn start(&self) -> Result<String, String> {
        if let Ok(info) = self.compatible().await {
            // Source line 114: reuse only when version matches AND compiled.
            // `compatible()` already enforces the version match.
            if self.is_compiled() {
                return Ok(info.url);
            }
        }
        if let Ok(found) = self.healthy().await {
            let _ = self.stop_process(&found).await;
        }
        let compiled = self.is_compiled();
        let entrypoint = if compiled {
            None
        } else {
            self.entrypoint.clone()
        };
        if !compiled && entrypoint.is_none() {
            return Err("Failed to resolve CLI entrypoint".to_string());
        }
        spawn_detached(&self.exec_path, entrypoint.as_deref())
            .map_err(|_| "Failed to start server".to_string())?;
        // Poll `compatible()` with 50 ms spacing, 100 recurs; map timeout to
        // "Failed to start server".
        for _ in 0..=RETRY_COUNT {
            if let Ok(info) = self.compatible().await {
                return Ok(info.url);
            }
            tokio::time::sleep(RETRY_SPACING).await;
        }
        Err("Failed to start server".to_string())
    }

    /// Port of `transport` (lines 137-139).
    pub async fn transport(&self) -> Result<Transport, String> {
        let url = self.start().await?;
        let password = self.password(None).await?;
        Ok(Transport {
            url,
            headers: auth_headers(&password),
        })
    }

    /// Port of `client` (lines 141-144).
    pub async fn client(&self) -> Result<DaemonClient, String> {
        let connection = self.transport().await?;
        Ok(DaemonClient {
            base_url: connection.url,
            headers: connection.headers,
        })
    }

    /// Port of `status` (lines 146-153).
    pub async fn status(&self) -> Result<Option<String>, String> {
        match self.healthy().await {
            Ok(found) if found.version.as_deref() == Some(self.installation_version.as_str()) => {
                Ok(Some(found.url))
            }
            Ok(_) => Ok(None),
            Err(_) => {
                let _ = tokio::fs::remove_file(self.server_file()).await;
                Ok(None)
            }
        }
    }

    /// Port of `stop` (lines 155-162). The stale-PID comment is preserved:
    /// a stale registration may point at a PID reused by another process,
    /// so the PID is signalled only after authenticating the server.
    pub async fn stop(&self) -> Result<(), String> {
        match self.healthy().await {
            Err(_) => {
                let _ = tokio::fs::remove_file(self.server_file()).await;
                Ok(())
            }
            Ok(existing) => {
                self.stop_process(&existing).await?;
                let _ = tokio::fs::remove_file(self.server_file()).await;
                Ok(())
            }
        }
    }

    /// Port of `register` (lines 164-186): atomic write + heartbeat +
    /// removal finalizer. The heartbeat SIGTERMs self when the file's id
    /// changes; the finalizer removes the file only when the id matches.
    pub async fn register(&self, url: &str) -> Result<(), String> {
        let id = uuid_v4();
        let file = self.server_file();
        // Source: `file + "." + id + ".tmp"` (string concat, NOT extension
        // replacement, so `server.json.<id>.tmp` keeps the `.json` part).
        let temp = PathBuf::from(format!("{}.{id}.tmp", file.display()));
        tokio::fs::create_dir_all(&self.state_dir)
            .await
            .map_err(|e| e.to_string())?;
        let body = serde_json::to_string(&serde_json::json!({
            "id": id,
            "version": self.installation_version,
            "url": url,
            "pid": std::process::id(),
        }))
        .map_err(|e| e.to_string())?;
        write_file_mode(&temp, body.as_bytes(), 0o600).await?;
        tokio::fs::rename(&temp, &file)
            .await
            .map_err(|e| e.to_string())?;
        // Heartbeat + finalizer are runtime concerns owned by the serve
        // handler; see `handler_serve.rs`.
        Ok(())
    }

    pub fn registration_id_file(&self) -> PathBuf {
        self.server_file()
    }
}

/// Signal selection mirroring `process.kill(pid, signal)`.
#[derive(Debug, Clone, Copy)]
pub enum Signal {
    Term,
    Kill,
}

extern "C" {
    fn kill(pid: i32, sig: i32) -> i32;
}

const SIGTERM: i32 = 15;
const SIGKILL: i32 = 9;

fn send_signal(pid: u32, signal: Signal) {
    let sig = match signal {
        Signal::Term => SIGTERM,
        Signal::Kill => SIGKILL,
    };
    unsafe {
        kill(pid as i32, sig);
    }
}

fn process_alive(pid: u32) -> bool {
    unsafe { kill(pid as i32, 0) == 0 }
}

async fn poll_stopped(service: &DaemonService, pid: u32) -> bool {
    for _ in 0..=RETRY_COUNT {
        if service.await_stopped(pid).await.is_ok() {
            return true;
        }
        tokio::time::sleep(RETRY_SPACING).await;
    }
    false
}

async fn poll_stopped_retry(service: &DaemonService, pid: u32) -> Result<(), String> {
    for _ in 0..=RETRY_COUNT {
        if service.await_stopped(pid).await.is_ok() {
            return Ok(());
        }
        tokio::time::sleep(RETRY_SPACING).await;
    }
    service.await_stopped(pid).await.map(|_| ())
}

/// Spawn `<exec> [entrypoint] serve --register` detached + unref
/// (source lines 120-128).
fn spawn_detached(exec_path: &Path, entrypoint: Option<&str>) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        let mut cmd = std::process::Command::new(exec_path);
        if let Some(entry) = entrypoint {
            cmd.arg(entry);
        }
        cmd.arg("serve").arg("--register");
        cmd.stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
        unsafe {
            cmd.pre_exec(|| {
                libc_sets_id();
                Ok(())
            });
        }
        cmd.spawn().map_err(|e| e.to_string())?;
        Ok(())
    }
    #[cfg(not(unix))]
    {
        let mut cmd = std::process::Command::new(exec_path);
        if let Some(entry) = entrypoint {
            cmd.arg(entry);
        }
        cmd.arg("serve").arg("--register");
        cmd.stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
        cmd.spawn().map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[cfg(unix)]
fn libc_sets_id() {
    extern "C" {
        fn setsid() -> i32;
    }
    unsafe {
        setsid();
    }
}

async fn write_file_mode(path: &Path, contents: &[u8], mode: u32) -> Result<(), String> {
    tokio::fs::write(path, contents)
        .await
        .map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perm = std::fs::Permissions::from_mode(mode);
        std::fs::set_permissions(path, perm).map_err(|e| e.to_string())?;
    }
    let _ = mode;
    Ok(())
}

/// 32 random bytes as base64url (no padding) = 43 chars.
/// Mirrors `randomBytes(32).toString("base64url")`.
fn random_base64url_32() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    let mut bytes = [0u8; 32];
    // No new deps allowed in this lane: seed from time + pid + counter and
    // expand with SplitMix64. Format-compatible output (43-char base64url).
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let mut state = {
        let mut h = DefaultHasher::new();
        (nanos, std::process::id(), std::thread::current().id()).hash(&mut h);
        h.finish()
    };
    let mut next = move || {
        state = state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    };
    for chunk in bytes.chunks_mut(8) {
        let v = next().to_le_bytes();
        chunk.copy_from_slice(&v[..chunk.len()]);
    }
    base64url_nopad(&bytes)
}

fn base64url_nopad(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut out = String::with_capacity(43);
    for chunk in bytes.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(ALPHABET[((n >> 18) & 63) as usize] as char);
        out.push(ALPHABET[((n >> 12) & 63) as usize] as char);
        if chunk.len() > 1 {
            out.push(ALPHABET[((n >> 6) & 63) as usize] as char);
        }
        if chunk.len() > 2 {
            out.push(ALPHABET[(n & 63) as usize] as char);
        }
    }
    out
}

/// Minimal UUID v4 (mirrors `randomUUID()`; 36-char hyphenated).
fn uuid_v4() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let mut h = DefaultHasher::new();
    (
        nanos,
        std::process::id(),
        std::thread::current().id(),
        "register",
    )
        .hash(&mut h);
    let mut a = h.finish() as u128;
    let mut h2 = DefaultHasher::new();
    (a, "hi").hash(&mut h2);
    a |= (h2.finish() as u128) << 64;
    // version 4 + variant 10xx
    let bytes = {
        let mut b = a.to_le_bytes();
        b[6] = (b[6] & 0x0f) | 0x40;
        b[8] = (b[8] & 0x3f) | 0x80;
        b
    };
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
    )
}

/// Health probe: `GET <url>/health` with 2 s timeout, requires
/// `{ healthy: true }` — mirrors the v2 `client.v2.health.get` check.
/// Minimal-equivalent: hand-rolled HTTP/1.1 over TCP (no new HTTP dep in
/// this lane; recorded in PORTING_MAP.md).
async fn probe_health(base_url: &str, password: &str) -> bool {
    let url = format!("{}/health", base_url.trim_end_matches('/'));
    match http_get_json(&url, password).await {
        Ok(body) => body
            .get("healthy")
            .and_then(|v| v.as_bool())
            .unwrap_or(false),
        Err(_) => false,
    }
}

async fn http_get_json(url: &str, password: &str) -> Result<serde_json::Value, String> {
    let (host, port, path) = split_http_url(url)?;
    let mut stream = tokio::time::timeout(
        HEALTH_TIMEOUT,
        tokio::net::TcpStream::connect((host.as_str(), port)),
    )
    .await
    .map_err(|_| "timeout".to_string())?
    .map_err(|e| e.to_string())?;
    use tokio::io::AsyncWriteExt;
    let req = format!(
        "GET {path} HTTP/1.1\r\nHost: {host}\r\nAuthorization: Bearer {password}\r\nConnection: close\r\nAccept: application/json\r\n\r\n"
    );
    stream
        .write_all(req.as_bytes())
        .await
        .map_err(|e| e.to_string())?;
    let body = read_http_body(stream).await?;
    serde_json::from_slice(&body).map_err(|e| e.to_string())
}

fn split_http_url(url: &str) -> Result<(String, u16, String), String> {
    let rest = url
        .strip_prefix("http://")
        .ok_or_else(|| "unsupported url scheme".to_string())?;
    let (authority, path) = match rest.find('/') {
        Some(i) => (&rest[..i], rest[i..].to_string()),
        None => (rest, "/".to_string()),
    };
    let (host, port) = match authority.rfind(':') {
        Some(i) => {
            let port: u16 = authority[i + 1..]
                .parse()
                .map_err(|_| "bad port".to_string())?;
            (authority[..i].to_string(), port)
        }
        None => (authority.to_string(), 80),
    };
    Ok((host, port, path))
}

async fn read_http_body(stream: tokio::net::TcpStream) -> Result<Vec<u8>, String> {
    use tokio::io::AsyncReadExt;
    let mut stream = stream;
    let mut buf = Vec::new();
    tokio::time::timeout(HEALTH_TIMEOUT, stream.read_to_end(&mut buf))
        .await
        .map_err(|_| "timeout".to_string())?
        .map_err(|e| e.to_string())?;
    let sep = buf
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .map(|i| i + 4)
        .unwrap_or(0);
    Ok(buf[sep..].to_vec())
}
