struct Table<'a> {
    name: String,
    entries: Vec<Entry<'a>>
}

impl Table {
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
    foreign_key: Option<&'a Entry<'a>>
}

impl Entry {
    pub fn new(key: String, value: Vec<String>, primary_key: bool, foreign_key: Option<& Entry>) -> Self {
        Self {
            key, value, primary_key, foreign_key
        }
    }
}

fn main() {
    println!("This is a database management engine. Maybe it is an HTTP server? Or a Linux systemd service? Who knows?");
}
