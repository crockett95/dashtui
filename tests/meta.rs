use std::path::{Path, PathBuf};

use dashtui::docset::meta::{
    DocsetMeta,
    DocsetMetaError::{Io, Plist},
};

/// Path to one of the docset fixtures under `tests/fixtures/meta/`.
fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/meta")
        .join(name)
}

/// `full.docset`'s Info.plist has all four keys DocsetMeta reads:
/// CFBundleIdentifier, CFBundleName, DocSetPlatformFamily, dashIndexFilePath.
/// Parsing it should succeed, and every field on the result should match
/// what's in the fixture file (open it and look).
#[test]
fn parses_every_field_when_present() {
    let fix = fixture("full.docset");
    let meta = DocsetMeta::load(&fix);

    let docset_meta = meta.unwrap();
    assert_eq!(docset_meta.name, "Fixture");
    assert_eq!(docset_meta.id, "fixture");
    assert_eq!(docset_meta.platform_family, "fixture");
    assert_eq!(
        docset_meta.index_file,
        Some(std::path::PathBuf::from("fixture/index.html"))
    )
}

/// `no_index_path.docset`'s Info.plist omits dashIndexFilePath, the one
/// field PLAN.md decision 6 says is optional. Parsing should still succeed,
/// with that field coming back `None` rather than an error.
#[test]
fn missing_optional_field_becomes_none() {
    let fix = fixture("no_index_path.docset");
    let meta = DocsetMeta::load(&fix);

    let docset_meta = meta.unwrap();
    assert_eq!(docset_meta.name, "Fixture");
    assert_eq!(docset_meta.id, "fixture");
    assert_eq!(docset_meta.platform_family, "fixture");
    assert_eq!(docset_meta.index_file, None)
}

/// `missing_name.docset`'s Info.plist omits CFBundleName, a required field.
/// Parsing should return an error identifying *which* field is missing
#[test]
fn missing_required_field_is_reported_by_name() {
    let fix = fixture("missing_name.docset");
    let meta = DocsetMeta::load(&fix);

    let error = meta.unwrap_err();
    std::assert_matches!(error, Plist(e) if e.to_string().contains("CFBundleName"))
}

/// `malformed.docset`'s Info.plist is not well-formed XML. Parsing should
/// return an error that came from the plist crate itself
#[test]
fn malformed_plist_is_a_plist_error() {
    let fix = fixture("malformed.docset");
    let meta = DocsetMeta::load(&fix);

    let error = meta.unwrap_err();
    std::assert_matches!(error, Plist(_))
}

/// `missing_plist.docset` has a Contents/ directory but no Info.plist file
/// inside it. Parsing should return an I/O-flavored error
#[test]
fn missing_plist_file_is_an_io_error() {
    let fix = fixture("missing_plist.docset");
    let meta = DocsetMeta::load(&fix);

    let error = meta.unwrap_err();
    std::assert_matches!(error, Io(_))
}
