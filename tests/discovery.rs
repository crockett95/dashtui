use std::path::{Path, PathBuf};

use dashtui::docset::find_docsets;

fn fixture(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(rel)
}

#[test]
fn finds_docsets_recursively() {
    let root = fixture("docsets_root");
    let mut found = find_docsets(&root).unwrap();
    found.sort();

    let mut expected = vec![
        root.join("Bash.docset"),
        root.join("Rust.docset"),
        root.join("nested/Nested.docset"),
    ];
    expected.sort();

    assert_eq!(found, expected);
}

#[test]
fn does_not_descend_into_a_found_docset() {
    // Bash.docset/FakeNested.docset exists in the fixture tree but must not
    // appear in the results: once a `.docset` dir is found, we don't look
    // inside it for more.
    let root = fixture("docsets_root");
    let found = find_docsets(&root).unwrap();

    assert!(!found.contains(&root.join("Bash.docset/FakeNested.docset")));
}

#[test]
fn returns_empty_vec_when_no_docsets_present() {
    let root = fixture("docsets_root/not_a_docset_dir");
    let found = find_docsets(&root).unwrap();

    assert!(found.is_empty());
}

#[test]
fn errors_on_missing_directory() {
    let root = fixture("does_not_exist");
    assert!(find_docsets(&root).is_err());
}
