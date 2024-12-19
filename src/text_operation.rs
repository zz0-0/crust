use crate::crdt_type::{CmRDT, CvRDT, Delta};

#[derive(Debug, Clone)]
pub enum TextOperation {
    Insert { position: usize, text: String },
    Delete { position: usize },
}

pub trait TextOperationToCmRDT<K>
where
    K: CmRDT,
{
    type Op;
    fn convert_operation(&self, op: TextOperation) -> Vec<Self::Op>;
}

pub trait TextOperationToCvRDT<K>
where
    K: CvRDT,
{
    fn convert_operation(&self, op: TextOperation);
}

pub trait TextOperationToDelta<K>
where
    K: Delta,
{
    fn convert_operation(&self, op: TextOperation);
}
