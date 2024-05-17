use std::thread::sleep;
use std::time::Duration;

struct Table<'a> {
    name: String,
    entries: Vec<Entry<'a>>
}

impl<'a> Table<'a> {
    pub fn new() {

    }

    pub fn get_entry_from_key(&self, key: String) -> Option<& Entry> {
        self.entries.
            iter().
            find(|entry| entry.key == key)
    }

    pub fn get_value_from_key(&self, key: String) -> Option<& Vec<String>> {
        self.entries.
            iter().
            find(|entry| entry.key == key).map(|entry| &entry.value)
    }
}

struct Entry<'a> {
    key: String,
    value: Vec<String>,
    primary_key: bool,
    non_null: bool,
    unique: bool,
    foreign_key: Option<Vec<&'a Entry<'a>>>
}

impl<'a> Entry<'a> {
    pub fn new(key: String, value: Vec<String>, primary_key: bool, non_null: bool, unique: bool, foreign_key: Option<Vec<&'a Entry<'a>>>) -> Self {

        if primary_key {
            if non_null == false || value.is_empty(){
                panic!("Primary Key can not be null!")
            }

            if unique == false {
                // Todo: Add actual check unique logic
                panic!("Primary Key has to be unique")
            }
        }

        Self {
            key, value, primary_key, non_null, unique, foreign_key
        }
    }
}

fn main() {
    loop {
        println!("This is a database management engine. Maybe it is an HTTP server? Or a Linux systemd service? Who knows?");

        sleep(Duration::from_secs(5))
    }

}
