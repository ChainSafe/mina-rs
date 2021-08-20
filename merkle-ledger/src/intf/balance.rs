pub trait Balancer {
    type T: Eq + Into<usize>;
    fn zero() -> Self::T;
    fn to_int(&self) -> usize;
}