use crate::data_containers::indexer::structures::documents::Document;
use crate::data_containers::indexer::structures::term_info::{TermInfo, TermInfoWithIDF};

pub trait PostIndexingAction<W: num::Num, D: Document<W>, TI: TermInfo<W, D>> {
    fn apply(&self, term_info: &mut TI, number_of_documents: u64);
}

pub struct CalculateIDFAction;

impl<W: num::Num, D: Document<W>, TI: TermInfoWithIDF<W, D>> PostIndexingAction<W, D, TI>
    for CalculateIDFAction
{
    fn apply(&self, term_info: &mut TI, number_of_documents: u64) {
        term_info.set_idf(f32::log10(
            number_of_documents as f32 / term_info.get_posting_list().len() as f32,
        ));
    }
}
