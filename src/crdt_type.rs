pub trait CmRDT {
    type Op;
    fn apply(&mut self, op: Self::Op);
}
pub trait CvRDT {
    fn merge(&mut self, other: &Self);
}
pub trait Delta {
    fn generate_delta(&self, since: &Self) -> Self;
    fn apply_delta(&mut self, other: &Self);
}
