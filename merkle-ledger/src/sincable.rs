// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub trait S {
    type RootHash;
    type Hash;
    type Account;
    type Addr;
    type T;
    type Path;

    fn depth(this: &Self::T) -> usize;
    // fn num_accounts
    // fn merkle_path_at_addr_exn
    // fn get_inner_hash_at_addr_exn
    // fn set_inner_hash_at_addr_exn
    // fn set_all_accounts_rooted_at_exn
    // fn set_batch_accounts
    // fn get_all_accounts_rooted_at_exn
    // fn merkle_root
    // fn make_space_for
}
