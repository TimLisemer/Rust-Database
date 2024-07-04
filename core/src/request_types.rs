use serde::{Deserialize, Serialize};
use crate::column::Column;
use crate::row::Row;

#[derive(Deserialize)]
pub struct CreateRequests {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateTableRequests {
    pub name: String,
    pub insert_column_requests: Vec<InsertColumnRequest>
}

impl CreateTableRequests {
    pub fn new(name: String) -> Self {
        CreateTableRequests {
            name,
            insert_column_requests: Vec::new()
        }
    }
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
pub struct InsertColumnRequest {
    pub table_name: String,
    pub key: String,
    pub primary_key: bool,
    pub non_null: bool,
    pub unique: bool,
    pub foreign_key: Option<Vec<Column>>,
}

#[derive(Deserialize, Serialize)]
pub struct InsertRowRequest {
    pub table_name: String,
    pub row: Row,
}