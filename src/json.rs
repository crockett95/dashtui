use std::collections::BTreeMap;

use serde::Serialize;

use crate::docset::{
    Docset,
    entry::{Entry, EntryType},
};

#[derive(Debug, Serialize)]
pub struct DiscoveryView {
    pub docsets: Vec<Docset>,
}

#[derive(Debug, Serialize)]
pub struct StatsView {
    pub docset: Docset,
    pub counts: BTreeMap<EntryType, usize>,
}

#[derive(Debug, Serialize)]
pub struct SearchView<'a> {
    pub docset: &'a Docset,
    pub query: &'a str,
    pub results: Vec<&'a Entry>,
}

impl Serialize for EntryType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;

    use crate::{
        docset::{Docset, entry::RawEntry, meta::DocsetMeta},
        json::DiscoveryView,
    };

    #[test]
    fn serializes_an_empty_vec() {
        let dut = DiscoveryView { docsets: vec![] };
        let serialized = serde_json::to_string(&dut);
        assert!(serialized.is_ok());
        assert_eq!(r#"{"docsets":[]}"#, serialized.unwrap());
    }

    #[test]
    fn serializes_a_path() {
        let dut = DiscoveryView {
            docsets: vec![Docset::from((PathBuf::from("./docset/"), None))],
        };
        let serialized = serde_json::to_string(&dut);
        assert_eq!(r#"{"docsets":[{"path":"./docset/"}]}"#, serialized.unwrap());
    }

    #[test]
    fn serializes_metadata() {
        let dut = DiscoveryView {
            docsets: vec![Docset::from((
                PathBuf::from("./docset/"),
                Some(DocsetMeta {
                    name: "Foo".to_owned(),
                    id: "foo".to_owned(),
                    platform_family: "foo".to_owned(),
                    index_file: None,
                }),
            ))],
        };
        let serialized = serde_json::to_string(&dut);
        assert_eq!(
            r#"{"docsets":[{"path":"./docset/","id":"foo","name":"Foo","platform_family":"foo","index_file":null}]}"#,
            serialized.unwrap()
        );
    }

    #[test]
    fn entry_types_serialize_to_their_string_name() {
        assert_eq!(
            format!("\"{}\"", EntryType::Builtin),
            serde_json::to_string(&EntryType::Builtin).unwrap()
        );
        assert_eq!(
            format!("\"{}\"", EntryType::Function),
            serde_json::to_string(&EntryType::Function).unwrap()
        );
        assert_eq!(
            format!("\"{}\"", EntryType::Guide),
            serde_json::to_string(&EntryType::Guide).unwrap()
        );
        assert_eq!(
            format!("\"{}\"", EntryType::Parameter),
            serde_json::to_string(&EntryType::Parameter).unwrap()
        );
        assert_eq!(
            format!("\"{}\"", EntryType::Variable),
            serde_json::to_string(&EntryType::Variable).unwrap()
        );
        assert_eq!(
            format!("\"{}\"", EntryType::Word),
            serde_json::to_string(&EntryType::Word).unwrap()
        );
        assert_eq!(
            format!("\"{}\"", EntryType::Macro),
            serde_json::to_string(&EntryType::Macro).unwrap()
        );
    }

    #[test]
    fn other_types_serialize_to_their_inner_string() {
        assert_eq!(
            format!("\"{}\"", String::new()),
            serde_json::to_string(&EntryType::Other(String::new())).unwrap()
        );
        assert_eq!(
            format!("\"{}\"", String::from("name")),
            serde_json::to_string(&EntryType::Other(String::from("name"))).unwrap()
        );
        assert_eq!(
            format!("\"{}\"", String::from("another")),
            serde_json::to_string(&EntryType::Other(String::from("another"))).unwrap()
        );
        assert_eq!(
            format!("\"{}\"", String::from("third")),
            serde_json::to_string(&EntryType::Other(String::from("third"))).unwrap()
        );
    }

    #[test]
    fn stats_serializes_with_its_docset() {
        let docset = Docset::from((PathBuf::from("path/To.docset"), None));
        let counts = BTreeMap::from([(EntryType::Function, 1), (EntryType::Guide, 3)]);

        let json = serde_json::to_string(&StatsView { docset, counts }).unwrap();
        assert_eq!(
            r#"{"docset":{"path":"path/To.docset"},"counts":{"Function":1,"Guide":3}}"#,
            json
        );
    }

    #[test]
    fn entry_serializes_with_anchor() {
        let entry = Entry::try_from(RawEntry {
            name: "name",
            kind: "type",
            path: "path/to/docs",
            anchor: Some("anchor"),
        })
        .unwrap();
        let json = serde_json::to_string(&entry).unwrap();
        assert_eq!(
            json,
            r#"{"name":"name","type":"type","path":"path/to/docs","anchor":"anchor"}"#
        )
    }

    #[test]
    fn entry_serializes_without_anchor() {
        let entry = Entry::try_from(RawEntry {
            name: "name",
            kind: "type",
            path: "path/to/docs",
            anchor: None,
        })
        .unwrap();
        let json = serde_json::to_string(&entry).unwrap();
        assert_eq!(
            json,
            r#"{"name":"name","type":"type","path":"path/to/docs","anchor":null}"#
        )
    }

    #[test]
    fn search_view_serializes_with_docset_and_query() {
        let entry1 = Entry::try_from(RawEntry {
            name: "result1",
            kind: "func",
            path: "path/to/docs",
            anchor: None,
        })
        .unwrap();
        let entry2 = Entry::try_from(RawEntry {
            name: "result2",
            kind: "type",
            path: "path/to/other",
            anchor: Some("anchor"),
        })
        .unwrap();
        let docset = Docset::from((PathBuf::from("path/to/docset"), None));
        let results = SearchView {
            docset: &docset,
            query: "res",
            results: vec![&entry1, &entry2],
        };

        let json = serde_json::to_string(&results).unwrap();
        assert_eq!(
            json,
            r#"{"docset":{"path":"path/to/docset"},"query":"res","results":[{"name":"result1","type":"Function","path":"path/to/docs","anchor":null},{"name":"result2","type":"type","path":"path/to/other","anchor":"anchor"}]}"#
        )
    }
}
