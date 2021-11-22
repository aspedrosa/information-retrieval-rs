use std::marker::PhantomData;
use crate::data_containers::indexer::structures::documents::DocumentTrait;

pub trait TermInfoTrait<W: num::Num + Copy, D: DocumentTrait<W>> {
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

pub struct TermInfo<W: num::Num + Copy, D: DocumentTrait<W>> {
    phantom: PhantomData<W>,
    posting_lists: Vec<D>
}

impl<W: num::Num + Copy, D: DocumentTrait<W>> TermInfoTrait<W, D> for TermInfo<W, D> {
    fn new() -> Self where Self: Sized {
        TermInfo {
            phantom: PhantomData::default(),
            posting_lists: Default::default()
        }
    }

    fn new_with_posting_list(posting_list: Vec<D>) -> Self where Self: Sized {
        TermInfo {
            posting_lists: posting_list,
            phantom: PhantomData::default(),
        }
    }

    fn get_posting_list(&self) -> &Vec<D> {
        &self.posting_lists
    }

    fn set_posting_list(&mut self, posting_list: Vec<D>) {
        self.posting_lists = posting_list;
    }

    fn add_to_posting_list(&mut self, document: D) {
        self.posting_lists.push(document);
    }
}

pub struct TermInfoWithIDF<W: num::Num + Copy, D: DocumentTrait<W>> {
    phantom: PhantomData<W>,
    posting_lists: Vec<D>,
    idf: f32,
}

impl<W: num::Num + Copy, D: DocumentTrait<W>> TermInfoWithIDF<W, D>  {
    pub fn get_idf(&self) -> f32 {
        self.idf
    }

    pub fn set_idf(&mut self, idf: f32) {
        self.idf = idf;
    }
}

impl<W: num::Num + Copy, D: DocumentTrait<W>> TermInfoTrait<W, D> for TermInfoWithIDF<W, D>  {
    fn new() -> Self where Self: Sized {
        TermInfoWithIDF {
            phantom: PhantomData::default(),
            posting_lists: Default::default(),
            idf: 0f32,
        }
    }

    fn new_with_posting_list(posting_list: Vec<D>) -> Self where Self: Sized {
        TermInfoWithIDF {
            phantom: PhantomData::default(),
            posting_lists: posting_list,
            idf: 0f32,
        }
    }

    fn get_posting_list(&self) -> &Vec<D> {
        &self.posting_lists
    }

    fn set_posting_list(&mut self, posting_list: Vec<D>) {
        self.posting_lists = posting_list;
    }

    fn add_to_posting_list(&mut self, document: D) {
        self.posting_lists.push(document);
    }
}
