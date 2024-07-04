use crate::column::Column;
use crate::row::Row;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
}

impl Table {
    pub fn new(name: String) -> Self {
        Table {
            name,
            columns: Vec::new(),
            rows: Vec::new()
        }
    }

    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column);
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }
}
