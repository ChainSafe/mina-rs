pub mod stable;

pub trait TokenId {
    type T : stable::Latest::K;
    fn default() -> Self::T;
    fn next(other: &Self::T) -> Self::T;
}