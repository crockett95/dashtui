use serde::Serialize;

use crate::docset::Docset;

#[derive(Debug, Serialize)]
pub struct DiscoveryView {
    pub docsets: Vec<Docset>,
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

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
}
