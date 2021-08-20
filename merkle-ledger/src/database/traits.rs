pub trait S : crate::base_ledger::S {
    fn create(directory_name: &str, depth: usize) -> Self::T;
    fn create_checkpoint(other: &Self::T, directory_name: &str) -> Self::T;
    fn with_ledger<F : FnOnce(Self::T)>(depth: usize) -> F;
}