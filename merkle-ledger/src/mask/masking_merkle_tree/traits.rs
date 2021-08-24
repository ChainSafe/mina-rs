use uuid::Uuid;

pub use crate::location::traits::{self, Addr};

trait Inner: S + Sized {}
pub trait S {
    type T;
    type Unattached;
    // type Attached : Attached<dyn Inner>;
    type Parent;
    type Account;
    type Location;
    type Hash;
    type Key;
    type TokenId;
    type TokenIdSet;
    type AccountId;
    type AccountIdSet;

    fn create(depth: usize) -> Self::T;
    fn get_uuid(&self) -> Uuid;
    // fn set_parent(other: Self::Unattached, parent: Self::Parent) -> Self::Attached;
}

pub trait Attached<K: S>: crate::mask::BaseMerkleTree {
    fn get_hash(&self, addr: Self::Addr) -> Option<Self::Hash>;
    fn commit(&self);
    fn unset_parent(trigger_signal: bool, loc: &str, this: &Self::T) -> K::Unattached;
    fn get_parent(this: &Self::T) -> K::Parent;
    fn parent_set_notify(this: &Self::T, account: Self::Account);
    fn copy(this: &Self::T) -> Self::T;
}
