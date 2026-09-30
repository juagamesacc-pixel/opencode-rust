//! Port of packages/app/src/components/file-tree-v2-model.ts
//! ——— 1:1 exact clone, zero diversion ———
//! Rename log: `file-tree-v2-model` → `file_tree_v2_model`.
#![allow(clippy::all)]
#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileTreeV2Node {
    pub name: String,
    pub path: String,
    pub absolute: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub ignored: bool,
    pub original_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FileTreeV2Model {
    pub children: HashMap<String, Vec<FileTreeV2Node>>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileTreeV2Row {
    pub node: FileTreeV2Node,
    pub level: usize,
}

pub fn normalize_file_tree_v2_path(value: &str) -> String {
    let v = value.replace('\\', "/");
    let v = v.trim_matches('/').to_string();
    // collapse //
    let mut out = String::new();
    let mut prev_slash = false;
    for c in v.chars() {
        if c == '/' {
            if prev_slash {
                continue;
            }
            prev_slash = true;
            out.push(c);
        } else {
            prev_slash = false;
            out.push(c);
        }
    }
    out
}

pub fn build_file_tree_v2_model(paths: &[String]) -> FileTreeV2Model {
    let mut nodes: BTreeMap<String, FileTreeV2Node> = BTreeMap::new();
    for value in paths {
        let file = normalize_file_tree_v2_path(value);
        if file.is_empty() {
            continue;
        }
        let parts: Vec<&str> = file.split('/').collect();
        for (idx, name) in parts.iter().enumerate() {
            let path = parts[..=idx].join("/");
            if nodes.contains_key(&path) {
                continue;
            }
            nodes.insert(
                path.clone(),
                FileTreeV2Node {
                    name: name.to_string(),
                    path: path.clone(),
                    absolute: path.clone(),
                    node_type: if idx == parts.len() - 1 {
                        "file".to_string()
                    } else {
                        "directory".to_string()
                    },
                    ignored: false,
                    original_path: if idx == parts.len() - 1 {
                        value.clone()
                    } else {
                        path.clone()
                    },
                },
            );
        }
    }
    let mut children: HashMap<String, Vec<FileTreeV2Node>> = HashMap::new();
    for node in nodes.values() {
        let parent = match node.path.rfind('/') {
            Some(i) => node.path[..i].to_string(),
            None => "".to_string(),
        };
        children.entry(parent).or_default().push(node.clone());
    }
    for list in children.values_mut() {
        list.sort_by(|a, b| {
            if a.node_type != b.node_type {
                return if a.node_type == "directory" {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                };
            }
            a.name.cmp(&b.name)
        });
    }
    FileTreeV2Model {
        total: nodes.len(),
        children,
    }
}

pub fn flatten_file_tree_v2(
    model: &FileTreeV2Model,
    expanded: impl Fn(&str) -> bool,
) -> Vec<FileTreeV2Row> {
    let mut rows = Vec::new();
    let mut stack: Vec<FileTreeV2Row> = model
        .children
        .get("")
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .rev()
        .map(|n| FileTreeV2Row { node: n, level: 0 })
        .collect();
    while let Some(row) = stack.pop() {
        let is_dir = row.node.node_type == "directory";
        let path = row.node.path.clone();
        let level = row.level;
        rows.push(row);
        if !is_dir || !expanded(&path) {
            continue;
        }
        if let Some(children) = model.children.get(&path) {
            for child in children.iter().rev() {
                stack.push(FileTreeV2Row {
                    node: child.clone(),
                    level: level + 1,
                });
            }
        }
    }
    rows
}
