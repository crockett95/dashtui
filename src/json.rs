use std::collections::BTreeMap;

use serde::Serialize;

use crate::docset::{Docset, entry::EntryType};

#[derive(Debug, Serialize)]
pub struct DiscoveryView {
    pub docsets: Vec<Docset>,
}

#[derive(Debug, Serialize)]
pub struct StatsView {
    pub docset: Docset,
    pub counts: BTreeMap<EntryType, usize>,
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
        docset::{Docset, meta::DocsetMeta},
        json::DiscoveryView,
    };

    #[test]
    fn serializes_an_empty_vec() {
        let dut = DiscoveryView { docsets: vec![] };
        let serialized = serde_json::to_string(&dut);
        assert!(serialized.is_ok());
        assert_eq!("{\"docsets\":[]}", serialized.unwrap());
    }

    #[test]
    fn serializes_a_path() {
        let dut = DiscoveryView {
            docsets: vec![Docset {
                path: PathBuf::from("./docset/"),
                meta: None,
            }],
        };
        let serialized = serde_json::to_string(&dut);
        assert_eq!(
            "{\"docsets\":[{\"path\":\"./docset/\"}]}",
            serialized.unwrap()
        );
    }

    #[test]
    fn serializes_metadata() {
        let dut = DiscoveryView {
            docsets: vec![Docset {
                path: PathBuf::from("./docset/"),
                meta: Some(DocsetMeta {
                    name: "Foo".to_owned(),
                    id: "foo".to_owned(),
                    platform_family: "foo".to_owned(),
                    index_file: None,
                }),
            }],
        };
        let serialized = serde_json::to_string(&dut);
        assert_eq!(
            "{\"docsets\":[{\"path\":\"./docset/\",\"id\":\"foo\",\"name\":\"Foo\",\"platform_family\":\"foo\",\"index_file\":null}]}",
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
        let docset = Docset {
            path: PathBuf::from("path/To.docset"),
            meta: None,
        };
        let counts = BTreeMap::from([(EntryType::Function, 1), (EntryType::Guide, 3)]);

        let json = serde_json::to_string(&StatsView { docset, counts }).unwrap();
        assert_eq!(
            "{\"docset\":{\"path\":\"path/To.docset\"},\"counts\":{\"Function\":1,\"Guide\":3}}",
            json
        );
    }
}
