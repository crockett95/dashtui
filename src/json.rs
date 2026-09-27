use std::path::PathBuf;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DiscoveryView {
    pub docsets: Vec<PathBuf>,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::json::DiscoveryView;

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
            docsets: vec![PathBuf::from("./docset/")],
        };
        let serialized = serde_json::to_string(&dut);
        assert!(serialized.is_ok());
        assert_eq!("{\"docsets\":[\"./docset/\"]}", serialized.unwrap());
    }
}
