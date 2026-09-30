// source: core/src/identifier.ts
//! 1:1 port of the `Identifier` namespace — prefixed ULID generation and validation.
//! Source pin: v1.18.30 @3104c14.

/// The `prefixes` map, in source order. Each variant keeps its verbatim prefix
/// string.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Prefix {
    Account,
    Auth,
    Benchmark,
    Billing,
    Key,
    Lite,
    Model,
    Payment,
    Provider,
    Referral,
    Subscription,
    Usage,
    User,
    Workspace,
}

/// The `prefixes` object, in source declaration order.
pub const PREFIXES: [(&str, Prefix, &str); 14] = [
    ("account", Prefix::Account, "acc"),
    ("auth", Prefix::Auth, "aut"),
    ("benchmark", Prefix::Benchmark, "ben"),
    ("billing", Prefix::Billing, "bil"),
    ("key", Prefix::Key, "key"),
    ("lite", Prefix::Lite, "lit"),
    ("model", Prefix::Model, "mod"),
    ("payment", Prefix::Payment, "pay"),
    ("provider", Prefix::Provider, "prv"),
    ("referral", Prefix::Referral, "ref"),
    ("subscription", Prefix::Subscription, "sub"),
    ("usage", Prefix::Usage, "usg"),
    ("user", Prefix::User, "usr"),
    ("workspace", Prefix::Workspace, "wrk"),
];

/// `prefixes[prefix]`.
pub fn prefix_value(prefix: Prefix) -> &'static str {
    PREFIXES
        .iter()
        .find(|(_, variant, _)| *variant == prefix)
        .map(|(_, _, value)| *value)
        .expect("every Prefix variant is present in PREFIXES")
}

/// The `Identifier` namespace.
pub struct Identifier;

/// Error thrown by `create()` when `given` carries the wrong prefix. Message
/// text is verbatim: `ID ${given} does not start with ${prefix}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrefixMismatch {
    pub given: String,
    pub expected: &'static str,
}

impl std::fmt::Display for PrefixMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID {} does not start with {}", self.given, self.expected)
    }
}

impl std::error::Error for PrefixMismatch {}

impl Identifier {
    /// `create(prefix, given?)`.
    ///
    /// With `given`, the value is returned unchanged when it already starts with
    /// the prefix; otherwise `Err(PrefixMismatch)` carries the verbatim message.
    /// Without `given`, the result is `"<prefix>_<ulid>"`.
    pub fn create(prefix: Prefix, given: Option<&str>) -> Result<String, PrefixMismatch> {
        let expected = prefix_value(prefix);
        if let Some(given) = given {
            if !given.is_empty() {
                if given.starts_with(expected) {
                    return Ok(given.to_string());
                }
                return Err(PrefixMismatch {
                    given: given.to_string(),
                    expected,
                });
            }
        }
        Ok(format!("{}_{}", expected, crate::runtime::ulid::ulid()))
    }

    /// `schema(prefix)` — `z.string().startsWith(prefixes[prefix])`, reduced to
    /// the predicate that is the whole of its runtime behavior.
    pub fn schema(prefix: Prefix) -> impl Fn(&str) -> bool {
        let expected = prefix_value(prefix);
        move |value: &str| value.starts_with(expected)
    }
}
