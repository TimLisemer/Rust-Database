use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Column {
    pub key: String,
    pub primary_key: bool,
    pub non_null: bool,
    pub unique: bool,
    pub foreign_key: Option<Vec<Box<Column>>>,
}

impl Column {
    pub fn new(
        key: String,
        primary_key: bool,
        non_null: bool,
        unique: bool,
        foreign_key: Option<Vec<Box<Column>>>,
    ) -> Self {
        Self {
            key,
            primary_key,
            non_null,
            unique,
            foreign_key,
        }
    }
}
