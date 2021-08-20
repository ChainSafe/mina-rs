pub trait Hash {
    type T: Eq + Into<&'static str>;
    type Account;
    fn to_string(other: &Self::T) -> String;
    fn merge(height: usize, other: &Self::T, another: &Self::T) -> Self::T;
    fn hash_account(account: &Self::Account) -> Self::T;
    fn empty_account() -> Self::T;
    // include Hashable.S_binable
}