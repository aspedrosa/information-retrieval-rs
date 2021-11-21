use crate::data_containers::indexer::structures::documents::{Document, DocumentFrequency};

pub trait TermInfo<W: num::Num, D: Document<W>> {
    fn new() -> Self
    where
        Self: Sized;
    fn new_with_posting_list(posting_list: Vec<D>) -> Self
    where
        Self: Sized;
    fn get_posting_list(&self) -> &Vec<D>;
    fn set_posting_list(&mut self, posting_list: Vec<D>);
    fn add_to_posting_list(&mut self, document: D);
}

pub trait TermInfoWithIDF<W: num::Num, D: Document<W>>: TermInfo<W, D> {
    fn get_idf(&self) -> f32;
    fn set_idf(&mut self, idf: f32);
}

// implementations vvv

pub struct FrequencyTermInfo {
    posting_list: Vec<DocumentFrequency>,
}

impl TermInfo<u64, DocumentFrequency> for FrequencyTermInfo {
    fn new() -> Self
    where
        Self: Sized,
    {
        FrequencyTermInfo {
            posting_list: Default::default(),
        }
    }

    fn new_with_posting_list(posting_list: Vec<DocumentFrequency>) -> Self
    where
        Self: Sized,
    {
        FrequencyTermInfo { posting_list }
    }

    fn get_posting_list(&self) -> &Vec<DocumentFrequency> {
        &self.posting_list
    }

    fn set_posting_list(&mut self, posting_list: Vec<DocumentFrequency>) {
        self.posting_list = posting_list;
    }

    fn add_to_posting_list(&mut self, document: DocumentFrequency) {
        self.posting_list.push(document);
    }
}
