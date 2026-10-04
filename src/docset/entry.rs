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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    name: String,
    kind: EntryType,
    /// The raw path exactly as recorded in the index. May still carry a
    /// `#anchor` fragment or a Dash `<dash_entry_*>` marker -- turning this
    /// into an actual filesystem path plus anchor is M10's job, not this one's.
    path: String,
}

/// Why a raw `(name, type, path)` index row could not become an `Entry`.
///
/// `type` is never a source of failure here: `EntryType::from` is total, so an
/// empty or unrecognized type string just becomes `EntryType::Other` instead
/// of an error.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EntryError {
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
        }
    }
}

impl TryFrom<(&str, &str, &str)> for Entry {
    type Error = EntryError;

    /// Converts a raw `(name, type, path)` index row into an `Entry`.
    ///
    /// # Errors
    ///
    /// Returns `EntryError::EmptyName` if `name` is empty, or
    /// `EntryError::EmptyPath` if `path` is empty.
    fn try_from((name, raw_type, path): (&str, &str, &str)) -> Result<Self, Self::Error> {
        if name.is_empty() {
            return Err(EntryError::EmptyName);
        }
        if path.is_empty() {
            return Err(EntryError::EmptyPath);
        }
        Ok(Self {
            name: name.to_owned(),
            path: path.to_owned(),
            kind: EntryType::from(raw_type),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_row_converts_to_entry() {
        let entry = Entry::try_from(("foo", "func", "baz")).unwrap();
        assert_eq!(entry.name, "foo");
        assert_eq!(entry.kind, EntryType::Function);
        assert_eq!(entry.path, "baz");
    }

    #[test]
    fn empty_name_is_rejected() {
        let error = Entry::try_from(("", "bar", "baz")).unwrap_err();
        assert_eq!(error, EntryError::EmptyName);
    }

    #[test]
    fn empty_path_is_rejected() {
        let error = Entry::try_from(("foo", "bar", "")).unwrap_err();
        assert_eq!(error, EntryError::EmptyPath);
    }

    #[test]
    fn collecting_valid_rows_yields_all_entries() {
        let data = [
            ("foo1", "bar", "baz"),
            ("foo2", "bar", "baz"),
            ("foo3", "bar", "baz"),
            ("foo4", "bar", "baz"),
            ("foo5", "bar", "baz"),
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
            ("foo", "bar", "baz"),
            // First error
            ("foo", "bar", ""),
            ("foo", "bar", "baz"),
            // Second error because we shouldn't make it this far
            ("", "bar", "baz"),
            ("foo", "bar", "baz"),
        ];
        let error = data
            .into_iter()
            .map(Entry::try_from)
            .collect::<Result<Vec<_>, _>>()
            .unwrap_err();
        assert_eq!(error, EntryError::EmptyPath);
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
}
