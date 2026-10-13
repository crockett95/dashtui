use std::sync::LazyLock;

use regex::Regex;
use serde::Serialize;

/// The kind of a docset entry, as recorded in the docset's own index.
///
/// A docset's index has no fixed, machine-checkable vocabulary for entry
/// types . Different docset generators spell the same concept differently
/// (`"func"` vs. `"Function"`), and a single docset can even mix several
/// incompatible naming conventions in one file.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EntryType {
    Builtin,
    Function,
    Guide,
    Macro,
    Parameter,
    Variable,
    Word,
    /// Any type string that doesn't match a known variant or alias, kept
    /// verbatim exactly as the docset's index recorded it.
    Other(String),
}

impl std::fmt::Display for EntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Builtin => "Builtin",
            Self::Function => "Function",
            Self::Guide => "Guide",
            Self::Macro => "Macro",
            Self::Parameter => "Parameter",
            Self::Variable => "Variable",
            Self::Word => "Word",
            Self::Other(other_type) => other_type,
        };
        f.pad(label)
    }
}

impl From<&str> for EntryType {
    /// Converts a raw `type` string from a docset's index into an
    /// `EntryType`.
    ///
    /// This conversion cannot fail: known aliases (e.g. `"func"`, the
    /// convention `SQLite.docset` and `Lua.docset` both use for what
    /// `Bash.docset` spells `"Function"`) are normalized to their canonical
    /// variant, and anything unrecognized becomes `Other`.
    fn from(raw: &str) -> Self {
        match raw {
            "Builtin" => Self::Builtin,
            "Function" | "func" => Self::Function,
            "Guide" => Self::Guide,
            "Parameter" => Self::Parameter,
            "Variable" => Self::Variable,
            "Word" => Self::Word,
            "Macro" | "macro" => Self::Macro,
            other => Self::Other(other.to_owned()),
        }
    }
}

/// A single row from a docset's index: one named, typed location in its
/// HTML documentation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Entry {
    name: String,
    #[serde(rename(serialize = "type"))]
    kind: EntryType,
    /// The page path from the index, with any leading Dash `<dash_entry_*>`
    /// markers and the `#anchor` removed.
    path: String,
    /// The anchor within the page, without the `#` that introduces it. Core
    /// Data stores it separately (`ZANCHOR`); for SearchIndex it is
    /// everything after the first `#` in the recorded path, so `page.html##id`
    /// gives `#id`. `None` when the entry has no anchor.
    anchor: Option<String>,
}

/// Why a raw `(name, type, path)` index row could not become an `Entry`.
///
/// `type` is never a source of failure here: `EntryType::from` is total, so an
/// empty or unrecognized type string just becomes `EntryType::Other` instead
/// of an error.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("entry name is empty")]
    EmptyName,
    #[error("entry path is empty")]
    EmptyPath,
}

impl Entry {
    /// Returns this entry's normalized type
    pub fn kind(&self) -> &EntryType {
        &self.kind
    }

    /// The entry's name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The entry's page path, without markers or anchor.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// The anchor within the entry's page (see [`Entry::path`]), without the
    /// leading `#`; `None` if the entry points at the page itself.
    pub fn anchor(&self) -> Option<&str> {
        self.anchor.as_deref()
    }

    /// Builds a test fixture Entry with a specified type
    ///
    /// Builds a placeholder Entry fixture with a specified [`EntryType`] and
    /// name "name" and path "path" as hardcoded values
    #[cfg(test)]
    pub fn fixture_with_kind(kind: EntryType) -> Self {
        Self {
            name: String::from("name"),
            kind,
            path: String::from("path"),
            anchor: None,
        }
    }
}

impl AsRef<str> for Entry {
    fn as_ref(&self) -> &str {
        self.name()
    }
}

static PATH_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("^(?<meta>(?:<[^>]*>)*)(?<path>.*)$").expect("Path/metamarker regex invalid")
});

impl TryFrom<(&str, &str, &str, Option<&str>)> for Entry {
    type Error = Error;

    /// Converts a raw `(name, type, path, anchor)` index row into an `Entry`.
    ///
    /// `path` must already have its `#anchor` split off into `anchor`. Any
    /// leading `<...>` groups on `path` are Dash metadata markers (such as
    /// `<dash_entry_name=...>`) and are removed; their values are discarded.
    /// `<` and `>` can't appear unencoded in a real page path, so a leading
    /// `<...>` is never part of one.
    ///
    /// # Errors
    ///
    /// Returns [`Error::EmptyName`] if `name` is empty, or
    /// [`Error::EmptyPath`] if `path` is empty once markers are removed, so a
    /// path made only of markers is rejected. `anchor` never fails: `None`
    /// and any string are both accepted.
    fn try_from(
        (name, raw_type, path, anchor): (&str, &str, &str, Option<&str>),
    ) -> Result<Self, Self::Error> {
        if name.is_empty() {
            return Err(Error::EmptyName);
        }
        if let Some(parts) = PATH_REGEX.captures(path)
            && !parts["path"].is_empty()
        {
            let path = parts["path"].to_string();
            // TODO:consume parts["meta"]
            Ok(Self {
                name: name.to_owned(),
                path,
                kind: EntryType::from(raw_type),
                anchor: anchor.map(String::from),
            })
        } else {
            Err(Error::EmptyPath)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_row_converts_to_entry() {
        let entry = Entry::try_from(("foo", "func", "baz", Some("quux"))).unwrap();
        assert_eq!(entry.name, "foo");
        assert_eq!(entry.kind, EntryType::Function);
        assert_eq!(entry.path, "baz");
        assert_eq!(entry.anchor.as_deref(), Some("quux"))
    }

    #[test]
    fn empty_name_is_rejected() {
        let error = Entry::try_from(("", "bar", "baz", Some("quux"))).unwrap_err();
        assert_eq!(error, Error::EmptyName);
    }

    #[test]
    fn empty_path_is_rejected() {
        let error = Entry::try_from(("foo", "bar", "", Some("quux"))).unwrap_err();
        assert_eq!(error, Error::EmptyPath);
    }

    #[test]
    fn none_anchor_is_not_rejected() {
        let entry = Entry::try_from(("foo", "bar", "baz", None)).unwrap();
        assert_eq!(entry.anchor, None);
    }

    #[test]
    fn collecting_valid_rows_yields_all_entries() {
        let data = [
            ("foo1", "bar", "baz", Some("quux")),
            ("foo2", "bar", "baz", Some("quux")),
            ("foo3", "bar", "baz", Some("quux")),
            ("foo4", "bar", "baz", Some("quux")),
            ("foo5", "bar", "baz", Some("quux")),
        ];
        let entries = data
            .into_iter()
            .map(Entry::try_from)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        // Check that the length and order are correct
        assert_eq!(
            entries
                .iter()
                .map(|e| e.name.as_str())
                .collect::<Vec<&str>>(),
            ["foo1", "foo2", "foo3", "foo4", "foo5"]
        );
    }

    #[test]
    fn collecting_rows_short_circuits_on_first_error() {
        let data = [
            ("foo", "bar", "baz", None),
            // First error
            ("foo", "bar", "", Some("quux")),
            ("foo", "bar", "baz", Some("quux")),
            // Second error because we shouldn't make it this far
            ("", "bar", "baz", None),
            ("foo", "bar", "baz", None),
        ];
        let error = data
            .into_iter()
            .map(Entry::try_from)
            .collect::<Result<Vec<_>, _>>()
            .unwrap_err();
        assert_eq!(error, Error::EmptyPath);
    }

    #[test]
    fn exact_canonical_strings_map_to_their_variant() {
        assert_eq!(EntryType::from("Builtin"), EntryType::Builtin);
        assert_eq!(EntryType::from("Function"), EntryType::Function);
        assert_eq!(EntryType::from("Guide"), EntryType::Guide);
        assert_eq!(EntryType::from("Parameter"), EntryType::Parameter);
        assert_eq!(EntryType::from("Variable"), EntryType::Variable);
        assert_eq!(EntryType::from("Word"), EntryType::Word);
        assert_eq!(EntryType::from("Macro"), EntryType::Macro);
    }

    #[test]
    fn known_aliases_normalize_to_their_canonical_variant() {
        // SQLite.docset and Lua.docset both use "func" where Bash.docset
        // uses "Function" -- verified against the real files, and matches
        // Zeal's own Docset::parseSymbolType alias table.
        assert_eq!(EntryType::from("func"), EntryType::Function);
        // SQLite.docset's spelling of what Bash-style docsets would call Macro.
        assert_eq!(EntryType::from("macro"), EntryType::Macro);
    }

    #[test]
    fn unrecognized_strings_are_preserved_as_other() {
        assert_eq!(
            EntryType::from("some-future-docset-type"),
            EntryType::Other("some-future-docset-type".to_owned())
        );
    }

    #[test]
    fn canonical_types_map_to_their_string_names() {
        assert_eq!("Builtin", EntryType::Builtin.to_string());
        assert_eq!("Function", EntryType::Function.to_string());
        assert_eq!("Guide", EntryType::Guide.to_string());
        assert_eq!("Parameter", EntryType::Parameter.to_string());
        assert_eq!("Variable", EntryType::Variable.to_string());
        assert_eq!("Word", EntryType::Word.to_string());
        assert_eq!("Macro", EntryType::Macro.to_string());
    }

    #[test]
    fn other_types_map_to_index_names() {
        assert_eq!("", EntryType::Other(String::new()).to_string());
        assert_eq!("first", EntryType::Other(String::from("first")).to_string());
        assert_eq!(
            "second",
            EntryType::Other(String::from("second")).to_string()
        );
    }

    #[test]
    fn entry_try_from_removes_path_markers() {
        let path = concat!(
            "<dash_entry_name=document structural>",
            "<dash_entry_originalName=document structural>",
            "<dash_entry_menuDescription=document structural>",
            "developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Roles/structural_roles.html"
        );
        let entry = Entry::try_from(("document structural", "Role", path, None)).unwrap();

        assert_eq!(
            entry.path(),
            "developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Roles/structural_roles.html"
        )
    }

    #[test]
    fn path_with_only_markers_is_empty() {
        let path = concat!(
            "<dash_entry_name=document structural>",
            "<dash_entry_originalName=document structural>",
            "<dash_entry_menuDescription=document structural>",
        );
        let error = Entry::try_from(("document structural", "Role", path, None)).unwrap_err();
        std::assert_matches!(error, Error::EmptyPath);
    }
}
