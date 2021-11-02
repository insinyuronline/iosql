use std::fs;

pub struct Pager {
    pub page: String
}

impl Pager {
    pub fn new() -> Pager {
        Pager {
            page: String::new(),
        }
    }

    pub fn load(&mut self, filepath: &str) {
        self.page = fs::read_to_string(filepath).unwrap();
    }

    pub fn save(&self, filepath: &str) {
        fs::write(filepath, &self.page).unwrap();
    }
}