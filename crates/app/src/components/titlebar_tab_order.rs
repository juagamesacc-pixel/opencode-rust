//! Port of packages/app/src/components/titlebar-tab-order.ts
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]

pub fn adjacent_tab_key(order: &[String], current: Option<&str>, offset: i32) -> Option<String> {
    let cur = current?;
    if order.is_empty() {
        return None;
    }
    let index = order.iter().position(|x| x == cur)?;
    let len = order.len() as i32;
    let next = ((index as i32 + offset + len) % len) as usize;
    order.get(next).cloned()
}

pub fn merge_visible_tab_order(all: &[String], current: &[String], next: &[String]) -> Vec<String> {
    use std::collections::HashSet;
    let visible: HashSet<&String> = current.iter().collect();
    let mut reordered = next.iter();
    all.iter()
        .map(|key| {
            if visible.contains(key) {
                reordered.next().cloned().unwrap_or_else(|| key.clone())
            } else {
                key.clone()
            }
        })
        .collect()
}
