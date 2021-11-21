use std::borrow::Borrow;
use std::fmt::{Display, Formatter};

pub struct Document {
    id: u64,
    to_tokenize: Vec<String>,
}

impl Document {
    pub fn new() -> Self {
        Document {
            id: 0,
            to_tokenize: Vec::new(),
        }
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn add_string_to_tokenize(&mut self, to_tokenize: String) {
        self.to_tokenize.push(to_tokenize);
    }

    pub fn get_to_tokenize(&self) -> &[String] {
        self.to_tokenize.borrow()
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.to_tokenize.join(""))
    }
}
