// source: src/util/flock.ts — exports: FlockGlobal, Flock (setGlobal, acquire,
// withLock, effect) + WaitEvent/Wait/Options/Lease types.
//
// PROVISIONAL pending node:fs/promises, node:crypto, AbortSignal, and the
// Effect wrapper equivalents:
// - async fs/promises → std::fs (sync); `sleep` via std::thread::sleep
//   (the abortable sleep rejects pre-abort / post-sleep — loop re-checks).
// - heartbeat `setInterval` → detached thread touching mtime (utimes ≈
//   File::set_modified; only mtime is observed by staleness checks).
// - `await using` disposal → Drop, with explicit `release()` when the caller
//   wants the error propagated. NOTE: TS *never* auto-releases an abandoned
//   lease (relies on stale expiry); Rust Drop releases immediately — recorded
//   divergence (safety over leak parity).
// - `os.hostname()` → /proc/sys/kernel/hostname read (Linux).
// Key/error strings and ordering are verbatim.

use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// source: `FlockGlobal = { state: string }`.
#[derive(Clone, Debug)]
pub struct FlockGlobal {
    pub state: String,
}

static GLOBAL: Mutex<Option<FlockGlobal>> = Mutex::new(None);

/// source: `Flock.setGlobal(g)`.
pub fn set_global(g: FlockGlobal) {
    *GLOBAL.lock().unwrap() = Some(g);
}

/// source: `root()` — `path.join(global.state, "locks")`; throws
/// `"Flock global not set"` when unset.
fn root() -> Result<String, String> {
    let guard = GLOBAL.lock().unwrap();
    match &*guard {
        Some(g) => Ok(Path::new(&g.state)
            .join("locks")
            .to_string_lossy()
            .into_owned()),
        None => Err("Flock global not set".to_string()),
    }
}

/// Defaults for callers that do not provide timing options (verbatim).
const DEFAULT_STALE_MS: u64 = 60_000;
const DEFAULT_TIMEOUT_MS: u64 = 5 * 60_000;
const DEFAULT_BASE_DELAY_MS: u64 = 100;
const DEFAULT_MAX_DELAY_MS: u64 = 2_000;

/// source: `WaitEvent { key, attempt, delay, waited }`.
#[derive(Debug, Clone)]
pub struct WaitEvent {
    pub key: String,
    pub attempt: u64,
    pub delay: u64,
    pub waited: u64,
}

/// source: `Wait = (input: WaitEvent) => void | Promise<void>` — sync port.
pub type Wait<'a> = &'a dyn Fn(&WaitEvent);

/// source: `Options` — all fields optional; defaults applied in `acquire`.
/// `signal` mirrors AbortSignal (see `AbortSignal` below).
#[derive(Default)]
pub struct Options<'a> {
    pub dir: Option<String>,
    pub signal: Option<&'a AbortSignal>,
    pub stale_ms: Option<u64>,
    pub timeout_ms: Option<u64>,
    pub base_delay_ms: Option<u64>,
    pub max_delay_ms: Option<u64>,
    pub on_wait: Option<Wait<'a>>,
}

/// Minimal AbortSignal port (TS `AbortSignal`: aborted / reason /
/// throwIfAborted). Reason defaults to `"Aborted"` like `sleep`'s fallback.
#[derive(Clone, Default)]
pub struct AbortSignal {
    aborted: Arc<AtomicBool>,
    reason: Arc<Mutex<Option<String>>>,
}

impl AbortSignal {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn abort(&self) {
        self.aborted.store(true, Ordering::SeqCst);
    }

    pub fn abort_with_reason(&self, reason: impl Into<String>) {
        *self.reason.lock().unwrap() = Some(reason.into());
        self.abort();
    }

    pub fn is_aborted(&self) -> bool {
        self.aborted.load(Ordering::SeqCst)
    }

    pub fn reason(&self) -> Option<String> {
        self.reason.lock().unwrap().clone()
    }

    /// source: `signal.throwIfAborted()`.
    pub fn throw_if_aborted(&self) -> Result<(), String> {
        if self.is_aborted() {
            return Err(self.reason().unwrap_or_else(|| "Aborted".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Opts {
    stale_ms: u64,
    timeout_ms: u64,
    base_delay_ms: u64,
    max_delay_ms: u64,
}

/// source: `code(err)` — extracts the string `code` of an error.
fn error_code(e: &std::io::Error) -> Option<&'static str> {
    match e.kind() {
        ErrorKind::NotFound => Some("ENOENT"),
        ErrorKind::PermissionDenied => Some("EACCES"),
        ErrorKind::AlreadyExists => Some("EEXIST"),
        ErrorKind::NotADirectory => Some("ENOTDIR"),
        ErrorKind::DirectoryNotEmpty => Some("ENOTEMPTY"),
        _ => None,
    }
}

/// source: `sleep(ms, signal)` — rejects when already aborted (reason or
/// `"Aborted"`); a sync sleep cannot be interrupted, so aborts are observed
/// before and after the sleep (loop re-checks).
fn sleep(ms: u64, signal: Option<&AbortSignal>) -> Result<(), String> {
    if let Some(s) = signal {
        s.throw_if_aborted()?;
    }
    std::thread::sleep(Duration::from_millis(ms));
    if let Some(s) = signal {
        s.throw_if_aborted()?;
    }
    Ok(())
}

/// source: `jitter(ms)` — ±30% uniform jitter, floored at 0.
fn jitter(ms: u64) -> u64 {
    let j = (ms as f64 * 0.3).floor() as i64;
    let d = (crate::util::hash::random01() * (2 * j + 1) as f64).floor() as i64 - j;
    (ms as i64 + d).max(0) as u64
}

static START: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

/// source: `mono()` — `performance.now()` (ms since process start).
fn mono() -> f64 {
    START.get_or_init(Instant::now).elapsed().as_secs_f64() * 1000.0
}

/// source: `wall()` — `performance.timeOrigin + performance.now()` = epoch ms.
fn wall() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64() * 1000.0)
        .unwrap_or(0.0)
}

fn mtime_ms(m: &fs::Metadata) -> f64 {
    m.modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs_f64() * 1000.0)
        .unwrap_or(0.0)
}

/// source: `stats(file)` — stat, swallowing ENOENT/ENOTDIR → None.
fn stats(file: &Path) -> Result<Option<fs::Metadata>, String> {
    match fs::metadata(file) {
        Ok(m) => Ok(Some(m)),
        Err(e) => {
            let code = error_code(&e);
            if code == Some("ENOENT") || code == Some("ENOTDIR") {
                return Ok(None);
            }
            Err(e.to_string())
        }
    }
}

/// source: `mkdir(dir, { mode: 0o700 })` — non-recursive atomic create.
fn create_dir_mode(dir: &str, mode: u32) -> std::io::Result<()> {
    use std::os::unix::fs::DirBuilderExt;
    let mut b = fs::DirBuilder::new();
    b.mode(mode);
    b.create(dir)
}

fn remove_dir_all(target: &str) -> std::io::Result<()> {
    fs::remove_dir_all(target)
}

/// source: `rm(path, { recursive: true, force: true })` — ignore missing.
fn force_remove_dir_all(target: &str) {
    let _ = fs::remove_dir_all(target);
}

/// source: `stale(lockDir, heartbeatPath, metaPath, staleMs)` — newest mtime
/// of heartbeat / meta / lock dir decides; no dir at all → false.
fn stale(
    lock_dir: &str,
    heartbeat_path: &Path,
    meta_path: &Path,
    stale_ms: u64,
) -> Result<bool, String> {
    let now = wall();
    if let Some(hb) = stats(heartbeat_path)? {
        return Ok(now - mtime_ms(&hb) > stale_ms as f64);
    }
    if let Some(meta) = stats(meta_path)? {
        return Ok(now - mtime_ms(&meta) > stale_ms as f64);
    }
    let dir = stats(Path::new(lock_dir))?;
    if dir.is_none() {
        return Ok(false);
    }
    Ok(now - mtime_ms(&dir.unwrap()) > stale_ms as f64)
}

/// source: `writeFile(path, "", { flag: "wx" })` semantics (exclusive create).
fn write_exclusive(path: &Path, content: &[u8]) -> std::io::Result<()> {
    let mut opts = fs::OpenOptions::new();
    opts.write(true).create_new(true);
    use std::io::Write;
    let mut f = opts.open(path)?;
    f.write_all(content)
}

/// source: `utimes(path, t, t)` — touch mtime (heartbeat).
fn touch(path: &Path) -> std::io::Result<()> {
    use std::io::Write;
    // Open for write (no truncate) and set_modified — approximates utimes.
    let f = fs::OpenOptions::new().write(true).open(path)?;
    f.set_modified(SystemTime::now())
}

/// source: `tryAcquireLockDir(lockDir, opts)` — `{ acquired: true, ... }` or
/// `{ acquired: false }`; throws on unexpected errors. All message strings
/// verbatim.
#[allow(clippy::type_complexity)]
fn try_acquire_lock_dir(lock_dir: &str, opts: &Opts) -> Result<TryResult, String> {
    let token = crate::util::hash::uuid_v4();
    let meta_path = Path::new(lock_dir).join("meta.json");
    let heartbeat_path = Path::new(lock_dir).join("heartbeat");

    let created = match create_dir_mode(lock_dir, 0o700) {
        Ok(()) => true,
        Err(e) => {
            let code = error_code(&e);
            if code != Some("EEXIST") {
                return Err(e.to_string());
            }
            if !stale(lock_dir, &heartbeat_path, &meta_path, opts.stale_ms)? {
                return Ok(TryResult::NotAcquired);
            }

            // Stale — race for breaker ownership.
            let breaker_path = format!("{lock_dir}.breaker");
            let claimed = match create_dir_mode(&breaker_path, 0o700) {
                Ok(()) => true,
                Err(claim_err) => {
                    let claim_code = error_code(&claim_err);
                    if claim_code == Some("EEXIST") {
                        let breaker = stats(Path::new(&breaker_path))?;
                        if let Some(b) = breaker {
                            if wall() - mtime_ms(&b) > opts.stale_ms as f64 {
                                force_remove_dir_all(&breaker_path);
                            }
                        }
                        return Ok(TryResult::NotAcquired);
                    }
                    if claim_code == Some("ENOENT") || claim_code == Some("ENOTDIR") {
                        return Ok(TryResult::NotAcquired);
                    }
                    return Err(claim_err.to_string());
                }
            };

            if !claimed {
                return Ok(TryResult::NotAcquired);
            }

            // We own the breaker — double-check staleness, nuke, recreate.
            let recreated = (|| -> Result<bool, String> {
                if !stale(lock_dir, &heartbeat_path, &meta_path, opts.stale_ms)? {
                    return Ok(false);
                }
                force_remove_dir_all(lock_dir);
                match create_dir_mode(lock_dir, 0o700) {
                    Ok(()) => Ok(true),
                    Err(retry_err) => {
                        let retry_code = error_code(&retry_err);
                        if retry_code == Some("EEXIST") || retry_code == Some("ENOTEMPTY") {
                            return Ok(false);
                        }
                        Err(retry_err.to_string())
                    }
                }
            })();
            force_remove_dir_all(&breaker_path); // finally

            match recreated {
                Ok(true) => true,
                Ok(false) => return Ok(TryResult::NotAcquired),
                Err(e) => return Err(e),
            }
        }
    };

    if !created {
        return Ok(TryResult::NotAcquired);
    }

    // We own the lock dir — write heartbeat + meta with exclusive create.
    if write_exclusive(&heartbeat_path, b"").is_err() {
        force_remove_dir_all(lock_dir);
        return Err(
            "Lock acquired but heartbeat already existed (possible compromise).".to_string(),
        );
    }

    // JSON.stringify(meta, null, 2) — key order token/pid/hostname/createdAt,
    // 2-space indent, no space around colons.
    let hostname = read_hostname();
    let created_at = iso_now();
    let meta_json = format!(
        "{{\n  \"token\": {},\n  \"pid\": {},\n  \"hostname\": {},\n  \"createdAt\": {}\n}}",
        json_string(&token),
        std::process::id(),
        json_string(&hostname),
        json_string(&created_at)
    );

    if write_exclusive(&meta_path, meta_json.as_bytes()).is_err() {
        force_remove_dir_all(lock_dir);
        return Err(
            "Lock acquired but meta.json already existed (possible compromise).".to_string(),
        );
    }

    Ok(TryResult::Owned(Owned {
        token,
        meta_path,
        heartbeat_path: heartbeat_path.clone(),
        lock_dir: PathBuf::from(lock_dir),
        stale_ms: opts.stale_ms,
        heartbeat_stop: Arc::new(AtomicBool::new(false)),
    }))
}

/// source: `new Date().toISOString()` — UTC `YYYY-MM-DDTHH:MM:SS.mmmZ`.
fn iso_now() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let millis = now.as_millis() as u64;
    let secs = millis / 1000;
    let ms = millis % 1000;
    let days = (secs / 86400) as i64;
    let secs_of_day = secs % 86400;
    // Howard Hinnant's civil-from-days algorithm.
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
        y,
        m,
        d,
        secs_of_day / 3600,
        (secs_of_day % 3600) / 60,
        secs_of_day % 60,
        ms
    )
}

fn read_hostname() -> String {
    std::fs::read_to_string("/proc/sys/kernel/hostname")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| std::env::var("HOSTNAME").unwrap_or_default())
}

fn json_string(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

enum TryResult {
    Owned(Owned),
    NotAcquired,
}

struct Owned {
    token: String,
    meta_path: PathBuf,
    heartbeat_path: PathBuf,
    lock_dir: PathBuf,
    stale_ms: u64,
    heartbeat_stop: Arc<AtomicBool>,
}

impl Owned {
    /// source: `startHeartbeat(intervalMs = max(100, floor(staleMs / 3)))`.
    /// Detached thread touching mtime on the interval (timer.unref()
    /// equivalent: a detached thread, documented divergence).
    fn start_heartbeat(&mut self, interval_ms: Option<u64>) {
        let interval = interval_ms.unwrap_or_else(|| 100.max(self.stale_ms / 3));
        let stop = self.heartbeat_stop.clone();
        let path = self.heartbeat_path.clone();
        std::thread::spawn(move || loop {
            std::thread::sleep(Duration::from_millis(interval));
            if stop.load(Ordering::SeqCst) {
                break;
            }
            let _ = touch(&path);
        });
    }

    /// source: `release()` — token check, then rm(lockDir, recursive, force).
    /// All error strings verbatim; the final rm propagates raw errors.
    fn release(mut self) -> Result<(), String> {
        self.heartbeat_stop.store(true, Ordering::SeqCst);

        let raw = match fs::read_to_string(&self.meta_path) {
            Ok(raw) => raw,
            Err(e) => {
                let code = error_code(&e);
                if code == Some("ENOENT") || code == Some("ENOTDIR") {
                    return Err(
                        "Refusing to release: lock is compromised (metadata missing).".to_string(),
                    );
                }
                return Err(e.to_string());
            }
        };

        let parsed: serde_json::Value = match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(_) => {
                return Err(
                    "Refusing to release: lock is compromised (metadata invalid).".to_string(),
                )
            }
        };

        // `!parsed || typeof parsed !== "object"` → {} (token undefined).
        let current_token: Option<String> = match &parsed {
            serde_json::Value::Object(map) => map
                .get("token")
                .and_then(|t| t.as_str())
                .map(|s| s.to_string()),
            _ => None,
        };

        if current_token.as_deref() != Some(self.token.as_str()) {
            return Err("Refusing to release: lock token mismatch (not the owner).".to_string());
        }

        fs::remove_dir_all(&self.lock_dir).map_err(|e| e.to_string())
    }
}

/// source: `Lease` — `{ release, [Symbol.asyncDispose] }`. `release()` takes
/// the lease by value (consume-once); Drop mirrors asyncDispose.
pub struct Lease {
    inner: Option<Owned>,
}

impl Lease {
    pub fn release(mut self) -> Result<(), String> {
        match self.inner.take() {
            Some(owned) => owned.release(),
            None => Ok(()),
        }
    }
}

impl Drop for Lease {
    fn drop(&mut self) {
        if let Some(owned) = self.inner.take() {
            let _ = owned.release();
        }
    }
}

/// source: `acquireLockDir(lockDir, { key, onWait, signal }, opts)`.
fn acquire_lock_dir(
    lock_dir: &str,
    key: &str,
    on_wait: Option<&dyn Fn(&WaitEvent)>,
    signal: Option<&AbortSignal>,
    opts: &Opts,
) -> Result<Owned, String> {
    let stop = mono() + opts.timeout_ms as f64;
    let mut attempt: u64 = 0;
    let mut waited: u64 = 0;
    let mut delay = opts.base_delay_ms;

    loop {
        if let Some(s) = signal {
            s.throw_if_aborted()?;
        }

        match try_acquire_lock_dir(lock_dir, opts)? {
            TryResult::Owned(owned) => return Ok(owned),
            TryResult::NotAcquired => {}
        }

        if mono() > stop {
            return Err(format!("Timed out waiting for lock: {key}"));
        }

        attempt += 1;
        let ms = jitter(delay);
        if let Some(cb) = on_wait {
            cb(&WaitEvent {
                key: key.to_string(),
                attempt,
                delay: ms,
                waited,
            });
        }
        sleep(ms, signal)?;
        waited += ms;
        delay = opts.max_delay_ms.min(delay * 17 / 10); // Math.min(max, floor(delay*1.7))
    }
}

/// source: `Flock.acquire(key, options)` — returns Lease.
pub fn acquire(key: &str, input: &Options<'_>) -> Result<Lease, String> {
    if let Some(s) = input.signal {
        s.throw_if_aborted()?;
    }
    let cfg = Opts {
        stale_ms: input.stale_ms.unwrap_or(DEFAULT_STALE_MS),
        timeout_ms: input.timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS),
        base_delay_ms: input.base_delay_ms.unwrap_or(DEFAULT_BASE_DELAY_MS),
        max_delay_ms: input.max_delay_ms.unwrap_or(DEFAULT_MAX_DELAY_MS),
    };
    let dir = match &input.dir {
        Some(d) => d.clone(),
        None => root()?,
    };

    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let lockfile = Path::new(&dir)
        .join(format!("{}.lock", crate::util::hash::fast_str(key)))
        .to_string_lossy()
        .into_owned();

    let mut lock = acquire_lock_dir(&lockfile, key, input.on_wait, input.signal, &cfg)?;
    lock.start_heartbeat(None);

    Ok(Lease { inner: Some(lock) })
}

/// source: `Flock.withLock(key, fn, options)`.
pub fn with_lock<T>(key: &str, f: impl FnOnce() -> T, input: &Options<'_>) -> Result<T, String> {
    let lease = acquire(key, input)?;
    if let Some(s) = input.signal {
        s.throw_if_aborted()?;
    }
    let result = f();
    match lease.release() {
        Ok(()) => Ok(result),
        Err(e) => Err(e),
    }
}

/// source: `Flock.effect(key, options)` — the Effect acquireRelease wrapper
/// (sync port: runs `body` under the lock; release guaranteed on body
/// completion/panic via Drop).
pub fn effect<T>(key: &str, input: &Options<'_>, body: impl FnOnce() -> T) -> Result<T, String> {
    let lease = acquire(key, input)?;
    let result = body();
    match lease.release() {
        Ok(()) => Ok(result),
        Err(e) => Err(e),
    }
}

/// source: `Flock.effect` used as a scoped lock (acquire + immediate release
/// when the caller only needs mutual exclusion, e.g. `Flock.effect(key)`).
pub fn effect_scoped(key: &str, input: &Options<'_>) -> Result<(), String> {
    let lease = acquire(key, input)?;
    lease.release()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_dir() -> String {
        std::env::temp_dir()
            .join(format!("flock-test-{}", crate::util::hash::uuid_v4()))
            .to_string_lossy()
            .into_owned()
    }

    #[test]
    fn root_requires_global() {
        assert_eq!(root(), Err("Flock global not set".to_string()));
        set_global(FlockGlobal {
            state: "/tmp".to_string(),
        });
        assert_eq!(root().unwrap(), "/tmp/locks");
    }

    #[test]
    fn acquire_and_release() {
        set_global(FlockGlobal {
            state: "/tmp".to_string(),
        });
        let dir = tmp_dir();
        let opts = Options {
            dir: Some(dir.clone()),
            ..Default::default()
        };
        let lease = acquire("test-key", &opts).unwrap();
        lease.release().unwrap();
        assert!(
            !std::path::Path::new(&dir).exists() || std::fs::read_dir(&dir).unwrap().count() == 0
        );
    }

    #[test]
    fn lock_is_exclusive() {
        set_global(FlockGlobal {
            state: "/tmp".to_string(),
        });
        let dir = tmp_dir();
        let opts = Options {
            dir: Some(dir.clone()),
            stale_ms: Some(100),
            timeout_ms: Some(2000),
            base_delay_ms: Some(10),
            max_delay_ms: Some(50),
            ..Default::default()
        };
        let _first = acquire("exclusive", &opts).unwrap();
        let waited = std::cell::Cell::new(false);
        let on_wait = |_: &WaitEvent| waited.set(true);
        let opts2 = Options {
            dir: Some(dir.clone()),
            stale_ms: Some(100),
            timeout_ms: Some(2000),
            base_delay_ms: Some(10),
            max_delay_ms: Some(50),
            on_wait: Some(&on_wait),
            ..Default::default()
        };
        let second = acquire("exclusive", &opts2);
        // Second contender should time out while first holds the lock
        // (stale_ms tiny but heartbeat keeps it fresh: sleep a bit first).
        std::thread::sleep(Duration::from_millis(300));
        assert!(second.is_err());
    }

    #[test]
    fn timed_out_message() {
        set_global(FlockGlobal {
            state: "/tmp".to_string(),
        });
        let dir = tmp_dir();
        let opts = Options {
            dir: Some(dir.clone()),
            timeout_ms: Some(50),
            base_delay_ms: Some(10),
            max_delay_ms: Some(20),
            ..Default::default()
        };
        let _first = acquire("timeout-key", &opts).unwrap();
        let err = acquire("timeout-key", &opts).unwrap_err();
        assert_eq!(err, "Timed out waiting for lock: timeout-key");
    }

    #[test]
    fn stale_lock_is_cleaned() {
        set_global(FlockGlobal {
            state: "/tmp".to_string(),
        });
        let dir = tmp_dir();
        let lock_dir = format!("{}/{}", dir, crate::util::hash::fast_str("stale-key"));
        std::fs::create_dir_all(&lock_dir).unwrap();
        // touch the dir far in the past to make it stale
        let old = SystemTime::now() - Duration::from_secs(3600);
        let f = std::fs::OpenOptions::new()
            .write(true)
            .open(&lock_dir)
            .unwrap();
        let _ = f.set_modified(old);
        let opts = Options {
            dir: Some(dir.clone()),
            stale_ms: Some(1000),
            timeout_ms: Some(2000),
            base_delay_ms: Some(10),
            max_delay_ms: Some(50),
            ..Default::default()
        };
        // holding a stale lock dir with no heartbeat/meta → stale → replaced
        let lease = acquire("stale-key", &opts).unwrap();
        lease.release().unwrap();
    }
}
