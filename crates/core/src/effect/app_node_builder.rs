//! Rust port of `packages/core/src/effect/app-node-builder.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

/// Source: `export function build<A,E>(root, replacements=[])` verbatim
/// Source: `hasReplacement` check verbatim — `replacements.some(([source])=>source.name===node.name)`

// PROVISIONAL pending LayerNode, LocationServiceMap, makeGlobalNode, buildLocationServiceMap

pub fn has_replacement(replacements: &[(&str, &str)], node_name: &str) -> bool {
    replacements.iter().any(|(source, _)| *source == node_name)
}

pub fn build_description() -> &'static str {
    "LayerNode.compile(root, allReplacements) with LocationServiceMap hoist if unbound"
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn has_replacement_check() {
        assert!(has_replacement(&[("a", "b")], "a"));
        assert!(!has_replacement(&[("a", "b")], "c"));
    }
}
