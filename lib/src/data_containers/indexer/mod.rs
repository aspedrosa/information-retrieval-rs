use crate::data_containers::indexer::post_indexing_actions::PostIndexingAction;
use crate::data_containers::indexer::structures::documents::{Document, DocumentFrequency};
use crate::data_containers::indexer::structures::term_info::{FrequencyTermInfo, TermInfo};
use std::collections::HashMap;

pub mod post_indexing_actions;
pub mod structures;
pub mod weight_calculations;

trait Indexer<T: Ord, W: num::Num, D: Document<W>, TI: TermInfo<W, D>> {
    fn new() -> Self;
    fn new_with_post_actions(
        post_indexing_actions: Vec<Box<dyn PostIndexingAction<W, D, TI>>>,
    ) -> Self;
    fn get_inverted_index(&self) -> &HashMap<T, TI>;
    fn index_terms(&mut self, doc_id: u64, terms: &[String]) {
        let mut frequencies = HashMap::<String, u64>::new();

        for term in terms {
            match frequencies.get_mut(term) {
                Some(current_count) => {
                    *current_count += 1; // TODO check if this really increments the exiting variable
                }
                None => {
                    frequencies.insert(String::from(term), 1);
                }
            }
        }

        self.insert_document(doc_id, frequencies);
    }
    fn insert_document(&mut self, doc_id: u64, frequencies: HashMap<String, u64>);
    fn clear(&mut self);
}

struct FrequencyIndexer {
    inverted_index: HashMap<String, FrequencyTermInfo>,
    post_indexing_action:
        Vec<Box<dyn PostIndexingAction<u64, DocumentFrequency, FrequencyTermInfo>>>,
}

impl Indexer<String, u64, DocumentFrequency, FrequencyTermInfo> for FrequencyIndexer {
    fn new() -> Self {
        FrequencyIndexer {
            inverted_index: Default::default(),
            post_indexing_action: Default::default(),
        }
    }

    fn new_with_post_actions(
        post_indexing_actions: Vec<
            Box<dyn PostIndexingAction<u64, DocumentFrequency, FrequencyTermInfo>>,
        >,
    ) -> Self {
        FrequencyIndexer {
            inverted_index: Default::default(),
            post_indexing_action: post_indexing_actions,
        }
    }

    fn get_inverted_index(&self) -> &HashMap<String, FrequencyTermInfo> {
        &self.inverted_index
    }

    fn insert_document(&mut self, doc_id: u64, frequencies: HashMap<String, u64>) {
        for entry in frequencies {
            let term = entry.0;

            match self.inverted_index.get_mut(&term) {
                Some(term_info) => {
                    term_info.add_to_posting_list(DocumentFrequency::new(doc_id, entry.1))
                }
                None => {
                    self.inverted_index.insert(
                        term,
                        FrequencyTermInfo::new_with_posting_list(vec![DocumentFrequency::new(
                            doc_id, entry.1,
                        )]),
                    );
                }
            }
        }
    }

    fn clear(&mut self) {
        self.inverted_index.clear();
    }
}
