use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value {
    pub value: String, // for now, only supporting strings
}

impl Value {
    pub fn new(value: String) -> Self {
        Value {
            value,
        }
    }
}