use crate::data_containers::indexer::structures::documents::DocumentTrait;
use crate::data_containers::indexer::structures::term_info::{TermInfoTrait, TermInfoWithIDF};

pub trait PostIndexingAction<W: num::Num + Copy, D: DocumentTrait<W>, TI: TermInfoTrait<W, D>> {
    fn apply(&self, term_info: &mut TI, number_of_documents: u64);
}

pub struct CalculateIDFAction;

impl<W: num::Num + Copy, D: DocumentTrait<W>> PostIndexingAction<W, D, TermInfoWithIDF<W, D>>
    for CalculateIDFAction
{
    fn apply(&self, term_info: &mut TermInfoWithIDF<W, D>, number_of_documents: u64) {
        term_info.set_idf(f32::log10(
            number_of_documents as f32 / term_info.get_posting_list().len() as f32,
        ));
    }
}
