use std::collections::HashMap;

struct DocumentRegistry {
    number_of_documents: u64,
    registry: HashMap<u64, u64>,
}

impl DocumentRegistry {
    fn new() -> Self {
        DocumentRegistry {
            number_of_documents: 0,
            registry: HashMap::new(),
        }
    }

    fn register_document(&mut self, id: u64) -> u64 {
        self.number_of_documents += 1;
        self.registry.insert(self.number_of_documents, id);

        self.number_of_documents
    }

    fn clear(&mut self) {
        self.registry.clear();
    }

    fn set_number_of_documents(&mut self, number_of_documents: u64) {
        self.number_of_documents = number_of_documents;
    }

    fn get_registry(&self) -> &HashMap<u64, u64> {
        &self.registry
    }
}
