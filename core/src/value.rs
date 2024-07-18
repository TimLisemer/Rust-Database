use serde::{Deserialize, Serialize};

/// Represents a value in a database table.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value {
    pub value: String, // for now, only supporting strings
}

impl Value {
    /// Creates a new `Value` instance.
    ///
    /// # Arguments
    ///
    /// * `value` - The value as a string.
    pub fn new(value: String) -> Self {
        Value {
            value,
        }
    }
}
