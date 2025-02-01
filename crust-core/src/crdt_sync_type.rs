use crate::text_operation::TextOperation;

pub trait CmRDT {
    type Op: Clone;
    type Value: Clone;

    fn apply(&mut self, op: &Self::Op);
    fn convert_operation(&self, op: TextOperation<Self::Value>) -> Vec<Self::Op>;
}
pub trait CvRDT {
    fn merge(&mut self, other: &Self);
}
pub trait Delta {
    type De: Clone;

    fn generate_delta(&self) -> Self::De;
    fn apply_delta(&mut self, delta: &Self::De);
}
