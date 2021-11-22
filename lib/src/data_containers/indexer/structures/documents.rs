pub trait DocumentTrait<T: num::Num + Copy> {
    fn new(doc_id: u64, weight: T) -> Self;
    fn get_doc_id(&self) -> u64;
    fn get_weight(&self) -> T;
}

pub trait DocumentWithInfoTrait<T: num::Num + Copy, I: Default>: DocumentTrait<T> {
    fn get_extra_info(&self) -> &I;
}

pub struct Document<T: num::Num + Copy> {
    doc_id: u64,
    weight: T,
}

impl<T: num::Num + Copy> DocumentTrait<T> for Document<T> {
    fn new(doc_id: u64, weight: T) -> Document<T> {
        Document {
            doc_id,
            weight,
        }
    }

    fn get_doc_id(&self) -> u64 {
        self.doc_id
    }

    fn get_weight(&self) -> T {
        self.weight
    }
}

pub struct DocumentWithInfo<T: num::Num + Copy, I> {
    document: Document<T>,
    extra_info: I,
}

impl<T: num::Num + Copy, I: Default> DocumentTrait<T> for DocumentWithInfo<T, I> {
    fn new(doc_id: u64, weight: T) -> DocumentWithInfo<T, I> {
        DocumentWithInfo {
            document: Document {
                doc_id,
                weight,
            },
            extra_info: Default::default(),
        }
    }

    fn get_doc_id(&self) -> u64 {
        self.document.get_doc_id()
    }

    fn get_weight(&self) -> T {
        self.document.get_weight()
    }
}

impl<T: num::Num + Copy, I: Default> DocumentWithInfoTrait<T, I> for DocumentWithInfo<T, I>  {
    fn get_extra_info(&self) -> &I {
        &self.extra_info
    }
}

