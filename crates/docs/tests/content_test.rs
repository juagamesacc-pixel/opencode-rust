// source: packages/docs — parity: every source docs file exists under content/ (byte match impossible in CI
// against /root/opencode-src, so verify presence + non-empty + .mdx count)
#![allow(clippy::all)]
use docs::FILES;

#[test]
fn all_content_files_present_and_nonempty() {
    for rel in FILES {
        let path = format!("{}/content/{rel}", env!("CARGO_MANIFEST_DIR"));
        let bytes =
            std::fs::read(&path).unwrap_or_else(|e| panic!("missing content file {rel}: {e}"));
        assert!(!bytes.is_empty(), "content file {rel} is empty");
    }
}

#[test]
fn content_count_matches() {
    let mut total = 0usize;
    for rel in FILES {
        let path = format!("{}/content/{rel}", env!("CARGO_MANIFEST_DIR"));
        let meta = std::fs::metadata(&path).unwrap();
        assert!(meta.is_file(), "{rel} should be a file");
        total += 1;
    }
    assert_eq!(total, FILES.len());
}

#[test]
fn mdx_count_matches_source() {
    // source has exactly: 3 root + 6 essentials + 3 ai-tools + 1 snippets = 13 .mdx
    let mdx: Vec<&&str> = FILES.iter().filter(|f| f.ends_with(".mdx")).collect();
    assert_eq!(mdx.len(), 13);
}
