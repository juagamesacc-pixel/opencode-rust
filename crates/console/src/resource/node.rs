// source: resource/resource.node.ts
//! 1:1 port of the SST/Node resource proxy. Source pin: v1.18.30 @3104c14.
//!
//! PROVISIONAL: the source proxies the `sst` package's `Resource` binding, so
//! every property lookup is an SST deployment binding resolved at runtime by the
//! SST CLI. There is no Rust equivalent, so `Sst::get` reproduces the proxy's
//! property resolution against a registered descriptor table, keeping the
//! verbatim binding names, the `sst.cloudflare.Bucket` / `sst.cloudflare.Kv`
//! type dispatch and the `CLOUDFLARE_API_TOKEN` /
//! `CLOUDFLARE_DEFAULT_ACCOUNT_ID` secret keys.

use std::cell::RefCell;
use std::collections::HashMap;

/// `sst.cloudflare.Bucket` — the branch that degrades to a no-op `put`.
pub const TYPE_CLOUDFLARE_BUCKET: &str = "sst.cloudflare.Bucket";

/// `sst.cloudflare.Kv` — the branch that builds a real Cloudflare KV client.
pub const TYPE_CLOUDFLARE_KV: &str = "sst.cloudflare.Kv";

/// Secret key read by the `Kv` branch to build the Cloudflare client.
pub const CLOUDFLARE_API_TOKEN: &str = "CLOUDFLARE_API_TOKEN";

/// Secret key read by the `Kv` branch for `account_id`.
pub const CLOUDFLARE_DEFAULT_ACCOUNT_ID: &str = "CLOUDFLARE_DEFAULT_ACCOUNT_ID";

/// One entry of the `Resource` proxy target: the `type` discriminator the source
/// checks with `"type" in value`, plus the binding's resolved properties.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Binding {
    /// The SST resource type, e.g. `sst.cloudflare.Kv`. `None` means the
    /// property has no `type` key, so the proxy returns `value` untouched.
    pub resource_type: Option<String>,
    pub properties: HashMap<String, String>,
}

impl Binding {
    /// A typed SST binding.
    pub fn typed(resource_type: &str, properties: &[(&str, &str)]) -> Self {
        Self {
            resource_type: Some(resource_type.to_string()),
            properties: properties
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect(),
        }
    }

    /// An untyped binding, for which the proxy returns the value as-is.
    pub fn plain(properties: &[(&str, &str)]) -> Self {
        Self {
            resource_type: None,
            properties: properties
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect(),
        }
    }

    /// Whether `"type" in value` holds, i.e. whether the dispatch is attempted.
    fn has_type(&self) -> bool {
        self.resource_type.is_some()
    }
}

thread_local! {
    static BINDINGS: RefCell<HashMap<String, Binding>> = RefCell::new(HashMap::new());
    static SECRETS: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

/// The `Resource` proxy, reduced to the property resolution the source performs.
pub struct Sst;

impl Sst {
    /// Registers a binding under `prop`, as the SST runtime would.
    pub fn register(prop: &str, binding: Binding) {
        BINDINGS.with(|slots| {
            slots.borrow_mut().insert(prop.to_string(), binding);
        });
    }

    /// Clears every registered binding.
    pub fn reset() {
        BINDINGS.with(|slots| slots.borrow_mut().clear());
    }

    /// Sets a `secrets.CLOUDFLARE_*.value` entry.
    pub fn set_secret(key: &str, value: &str) {
        SECRETS.with(|slots| {
            slots
                .borrow_mut()
                .insert(key.to_string(), value.to_string());
        });
    }

    /// Clears every registered secret.
    pub fn reset_secrets() {
        SECRETS.with(|slots| slots.borrow_mut().clear());
    }

    /// `Resource[prop]` reduced to the property value the proxy resolves, for the
    /// untyped and non-Cloudflare-typed bindings that carry plain values.
    ///
    /// `sst.cloudflare.Bucket` and `sst.cloudflare.Kv` are handled by
    /// [`Sst::bucket`] and [`Kv::client`] respectively, matching the source's
    /// early returns.
    pub fn get(prop: &str, key: &str) -> String {
        BINDINGS.with(|slots| {
            let slots = slots.borrow();
            let binding = slots.get(prop);
            match binding {
                Some(binding) if binding.has_type() => {
                    // Typed Cloudflare resources never expose flat properties.
                    String::new()
                }
                Some(binding) => binding.properties.get(key).cloned().unwrap_or_default(),
                None => String::new(),
            }
        })
    }

    /// The `sst.cloudflare.Bucket` branch — the source returns an object whose
    /// only member is a no-op `put`.
    pub fn bucket(prop: &str) -> Option<Bucket> {
        BINDINGS.with(|slots| {
            let slots = slots.borrow();
            match slots.get(prop) {
                Some(binding)
                    if binding.resource_type.as_deref() == Some(TYPE_CLOUDFLARE_BUCKET) =>
                {
                    Some(Bucket {
                        _prop: prop.to_string(),
                    })
                }
                _ => None,
            }
        })
    }

    /// The `sst.cloudflare.Kv` branch — the source builds `new Cloudflare({
    /// apiToken: secrets.CLOUDFLARE_API_TOKEN.value })` and captures
    /// `value.namespaceId`.
    pub fn kv(prop: &str) -> Option<Kv> {
        BINDINGS.with(|slots| {
            let slots = slots.borrow();
            match slots.get(prop) {
                Some(binding) if binding.resource_type.as_deref() == Some(TYPE_CLOUDFLARE_KV) => {
                    let namespace_id = binding
                        .properties
                        .get("namespaceId")
                        .cloned()
                        .unwrap_or_default();
                    Some(Kv {
                        _prop: prop.to_string(),
                        namespace_id,
                    })
                }
                _ => None,
            }
        })
    }

    /// `secrets.CLOUDFLARE_API_TOKEN.value`.
    pub fn cloudflare_api_token() -> String {
        Sst::secret(CLOUDFLARE_API_TOKEN)
    }

    /// `secrets.CLOUDFLARE_DEFAULT_ACCOUNT_ID.value`.
    pub fn cloudflare_default_account_id() -> String {
        Sst::secret(CLOUDFLARE_DEFAULT_ACCOUNT_ID)
    }

    /// `secrets[key].value`.
    pub fn secret(key: &str) -> String {
        SECRETS.with(|slots| slots.borrow().get(key).cloned().unwrap_or_default())
    }
}

/// `sst.cloudflare.Bucket` — `put` is an awaited no-op, matching the source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bucket {
    _prop: String,
}

impl Bucket {
    /// `await value.put()`.
    pub async fn put(&self) {}
}

/// `sst.cloudflare.Kv` — the namespace-bound client the proxy hands back.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Kv {
    _prop: String,
    /// `value.namespaceId`.
    pub namespace_id: String,
}

impl Kv {
    /// The `client` the source constructs for this namespace.
    pub fn client(&self) -> KvClient {
        KvClient {
            namespace_id: self.namespace_id.clone(),
            api_token: Sst::cloudflare_api_token(),
            account_id: Sst::cloudflare_default_account_id(),
        }
    }
}

/// The `new Cloudflare({ apiToken })` client scoped to one KV namespace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KvClient {
    pub namespace_id: String,
    pub api_token: String,
    pub account_id: String,
}

impl KvClient {
    /// `client.kv.namespaces.bulkGet(namespaceId, { keys, account_id })`.
    ///
    /// PROVISIONAL: the Cloudflare REST round-trip is a host surface. The
    /// multi-key flag that drives the `Map` construction is ported faithfully.
    pub fn bulk_get(&self, keys: &[&str]) -> KvBulkGet {
        KvBulkGet {
            multi: keys.len() > 1,
            keys: keys.iter().map(|key| key.to_string()).collect(),
            account_id: self.account_id.clone(),
        }
    }
}

/// The resolved `bulkGet` request descriptor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KvBulkGet {
    /// `isMulti` — true when the source was called with a `string[]`.
    pub multi: bool,
    /// `keys` — `Array.isArray(k) ? k : [k]`.
    pub keys: Vec<String>,
    /// `account_id`.
    pub account_id: String,
}

impl KvBulkGet {
    /// The result branch the source takes: a `Map` for the multi-key case, the
    /// single value otherwise.
    pub fn shape(&self) -> KvBulkGetShape {
        if self.multi {
            KvBulkGetShape::Map
        } else {
            KvBulkGetShape::Single(self.keys.first().cloned().unwrap_or_default())
        }
    }
}

/// `isMulti ? new Map(Object.entries(result?.values ?? {})) : result?.values?.[k]`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KvBulkGetShape {
    Map,
    Single(String),
}

/// `waitUntil(promise)` — awaits the promise; the source has no other behavior.
pub async fn wait_until<F: std::future::Future<Output = ()>>(promise: F) {
    promise.await
}
