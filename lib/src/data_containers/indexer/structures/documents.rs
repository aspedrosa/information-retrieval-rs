pub trait Document<T: num::Num> {
    fn new(doc_id: u64, weight: T) -> Self;
    fn get_doc_id(&self) -> u64;
    fn get_weight(&self) -> T;
}

pub trait DocumentWithInfo<T: num::Num, I>: Document<T> {
    fn get_extra_info(&self) -> &I;
}

// implementations vv

pub struct DocumentFrequency {
    doc_id: u64,
    frequency: u64,
}

impl Document<u64> for DocumentFrequency {
    fn new(doc_id: u64, weight: u64) -> Self {
        DocumentFrequency {
            doc_id,
            frequency: weight,
        }
    }

    fn get_doc_id(&self) -> u64 {
        self.doc_id
    }

    fn get_weight(&self) -> u64 {
        self.frequency
    }
}

struct DocumentWithPositions {
    doc_id: u64,
    weight: f32,
    extra_info: Vec<u32>,
}

impl Document<f32> for DocumentWithPositions {
    fn new(doc_id: u64, weight: f32) -> Self {
        DocumentWithPositions {
            doc_id,
            weight,
            extra_info: Default::default(),
        }
    }

    fn get_doc_id(&self) -> u64 {
        self.doc_id
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }
}

impl DocumentWithInfo<f32, Vec<u32>> for DocumentWithPositions {
    fn get_extra_info(&self) -> &Vec<u32> {
        &self.extra_info
    }
}
