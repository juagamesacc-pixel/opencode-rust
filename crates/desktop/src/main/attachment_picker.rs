//! Rust port of `src/main/attachment-picker.ts` (opencode v1.18.30).
//!
//! Ported exactly: `MAX_ATTACHMENT_BYTES`, the per-token authorization
//! registry (sender-scoped, single-use `paths` set, running byte budget that
//! only counts successful reads, drop-the-token-when-empty), the
//! `assertAttachmentBudget` total, and both `sizeLimit` messages with the
//! `MAX_ATTACHMENT_BYTES / 1024 / 1024` param.
//!
//! PROVISIONAL: `randomUUID` (Node `crypto`) and `open` (Node `fs`) have no
//! in-workspace Rust binding, so the token generator and the default
//! `readAttachment` reader are PROVISIONAL; the registry is ported over an
//! injected reader and token factory.
//!
//! Original file: `packages/desktop/src/main/attachment-picker.ts`

use std::collections::BTreeSet;

use crate::main::native_translations::native_t;

/// Mirrors `export const MAX_ATTACHMENT_BYTES = 20 * 1024 * 1024`.
pub const MAX_ATTACHMENT_BYTES: u64 = 20 * 1024 * 1024;

/// Mirrors the `selections` map value type.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Selection {
    sender: u32,
    paths: BTreeSet<String>,
    remaining: u64,
}

/// The registry returned by `createPickedFileAuthorizations`.
pub struct PickedFileAuthorizations {
    selections: std::cell::RefCell<std::collections::BTreeMap<String, Selection>>,
    budget: u64,
    next_token: std::cell::Cell<u64>,
}

impl PickedFileAuthorizations {
    /// Mirrors `createPickedFileAuthorizations(read, budget)`. The token
    /// factory stands in for `randomUUID` (PROVISIONAL); `read` stands in
    /// for `readAttachment`.
    pub fn new(budget: u64) -> Self {
        Self {
            selections: std::cell::RefCell::new(std::collections::BTreeMap::new()),
            budget,
            next_token: std::cell::Cell::new(0),
        }
    }

    /// Mirrors `add(sender, paths)`: a fresh token scoped to `sender` with
    /// the full budget.
    pub fn add(&self, sender: u32, paths: &[String]) -> String {
        // PROVISIONAL(packages/desktop/src/main/attachment-picker.ts):
        // `randomUUID()` needs Node `crypto`.
        let token = format!("token-{}", self.next_token.get());
        self.next_token.set(self.next_token.get() + 1);
        self.selections.borrow_mut().insert(
            token.clone(),
            Selection {
                sender,
                paths: paths.iter().cloned().collect(),
                remaining: self.budget,
            },
        );
        token
    }

    /// Mirrors `read(sender, token, path)`.
    ///
    /// A wrong sender or an already-consumed (or unknown) path throws
    /// `desktop.picker.error.notSelected`. The budget is decremented by the
    /// bytes actually read, and the token is dropped once its last path is
    /// consumed.
    pub fn read<F>(&self, sender: u32, token: &str, path: &str, read: &F) -> Result<Vec<u8>, String>
    where
        F: Fn(&str, u64) -> Result<Vec<u8>, String>,
    {
        let remaining = {
            let mut selections = self.selections.borrow_mut();
            let Some(selection) = selections.get_mut(token) else {
                return Err(native_t("desktop.picker.error.notSelected", &[]));
            };
            if selection.sender != sender || !selection.paths.remove(path) {
                return Err(native_t("desktop.picker.error.notSelected", &[]));
            }
            selection.remaining
        };
        let bytes = read(path, remaining)?;
        {
            let mut selections = self.selections.borrow_mut();
            if let Some(selection) = selections.get_mut(token) {
                selection.remaining = selection.remaining.saturating_sub(bytes.len() as u64);
                if selection.paths.is_empty() {
                    selections.remove(token);
                }
            }
        }
        Ok(bytes)
    }

    /// Mirrors `release(sender, token)`.
    pub fn release(&self, sender: u32, token: &str) {
        let mut selections = self.selections.borrow_mut();
        if selections
            .get(token)
            .is_some_and(|selection| selection.sender == sender)
        {
            selections.remove(token);
        }
    }

    /// Mirrors `selections.size === 0` at the end of a `read`.
    pub fn is_empty(&self) -> bool {
        self.selections.borrow().is_empty()
    }
}

/// Mirrors `function assertAttachmentBudget(files)`.
pub fn assert_attachment_budget(sizes: &[u64]) -> Result<(), String> {
    let total: u64 = sizes.iter().sum();
    if total <= MAX_ATTACHMENT_BYTES {
        return Ok(());
    }
    Err(size_limit_message())
}

/// Mirrors the `desktop.picker.error.sizeLimit` message with its
/// `MAX_ATTACHMENT_BYTES / 1024 / 1024` param.
pub fn size_limit_message() -> String {
    let limit = MAX_ATTACHMENT_BYTES / 1024 / 1024;
    native_t(
        "desktop.picker.error.sizeLimit",
        &[("limit", limit.to_string())],
    )
}

/// Mirrors `export async function readAttachment(filePath, maxBytes?)`.
///
/// PROVISIONAL(packages/desktop/src/main/attachment-picker.ts): `open`/`stat`
/// need the Node filesystem. The `size > maxBytes` guard and the short-read
/// loop are reproduced by [`read_attachment_from`].
pub fn read_attachment(file_path: &str, max_bytes: u64) -> Result<Vec<u8>, String> {
    let _ = (file_path, max_bytes);
    Err("readAttachment needs the Node filesystem".to_string())
}

/// The `size > maxBytes` guard plus the `bytesRead === 0` short-read loop,
/// over an injected reader. Mirrors `readAttachment`'s body.
pub fn read_attachment_from<F>(size: u64, max_bytes: u64, mut read_at: F) -> Result<Vec<u8>, String>
where
    F: FnMut(u64) -> Result<(Vec<u8>, usize), String>,
{
    if size > max_bytes {
        return Err(size_limit_message());
    }
    let mut bytes: Vec<u8> = Vec::with_capacity(size as usize);
    let mut offset: u64 = 0;
    while offset < size {
        let (chunk, bytes_read) = read_at(offset)?;
        if bytes_read == 0 {
            break;
        }
        bytes.extend_from_slice(&chunk[..bytes_read]);
        offset += bytes_read as u64;
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    // The source ships no `attachment-picker.test.ts`; these cover the 1:1
    // behaviour of the authorization registry and the budget guard.
    use super::*;

    fn token_paths() -> Vec<String> {
        vec!["/a.txt".to_string(), "/b.txt".to_string()]
    }

    #[test]
    fn a_picked_path_can_only_be_read_once() {
        let auth = PickedFileAuthorizations::new(MAX_ATTACHMENT_BYTES);
        let token = auth.add(1, &token_paths());
        let read = |_: &str, _: u64| Ok(b"hello".to_vec());

        assert_eq!(auth.read(1, &token, "/a.txt", &read).unwrap(), b"hello");
        assert_eq!(
            auth.read(1, &token, "/a.txt", &read).unwrap_err(),
            native_t("desktop.picker.error.notSelected", &[])
        );
        // An unknown path is rejected the same way.
        assert_eq!(
            auth.read(1, &token, "/c.txt", &read).unwrap_err(),
            native_t("desktop.picker.error.notSelected", &[])
        );
    }

    #[test]
    fn another_sender_cannot_use_the_token() {
        let auth = PickedFileAuthorizations::new(MAX_ATTACHMENT_BYTES);
        let token = auth.add(1, &token_paths());
        let read = |_: &str, _: u64| Ok(b"hello".to_vec());
        assert_eq!(
            auth.read(2, &token, "/a.txt", &read).unwrap_err(),
            native_t("desktop.picker.error.notSelected", &[])
        );
    }

    #[test]
    fn the_token_is_dropped_once_its_last_path_is_consumed() {
        let auth = PickedFileAuthorizations::new(MAX_ATTACHMENT_BYTES);
        let token = auth.add(1, &token_paths());
        let read = |_: &str, _: u64| Ok(vec![0u8; 10]);
        assert!(!auth.is_empty());
        auth.read(1, &token, "/a.txt", &read).unwrap();
        assert!(!auth.is_empty(), "one path remains");
        auth.read(1, &token, "/b.txt", &read).unwrap();
        assert!(auth.is_empty());
    }

    #[test]
    fn release_is_sender_scoped() {
        let auth = PickedFileAuthorizations::new(MAX_ATTACHMENT_BYTES);
        let token = auth.add(1, &token_paths());
        auth.release(2, &token);
        assert!(!auth.is_empty(), "a foreign release is a no-op");
        auth.release(1, &token);
        assert!(auth.is_empty());
    }

    #[test]
    fn the_budget_only_counts_successful_reads() {
        let auth = PickedFileAuthorizations::new(16);
        let token = auth.add(1, &token_paths());
        let seen = std::cell::RefCell::new(Vec::new());
        let read = |path: &str, remaining: u64| {
            seen.borrow_mut().push((path.to_string(), remaining));
            Ok(b"0123456789".to_vec())
        };
        auth.read(1, &token, "/a.txt", &read).unwrap();
        assert_eq!(seen.borrow()[0], ("/a.txt".to_string(), 16));
        // Ten bytes were read, so six remain for the next read.
        auth.read(1, &token, "/b.txt", &read).unwrap();
        assert_eq!(seen.borrow()[1], ("/b.txt".to_string(), 6));
    }

    #[test]
    fn attachment_totals_are_capped_at_twenty_mebibytes() {
        assert!(assert_attachment_budget(&[1024, 2048]).is_ok());
        assert!(assert_attachment_budget(&[MAX_ATTACHMENT_BYTES]).is_ok());
        assert_eq!(
            assert_attachment_budget(&[MAX_ATTACHMENT_BYTES + 1]),
            Err(size_limit_message())
        );
    }

    #[test]
    fn reading_stops_at_the_size_cap_and_the_short_read() {
        assert_eq!(
            read_attachment_from(5, 4, |_| unreachable!()),
            Err(size_limit_message())
        );
        let bytes = read_attachment_from(6, 6, |offset| {
            Ok((vec![b'a'; 3], if offset < 3 { 3 } else { 0 }))
        })
        .unwrap();
        assert_eq!(bytes, b"aaa");
    }
}
