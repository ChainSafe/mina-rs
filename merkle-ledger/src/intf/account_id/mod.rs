pub mod stable;

pub trait AccountId {
    type Key;
    type TokenId;

    type T : stable::v1::K;
    fn public_key(other: &Self::T) -> &Self::Key;
    fn token_id(other: &Self::T) -> &Self::TokenId;
    fn create(key: &Self::Key, token_id: &Self::TokenId) -> Self::T;
}