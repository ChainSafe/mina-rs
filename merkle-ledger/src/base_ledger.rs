use crate::sincable;

pub trait S : sincable::S {
    type Key;
    type TokenId;
    type TokenIdSet;
    type AccountId;
    type AccountIdSet;
    type Index;
  
    // fn to_list;
    // fn foldi_with_ignored_accounts;
    // fn iteri;
    // fn foldi;
    // fn fold_until;
    // fn accounts;
    // fn token_owner;
    // fn token_owners;
    // fn tokens;
    // fn next_available_token;
    // fn set_next_available_token;
    // fn location_of_account;
    // fn get_or_create_account;
    // fn get_or_create_account_exn;
    // fn close;
    // fn last_filled;
    // fn get_uuid;
    // fn get_directory;
    // fn get;
    // fn set;
    // fn set_batch;
    // fn get_at_index_exn;
    // fn set_at_index_exn;
    // fn index_of_account_exn;
    // fn merkle_root;
    // fn merkle_path;
    // fn merkle_path_at_index_exn;
    // fn remove_accounts_exn;
    // fn detached_signal;
}