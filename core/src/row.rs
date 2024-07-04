use serde::{Deserialize, Serialize};

use crate::value::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Row {
    pub values: Vec<Value>,
}

impl Row{
    pub fn new(values: Vec<Value>, ) -> Self {
        Row {
            values,
        }
    }
}