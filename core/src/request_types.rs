use serde::{Deserialize, Serialize};
use crate::column::Column;
use crate::row::Row;

/// Represents a request to create a new table.
#[derive(Deserialize)]
pub struct CreateRequests {
    pub name: String,
}

/// Represents a request to create a new table with columns.
#[derive(Deserialize, Serialize)]
pub struct CreateTableRequests {
    pub name: String,
    pub insert_column_requests: Vec<InsertColumnRequest>
}

impl CreateTableRequests {
    /// Creates a new `CreateTableRequests` instance.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the table to create.
    pub fn new(name: String) -> Self {
        CreateTableRequests {
            name,
            insert_column_requests: Vec::new()
        }
    }
}

/// Represents a request to drop a table.
#[derive(Deserialize, Serialize)]
pub struct DropTableRequest {
    pub name: String,
}

/// Represents a request to update a table's name.
#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateTableRequest {
    pub current_name: String,
    pub new_name: String,
}

/// Represents a request to insert a new column into a table.
#[derive(Deserialize, Serialize, Debug)]
pub struct InsertColumnRequest {
    pub table_name: String,
    pub key: String,
    pub primary_key: bool,
    pub non_null: bool,
    pub unique: bool,
    pub foreign_key: Option<Vec<Column>>,
}

/// Represents a request to insert a new row into a table.
#[derive(Deserialize, Serialize, Debug)]
pub struct InsertRowRequest {
    pub table_name: String,
    pub row: Row,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SelectRequest {
    pub columns: Option<Vec<String>>,  // None means SELECT *
    pub table_name: String,
    pub condition: Option<Condition>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Condition {
    pub column: String,
    pub value: String,
}