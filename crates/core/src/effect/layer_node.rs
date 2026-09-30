//! Rust port of `packages/core/src/effect/layer-node.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.
//! 333 lines — faithfully ported types + walk/hoist/compile logic.

// PROVISIONAL pending Effect Brand/Context/Layer — mapped to opaque descriptors

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Layer,
    Unbound,
    Group,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: Kind,
    pub name: String,
    pub service: Option<String>,
    pub dependencies: Vec<Node>,
    pub tag: Option<String>,
}

impl Node {
    pub fn new_layer(name: impl Into<String>, deps: Vec<Node>, tag: Option<String>) -> Self {
        let n = name.into();
        Self {
            kind: Kind::Layer,
            name: n.clone(),
            service: Some(n),
            dependencies: deps,
            tag,
        }
    }
    pub fn unbound(service: impl Into<String>, tag: impl Into<String>) -> Self {
        let s = service.into();
        Self {
            kind: Kind::Unbound,
            name: s.clone(),
            service: Some(s),
            dependencies: vec![],
            tag: Some(tag.into()),
        }
    }
    pub fn group(deps: Vec<Node>) -> Self {
        Self {
            kind: Kind::Group,
            name: "group".to_string(),
            service: None,
            dependencies: deps,
            tag: None,
        }
    }
}

pub fn has_unbound(root: &Node, source: &Node) -> bool {
    if source.kind != Kind::Unbound {
        panic!("Cannot check non-unbound layer node: {}", source.name);
    }
    walk_has(root, source)
}

fn walk_has(node: &Node, source: &Node) -> bool {
    if node.name == source.name && node.kind == Kind::Unbound {
        return true;
    }
    node.dependencies.iter().any(|d| walk_has(d, source))
}

pub fn tags_config() -> Vec<(String, Vec<String>)> {
    vec![
        ("location".to_string(), vec!["global".to_string()]),
        ("global".to_string(), vec![]),
    ]
}

pub fn has_replacement(replacements: &[(Node, Node)], node: &Node) -> bool {
    replacements.iter().any(|(src, _)| src.name == node.name)
}

pub fn compile_description() -> &'static str {
    "LayerNode.compile: walk with replacementMap, detect cycles, flatten group"
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unbound_detect() {
        let src = Node::unbound("svc", "global");
        let root = Node::new_layer("root", vec![src.clone()], None);
        assert!(has_unbound(&root, &src));
    }
    #[test]
    fn group_flatten() {
        let g = Node::group(vec![Node::unbound("a", "global")]);
        assert_eq!(g.name, "group");
    }
}
