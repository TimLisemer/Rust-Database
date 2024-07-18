use serde::{Deserialize, Serialize};
use crate::value::Value;

/// Represents a row in a database table.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Row {
    pub values: Vec<Value>,
}

impl Row {
    /// Creates a new `Row` instance.
    ///
    /// # Arguments
    ///
    /// * `values` - The values of the row.
    pub fn new(values: Vec<Value>) -> Self {
        Row {
            values,
        }
    }
}
