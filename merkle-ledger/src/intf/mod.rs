pub mod key;
pub mod token_id;
pub mod account_id;
pub mod balance;
pub mod account;
pub mod hash;
pub mod key_value_database;

pub mod depth {
    pub trait Depth {
        fn depth(&self) -> usize;
    }
}

pub trait StorageLocations {
    fn key_value_db_dir() -> String;
}
