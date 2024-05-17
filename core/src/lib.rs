// To make sure that other projects can access your code, everything must be publically exported from THIS file:
// - Either you have `pub` methods here (like `add`), or you have public module declarations (`pub mod $WHATEVER`)

/// The world's most useless database! Think about what your database should be able to do and design an API
/// accordingly!
pub struct DatabaseDummy {
    data: Vec<String>,
}

impl DatabaseDummy {
    pub fn store(&mut self, data: String) {
        self.data.push(data);
    }

    pub fn get_data(&self) -> Vec<String> {
        // Returns all data starting with 'a'. What a useful query!
        self.data
            .iter()
            .filter(|entry| entry.starts_with('a'))
            .cloned()
            .collect()
    }
}
