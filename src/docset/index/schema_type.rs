//! Handle any differences between the two docset schemae in the wild
use rusqlite::{Connection, Statement};

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum SchemaType {
    SearchIndex,
    CoreData,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("invalid schema type")]
    InvalidSchema,
}

impl TryFrom<i64> for SchemaType {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::SearchIndex),
            2 => Ok(Self::CoreData),
            _ => Err(Self::Error::InvalidSchema),
        }
    }
}

impl SchemaType {
    pub fn read_all<'a>(self, conn: &'a Connection) -> rusqlite::Result<Statement<'a>> {
        match self {
            SchemaType::SearchIndex => conn.prepare(include_str!("read_search_index_all.sql")),
            SchemaType::CoreData => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn maps_1_to_search_index() {
        let val = SchemaType::try_from(1).unwrap();
        assert_eq!(SchemaType::SearchIndex, val);
    }

    #[test]
    fn maps_2_to_core_data() {
        let val = SchemaType::try_from(2).unwrap();
        assert_eq!(SchemaType::CoreData, val);
    }

    #[test]
    fn maps_higher_values_to_error() {
        for val in 3..100 {
            let err = SchemaType::try_from(val).unwrap_err();
            std::assert_matches!(err, Error::InvalidSchema);
        }
    }

    #[test]
    fn maps_lower_values_to_error() {
        for val in -100..1 {
            let err = SchemaType::try_from(val).unwrap_err();
            std::assert_matches!(err, Error::InvalidSchema);
        }
    }
}
