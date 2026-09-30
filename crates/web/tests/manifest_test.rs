// Parity tests: the manifest must describe exactly the `.mdx` files on disk.
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use web::manifest;

fn docs_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("content/docs")
}

fn mdx_files() -> Vec<(String, String, PathBuf)> {
    // (locale, slug, absolute path); locale is "root" for top-level files.
    let mut out = Vec::new();
    let root = docs_dir();
    for entry in fs::read_dir(&root).expect("content/docs must exist") {
        let entry = entry.expect("readable dir entry");
        let path = entry.path();
        if path.is_dir() {
            let locale = path
                .file_name()
                .expect("dir name")
                .to_string_lossy()
                .to_string();
            for file in fs::read_dir(&path).expect("readable locale dir") {
                let file = file.expect("readable file entry");
                let name = file.file_name().to_string_lossy().to_string();
                if name.ends_with(".mdx") {
                    out.push((
                        locale.clone(),
                        name[..name.len() - 4].to_string(),
                        file.path(),
                    ));
                }
            }
        } else if path.extension().map(|ext| ext == "mdx").unwrap_or(false) {
            let name = path
                .file_name()
                .expect("file name")
                .to_string_lossy()
                .to_string();
            out.push(("root".to_string(), name[..name.len() - 4].to_string(), path));
        }
    }
    out.sort();
    out
}

fn frontmatter(path: &Path) -> (String, String) {
    let text = fs::read_to_string(path).expect("readable mdx");
    assert!(
        text.starts_with("---\n"),
        "frontmatter block in {}",
        path.display()
    );
    let end = text.find("\n---").expect("terminated frontmatter");
    let mut title = None;
    let mut description = None;
    for line in text[4..end].split('\n') {
        if let Some(value) = line.strip_prefix("title:") {
            title = Some(unquote(value.trim()));
        }
        if let Some(value) = line.strip_prefix("description:") {
            description = Some(unquote(value.trim()));
        }
    }
    (
        title.expect("title in frontmatter"),
        description.expect("description in frontmatter"),
    )
}

fn unquote(value: &str) -> String {
    if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
        value[1..value.len() - 1].to_string()
    } else {
        value.to_string()
    }
}

#[test]
fn total_page_count_matches_source_tree() {
    assert_eq!(manifest::TOTAL_PAGES, 614);
    assert_eq!(manifest::PAGES.len(), manifest::TOTAL_PAGES);
    assert_eq!(mdx_files().len(), manifest::TOTAL_PAGES);
}

#[test]
fn per_locale_counts_match_source_tree() {
    let expected: HashMap<&str, usize> = manifest::LOCALE_PAGE_COUNTS.iter().copied().collect();
    assert_eq!(expected["root"], 36);
    for locale in manifest::LOCALES.iter().filter(|locale| **locale != "root") {
        assert_eq!(expected[locale], 34, "locale {}", locale);
    }
    let mut actual: HashMap<String, usize> = HashMap::new();
    for (locale, _, _) in mdx_files() {
        *actual.entry(locale).or_default() += 1;
    }
    for (locale, count) in manifest::LOCALE_PAGE_COUNTS {
        assert_eq!(
            actual.get(locale).copied().unwrap_or(0),
            count,
            "locale {}",
            locale
        );
    }
    assert_eq!(actual.len(), manifest::LOCALES.len());
}

#[test]
fn every_manifest_page_has_a_file_and_every_file_has_a_manifest_page() {
    let root = docs_dir();
    let mut on_disk: HashSet<(String, String)> = HashSet::new();
    for (locale, slug, path) in mdx_files() {
        on_disk.insert((locale, slug));
        let _ = path;
    }
    for page in manifest::PAGES {
        let full = Path::new(env!("CARGO_MANIFEST_DIR")).join(page.path);
        assert!(full.is_file(), "copied file for {}", page.path);
        assert!(
            on_disk.remove(&(page.locale.to_string(), page.slug.to_string())),
            "manifest page present on disk: {}/{}",
            page.locale,
            page.slug
        );
        let expected = if page.locale == "root" {
            root.join(format!("{}.mdx", page.slug))
        } else {
            root.join(page.locale).join(format!("{}.mdx", page.slug))
        };
        assert_eq!(full, expected, "manifest path mirrors source tree");
    }
    assert!(on_disk.is_empty(), "no unlisted files: {:?}", on_disk);
}

#[test]
fn frontmatter_matches_manifest_verbatim() {
    for page in manifest::PAGES {
        let full = Path::new(env!("CARGO_MANIFEST_DIR")).join(page.path);
        let (title, description) = frontmatter(&full);
        assert_eq!(title, page.title, "title of {}", page.path);
        assert_eq!(
            description, page.description,
            "description of {}",
            page.path
        );
    }
}

#[test]
fn lookups_cover_every_locale_and_slug() {
    for locale in manifest::LOCALES {
        let pages = manifest::pages_for_locale(locale);
        assert_eq!(pages.len(), manifest::count_for_locale(locale));
        assert!(pages.iter().all(|page| page.locale == locale));
    }
    assert_eq!(manifest::count_for_locale("xx"), 0);
    assert!(manifest::pages_for_locale("xx").is_empty());
    assert_eq!(manifest::slugs().len(), 36);
    assert!(manifest::slugs().contains(&"index"));
    assert!(manifest::slugs().contains(&"policies"));
    assert!(manifest::slugs().contains(&"references"));

    let page = manifest::find("de", "config").expect("de/config exists");
    assert_eq!(page.entry_id(), "de/config");
    assert_eq!(page.url_path(), "/docs/de/config/");
    assert!(!page.is_default_locale());

    let root = manifest::find("root", "config").expect("root/config exists");
    assert_eq!(root.entry_id(), "config");
    assert_eq!(root.url_path(), "/docs/config/");
    assert!(root.is_default_locale());

    let index = manifest::find("root", "index").expect("root/index exists");
    assert_eq!(index.url_path(), "/docs/");
    let ja_index = manifest::find("ja", "index").expect("ja/index exists");
    assert_eq!(ja_index.url_path(), "/docs/ja/");

    assert!(manifest::find("root", "missing").is_none());
    assert!(manifest::find("uk", "index").is_none());
}

#[test]
fn untranslated_slugs_have_no_locale_copies() {
    for slug in ["policies", "references"] {
        assert!(manifest::find("root", slug).is_some());
        for locale in manifest::LOCALES.iter().filter(|locale| **locale != "root") {
            assert!(
                manifest::find(locale, slug).is_none(),
                "{}/{}",
                locale,
                slug
            );
        }
    }
}
