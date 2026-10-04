use std::collections::BTreeMap;

use crate::docset::entry::{Entry, EntryType};

/// Counts the entries per distinct [`EntryType`] in a collection of docset [`Entry`]s.
///
/// Given a slice of [`Entry`] objects, accumulates the counts of each type
/// ([`Entry::kind`]) of entry in the collection, keyed and ordered by [`EntryType`].
pub fn count_types(entries: &[Entry]) -> BTreeMap<EntryType, usize> {
    let mut counts = BTreeMap::new();
    for entry in entries {
        if let Some(kind_count) = counts.get_mut(entry.kind()) {
            *kind_count += 1;
        } else {
            counts.insert(entry.kind().clone(), 1);
        }
    }
    counts
}

#[cfg(test)]
mod test {
    use super::*;

    /// Check that all known types (as of the writing of this test) produce
    /// their own separate count.
    #[test]
    fn counts_each_type_separately() {
        let entries = [
            Entry::fixture_with_kind(EntryType::Function),
            Entry::fixture_with_kind(EntryType::Macro),
            Entry::fixture_with_kind(EntryType::Builtin),
            Entry::fixture_with_kind(EntryType::Guide),
            Entry::fixture_with_kind(EntryType::Variable),
            Entry::fixture_with_kind(EntryType::Parameter),
            Entry::fixture_with_kind(EntryType::Word),
            Entry::fixture_with_kind(EntryType::Other(String::new())),
        ];
        let expected = BTreeMap::from([
            (EntryType::Builtin, 1),
            (EntryType::Function, 1),
            (EntryType::Guide, 1),
            (EntryType::Macro, 1),
            (EntryType::Parameter, 1),
            (EntryType::Variable, 1),
            (EntryType::Word, 1),
            (EntryType::Other(String::new()), 1),
        ]);

        let counts = count_types(&entries);

        assert_eq!(counts, expected);
    }

    /// Verify that multiple entries with a duplicate type are counted together
    /// and drive up the count.
    #[test]
    fn counts_the_same_type_together() {
        let entries = [
            Entry::fixture_with_kind(EntryType::Function),
            Entry::fixture_with_kind(EntryType::Variable),
            Entry::fixture_with_kind(EntryType::Function),
            Entry::fixture_with_kind(EntryType::Macro),
            Entry::fixture_with_kind(EntryType::Macro),
            Entry::fixture_with_kind(EntryType::Function),
            Entry::fixture_with_kind(EntryType::Builtin),
            Entry::fixture_with_kind(EntryType::Function),
            Entry::fixture_with_kind(EntryType::Variable),
            Entry::fixture_with_kind(EntryType::Function),
            Entry::fixture_with_kind(EntryType::Builtin),
            Entry::fixture_with_kind(EntryType::Guide),
            Entry::fixture_with_kind(EntryType::Guide),
            Entry::fixture_with_kind(EntryType::Guide),
            Entry::fixture_with_kind(EntryType::Builtin),
            Entry::fixture_with_kind(EntryType::Parameter),
            Entry::fixture_with_kind(EntryType::Word),
            Entry::fixture_with_kind(EntryType::Other(String::new())),
        ];
        let expected = BTreeMap::from([
            (EntryType::Builtin, 3),
            (EntryType::Function, 5),
            (EntryType::Guide, 3),
            (EntryType::Macro, 2),
            (EntryType::Parameter, 1),
            (EntryType::Variable, 2),
            (EntryType::Word, 1),
            (EntryType::Other(String::new()), 1),
        ]);

        let counts = count_types(&entries);

        assert_eq!(counts, expected);
    }

    /// Verify that entries with [`Entry::kind`] of [`EntryType::Other`] are
    /// not compacted into a single count.
    #[test]
    fn counts_other_types_uniquely_by_name() {
        let entries = [
            Entry::fixture_with_kind(EntryType::Other(String::from("A"))),
            Entry::fixture_with_kind(EntryType::Other(String::from("B"))),
            Entry::fixture_with_kind(EntryType::Other(String::from("C"))),
            Entry::fixture_with_kind(EntryType::Other(String::from("D"))),
            Entry::fixture_with_kind(EntryType::Other(String::from("E"))),
        ];
        let expected = BTreeMap::from([
            (EntryType::Other(String::from("A")), 1),
            (EntryType::Other(String::from("B")), 1),
            (EntryType::Other(String::from("C")), 1),
            (EntryType::Other(String::from("D")), 1),
            (EntryType::Other(String::from("E")), 1),
        ]);

        let counts = count_types(&entries);

        assert_eq!(counts, expected);
    }

    /// Verify that entries with [`Entry::kind`] of [`EntryType::Other`] are
    /// counted together if their `String` variant is the same.
    #[test]
    fn counts_other_types_with_the_same_name_together() {
        let entries = [
            Entry::fixture_with_kind(EntryType::Other(String::from("B"))),
            Entry::fixture_with_kind(EntryType::Other(String::from("A"))),
            Entry::fixture_with_kind(EntryType::Other(String::from("A"))),
            Entry::fixture_with_kind(EntryType::Other(String::from("A"))),
            Entry::fixture_with_kind(EntryType::Other(String::from("B"))),
            Entry::fixture_with_kind(EntryType::Other(String::from("C"))),
            Entry::fixture_with_kind(EntryType::Other(String::from("C"))),
            Entry::fixture_with_kind(EntryType::Other(String::from("C"))),
            Entry::fixture_with_kind(EntryType::Other(String::from("D"))),
            Entry::fixture_with_kind(EntryType::Other(String::from("E"))),
            Entry::fixture_with_kind(EntryType::Other(String::from("E"))),
            Entry::fixture_with_kind(EntryType::Other(String::from("A"))),
            Entry::fixture_with_kind(EntryType::Other(String::from("E"))),
        ];
        let expected = BTreeMap::from([
            (EntryType::Other(String::from("A")), 4),
            (EntryType::Other(String::from("B")), 2),
            (EntryType::Other(String::from("C")), 3),
            (EntryType::Other(String::from("D")), 1),
            (EntryType::Other(String::from("E")), 3),
        ]);
        let counts = count_types(&entries);
        assert_eq!(counts, expected);
    }

    /// Verify that the function produces an empty map, given an empty dataset.
    #[test]
    fn handles_empty_slices() {
        let counts = count_types(&[]);
        assert_eq!(counts, BTreeMap::new());
    }
}
