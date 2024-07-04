use std::thread::sleep;
use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Table {
    name: String,
    entries: Vec<Entry>,
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

    pub fn get_value_from_key(&self, key: &str) -> Option<&Vec<String>> {
        self.entries.iter().find(|entry| entry.key == key).map(|entry| &entry.value)
    }
}

#[derive(Serialize, Deserialize)]
struct Entry {
    key: String,
    value: Vec<String>,
    primary_key: bool,
    non_null: bool,
    unique: bool,
    foreign_key: Option<Vec<Box<Entry>>>,
}

impl Entry {
    pub fn new(
        key: String,
        value: Vec<String>,
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

fn main() {
    // Create a new table
    let mut table = Table::new("Users".to_string());

    // Create entries
    let entry1 = Entry::new(
        "id".to_string(),
        vec!["1".to_string()],
        true,
        true,
        true,
        None,
    );

    let entry2 = Entry::new(
        "name".to_string(),
        vec!["Alice".to_string()],
        false,
        true,
        false,
        None,
    );

    let entry3 = Entry::new(
        "email".to_string(),
        vec!["alice@example.com".to_string()],
        false,
        true,
        true,
        None,
    );

    // Add entries to the table
    table.add_entry(entry1);
    table.add_entry(entry2);
    table.add_entry(entry3);

    // Convert table to JSON string
    let table_json = serde_json::to_string(&table).unwrap();

    // Pause for a while
    loop {
        println!("This is a database management engine. Maybe it is an HTTP server? Or a Linux systemd service? Who knows?");
        println!("Test Table: {}", table_json);
        sleep(Duration::from_secs(5));
    }
}
