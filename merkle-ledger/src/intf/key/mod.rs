pub mod stable;

pub trait Key {
    type T : stable::v1::K;
    fn empty() -> Self::T;
    fn to_string(other: &Self::T) -> String;
}