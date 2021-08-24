pub trait S: crate::mask::BaseMerkleTree {
    type Unattached_mask;
    type Attached_mask;
    fn register_mask(this: &Self::T, other: &Self::Unattached_mask) -> Self::Attached_mask;
    // fn unregister_mask_exn(grandchildren: TODO, loc: &str, attached_mask: &Self::Attached_mask) -> Self::Unattached;
}
