use crate::entry::Entry;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Table {
    pub name: String,
    pub entries: Vec<Entry>,
}

impl Table {
    pub fn new(name: String) -> Self {
        Table {
            name,
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn get_entry_from_key(&self, key: &str) -> Option<&Entry> {
        self.entries.iter().find(|entry| entry.key == key)
    }

    pub fn get_value_from_key(&self, key: &str) -> Option<&String> {
        self.entries.iter().find(|entry| entry.key == key).map(|entry| &entry.value)
    }
}
