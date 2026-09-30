// source: packages/tui/src/context/kv.tsx (66 lines, v1.18.30)
// 1:1 port — SolidJS store becomes a plain map; same-process writes stay
// ordered behind an async mutex (mirrors the promise chain); cross-process
// flock has no dependency-free equivalent and is documented, not faked.

#![allow(dead_code)]

use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::thinking::KvAccess;
use crate::util::persistence;

/// Filename verbatim (`kv.json` under the TUI state dir).
pub const KV_FILENAME: &str = "kv.json";

/// Mirrors the KV context value.
#[derive(Clone)]
pub struct KvStore {
    file: PathBuf,
    store: HashMap<String, Value>,
    ready: bool,
    write_lock: Arc<Mutex<()>>,
}

impl KvStore {
    /// Mirrors `init` — reads state, then marks ready (errors are logged,
    /// mirroring `console.error`, and still resolve ready).
    pub async fn init(state_dir: &str) -> Self {
        let file = PathBuf::from(state_dir).join(KV_FILENAME);
        let mut store = Self {
            file,
            store: HashMap::new(),
            ready: false,
            write_lock: Arc::new(Mutex::new(())),
        };
        match persistence::read_json::<HashMap<String, Value>>(&store.file).await {
            Ok(values) => {
                store.store = values;
            }
            Err(error) => {
                eprintln!("Failed to read KV state {error}");
            }
        }
        store.ready = true;
        store
    }

    pub fn ready(&self) -> bool {
        self.ready
    }

    pub fn snapshot(&self) -> HashMap<String, Value> {
        self.store.clone()
    }

    /// Mirrors `signal(name, default)` seeding.
    pub fn signal_seed(&mut self, name: &str, default: Value) {
        self.store.entry(name.to_string()).or_insert(default);
    }

    /// Mirrors `get(key, default?)`.
    pub fn get(&self, key: &str, default: Option<Value>) -> Value {
        self.store
            .get(key)
            .cloned()
            .unwrap_or(default.unwrap_or(Value::Null))
    }

    /// Mirrors `set` — memory first, then the ordered atomic persist.
    pub fn set(&mut self, key: &str, value: Value) {
        self.store.insert(key.to_string(), value);
        let file = self.file.clone();
        let snapshot = Value::Object(
            self.store
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        );
        let lock = self.write_lock.clone();
        tokio::spawn(async move {
            let _guard = lock.lock().await;
            if let Err(error) = persistence::write_json_atomic(&file, &snapshot).await {
                eprintln!("Failed to write KV state {error}");
            }
        });
    }
}

impl KvAccess for KvStore {
    fn kv_get(&self, key: &str) -> Option<Value> {
        self.store.get(key).cloned()
    }

    fn kv_set(&mut self, key: &str, value: Value) {
        self.set(key, value);
    }
}
