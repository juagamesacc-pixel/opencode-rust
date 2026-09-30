// source: src/util/array.ts — exports: findLast

/// source: findLast(items, predicate) — reverse scan, predicate(item, index, items).
/// Returns the last item for which predicate is truthy, or None (JS undefined). Verbatim.
pub fn find_last<'a, T, F>(items: &'a [T], mut predicate: F) -> Option<&'a T>
where
    F: FnMut(&T, usize, &[T]) -> bool,
{
    for i in (0..items.len()).rev() {
        let item = &items[i];
        if predicate(item, i, items) {
            return Some(item);
        }
    }
    None
}
