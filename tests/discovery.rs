use std::path::{Path, PathBuf};

use dashtui::docset::{Docset, find_docsets};

fn fixture(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(rel)
}

#[test]
fn finds_docsets_recursively() {
    let root = fixture("docsets_root");
    let found = find_docsets(&root).unwrap();
    let mut paths: Vec<PathBuf> = found
        .iter()
        .map(|d: &Docset| d.path().to_path_buf())
        .collect();
    paths.sort();

    let mut expected = vec![
        root.join("Bash.docset"),
        root.join("Rust.docset"),
        root.join("nested/Nested.docset"),
    ];
    expected.sort();

    assert_eq!(paths, expected);
}

#[test]
fn does_not_descend_into_a_found_docset() {
    // Bash.docset/FakeNested.docset exists in the fixture tree but must not
    // appear in the results: once a `.docset` dir is found, we don't look
    // inside it for more.
    let root = fixture("docsets_root");
    let found = find_docsets(&root).unwrap();

    assert!(
        !found
            .iter()
            .any(|docset| docset.path().ends_with("FakeNested.docset"))
    );
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

#[cfg(unix)]
mod permissions {
    use super::*;
    use std::{
        fs::{Permissions, create_dir},
        os::unix::fs::PermissionsExt,
    };

    struct Unlock(PathBuf);

    impl Drop for Unlock {
        fn drop(&mut self) {
            let _ = std::fs::set_permissions(&self.0, Permissions::from_mode(0o755));
        }
    }

    #[test]
    fn skips_directory_with_no_permissions() {
        use is_root::is_root;
        use tempfile::tempdir;

        assert!(!is_root(), "Test cannot be run by root user");

        let temproot = tempdir().unwrap();
        let unreachable = temproot.path().join("unreachable");
        let docset = temproot.path().join("Fixture.docset");

        create_dir(&unreachable).unwrap();
        std::fs::set_permissions(&unreachable, Permissions::from_mode(0o000)).unwrap();
        let _unlock = Unlock(unreachable);

        create_dir(&docset).unwrap();

        let results: Vec<PathBuf> = find_docsets(temproot.path())
            .unwrap()
            .into_iter()
            .map(|a| a.path().to_path_buf())
            .collect();

        let expected = vec![docset];

        assert_eq!(results, expected);
    }

    #[test]
    fn errors_if_no_permissions_to_search_root() {
        use std::io::ErrorKind::PermissionDenied;

        use is_root::is_root;
        use tempfile::tempdir;

        assert!(!is_root(), "Test cannot be run by root user");

        let temproot = tempdir().unwrap();

        std::fs::set_permissions(temproot.path(), Permissions::from_mode(0o000)).unwrap();
        let _unlock = Unlock(temproot.path().to_path_buf());

        let error = find_docsets(temproot.path()).unwrap_err();
        assert_eq!(error.kind(), PermissionDenied);
    }
}
