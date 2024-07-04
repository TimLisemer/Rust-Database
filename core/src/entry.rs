use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Entry {
    pub key: String,
    pub value: String,
    pub primary_key: bool,
    pub non_null: bool,
    pub unique: bool,
    pub foreign_key: Option<Vec<Box<Entry>>>,
}

impl Entry {
    pub fn new(
        key: String,
        value: String,
        primary_key: bool,
        non_null: bool,
        unique: bool,
        foreign_key: Option<Vec<Box<Entry>>>,
    ) -> Self {
        if primary_key && (non_null == false || value.is_empty()) {
            panic!("Primary Key cannot be null or empty!");
        }

        if primary_key && !unique {
            panic!("Primary Key has to be unique!");
        }

        Self {
            key,
            value,
            primary_key,
            non_null,
            unique,
            foreign_key,
        }
    }
}
