pub trait CmRDT {
    fn apply(&mut self, other: &Self) -> Self;
}
pub trait CvRDT {
    fn merge(&mut self, other: &Self) -> Self;
}
pub trait Delta {
    fn generate_delta(&self, since: &Self) -> Self;
    fn apply_delta(&mut self, other: &Self) -> Self;
}
