/// The kind of a docset entry, as recorded in the docset's own index.
///
/// A docset's index has no fixed, machine-checkable vocabulary for entry
/// types (verified against real docsets and their raw SQL schema: the `type`
/// column is a plain, unconstrained `TEXT`, no `CHECK` constraint, no lookup
/// table). Different docset generators spell the same concept differently
/// (`"func"` vs. `"Function"`), and — confirmed by a 23-docset survey done to
/// settle this design — a single docset can even mix several incompatible
/// naming conventions in one file.
///
/// `EntryType` therefore only names the labels our own local docsets
/// (`docsets/`) actually need after normalizing known aliases. Anything else
/// is preserved verbatim in `Other`, so no entry's real type string is ever
/// lost, even though we don't have a dedicated variant for it (yet).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
            "Builtin" => EntryType::Builtin,
            "Function" | "func" => EntryType::Function,
            "Guide" => EntryType::Guide,
            "Parameter" => EntryType::Parameter,
            "Variable" => EntryType::Variable,
            "Word" => EntryType::Word,
            "Macro" | "macro" => EntryType::Macro,
            other => EntryType::Other(other.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_canonical_strings_map_to_their_variant() {
        assert_eq!(EntryType::from("Builtin"), EntryType::Builtin);
        assert_eq!(EntryType::from("Function"), EntryType::Function);
        assert_eq!(EntryType::from("Guide"), EntryType::Guide);
        assert_eq!(EntryType::from("Parameter"), EntryType::Parameter);
        assert_eq!(EntryType::from("Variable"), EntryType::Variable);
        assert_eq!(EntryType::from("Word"), EntryType::Word);
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
            EntryType::Other("some-future-docset-type".to_string())
        );
    }
}
