pub trait Account {
    type T: Eq;
    type TokenId;
    type AccountId;
    type Balance;
    fn token(other: &Self::T) -> Self::TokenId;
    fn identifier(other: &Self::T) -> Self::AccountId;
    fn balance(other: &Self::T) -> Self::Balance;
    fn token_owner(other: &Self::T) -> bool;
    fn empty() -> Self::T;
}