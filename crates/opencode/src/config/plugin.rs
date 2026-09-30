// source: src/config/plugin.ts — exports: Scope, Origin, load,
// pluginSpecifier, pluginOptions, resolvePluginSpec,
// deduplicatePluginOrigins, ConfigPlugin
// PROVISIONAL pending core v1/config/plugin + @/plugin/shared: spec tuple
// rules (spec=[0], options=[1]), path-spec resolution, reverse-dedupe
// (file:// identity vs package name) verbatim.

/// source: Scope = "global" | "local" — verbatim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scope {
    Global,
    Local,
}

impl Scope {
    pub fn as_str(&self) -> &'static str {
        match self {
            Scope::Global => "global",
            Scope::Local => "local",
        }
    }
}

/// source: Origin { spec, source, scope } — verbatim.
#[derive(Debug, Clone)]
pub struct Origin {
    pub spec: String,
    pub options: Option<String>,
    pub source: String,
    pub scope: Scope,
}

/// source: plugin glob "{plugin,plugins}/*.{ts,js}" — verbatim.
pub const PLUGIN_GLOB: &str = "{plugin,plugins}/*.{ts,js}";

/// source: resolvePluginSpec() — non-path passthrough; file:// keep;
/// absolute-or-drive → file URL; else resolve(base, spec). Verbatim.
pub fn resolve_plugin_spec(spec: &str, config_dir: &str, is_path_spec: bool) -> String {
    if !is_path_spec {
        return spec.to_string();
    }
    if spec.starts_with("file://") {
        return spec.to_string();
    }
    if spec.starts_with('/') || spec.len() > 2 && spec.as_bytes()[1] == b':' {
        return format!("file://{}", spec);
    }
    format!("file://{}/{}", config_dir.trim_end_matches('/'), spec)
}

/// source: deduplicatePluginOrigins() — reverse, dedupe on load identity
/// (file:// exact vs package name), reverse back. Verbatim.
pub fn deduplicate_origins(plugins: Vec<Origin>, pkg_of: &dyn Fn(&str) -> String) -> Vec<Origin> {
    let mut seen = std::collections::HashSet::new();
    let mut list = Vec::new();
    for plugin in plugins.iter().rev() {
        let name = if plugin.spec.starts_with("file://") {
            plugin.spec.clone()
        } else {
            pkg_of(&plugin.spec)
        };
        if seen.contains(&name) {
            continue;
        }
        seen.insert(name);
        list.push(plugin.clone());
    }
    list.reverse();
    list
}
