use crate::crdt_type::{CmRDT, CvRDT, Delta};

pub enum TextOperation {
    Insert { position: usize, text: String },
    Delete { position: usize },
}

pub trait TextOperationToCmRDT: CmRDT {
    type Op;
    fn convert_operation(&self, op: TextOperation) -> Vec<<Self as CmRDT>::Op>;
}

pub trait TextOperationToCvRDT: CvRDT {
    fn convert_operation(&self, op: TextOperation);
}

pub trait TextOperationToDelta: Delta {
    fn convert_operation(&self, op: TextOperation);
}
