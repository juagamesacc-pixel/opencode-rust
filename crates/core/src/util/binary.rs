// source: src/util/binary.ts — exports: search, insert

/// Mirrors `{ found: boolean; index: number }` from `Binary.search`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SearchResult {
    pub found: bool,
    pub index: usize,
}

/// source: Binary.search(array, id, compare) — classic binary search over a
/// compare()-sorted array. `index` is the insertion point when not found
/// (JS `left` after the loop). Verbatim semantics incl. empty array.
pub fn search<T, F>(array: &[T], id: &str, compare: F) -> SearchResult
where
    F: Fn(&T) -> String,
{
    let mut left: usize = 0;
    let mut right: isize = array.len() as isize - 1;

    while (left as isize) <= right {
        let mid = ((left as isize + right) / 2) as usize;
        let mid_id = compare(&array[mid]);

        if mid_id == id {
            return SearchResult {
                found: true,
                index: mid,
            };
        } else if mid_id < id {
            left = mid + 1;
        } else {
            right = mid as isize - 1;
        }
    }

    SearchResult {
        found: false,
        index: left,
    }
}

/// source: Binary.insert(array, item, compare) — lower-bound insertion point
/// (first index where `midId >= id`), then splice-in at that index. Mutates
/// and returns the same array (TS `T[]`).
pub fn insert<T, F>(array: &mut Vec<T>, item: T, compare: F) -> &mut Vec<T>
where
    F: Fn(&T) -> String,
{
    let id = compare(&item);
    let mut left: usize = 0;
    let mut right: usize = array.len();

    while left < right {
        let mid = (left + right) / 2;
        let mid_id = compare(&array[mid]);

        if mid_id < id {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    array.insert(left, item);
    array
}
