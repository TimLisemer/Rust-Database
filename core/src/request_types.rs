use serde::{Deserialize, Serialize};
use crate::entry::Entry;

#[derive(Deserialize)]
pub struct CreateTableRequest {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct DropTableRequest {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateTableRequest {
    pub current_name: String,
    pub new_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct InsertEntryRequest {
    pub table_name: String,
    pub key: String,
    pub value: String,
    pub primary_key: bool,
    pub non_null: bool,
    pub unique: bool,
    pub foreign_key: Option<Vec<Entry>>,
}