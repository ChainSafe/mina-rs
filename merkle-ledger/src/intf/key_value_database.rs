use uuid::Uuid;

pub trait KeyValueDatabase {
    type T;
    type Config;

    // include
    // Key_value_database.Intf.Ident    

    fn create_checkpoint(other: &Self::T, s: &str) -> Self::T;
    fn get_uuid(other: &Self::T) -> Uuid;
    // fn set_batch(other: &Self::T, remove_keys: &[BigString], key_data_pairs: &[(BigString, BigString)]);
    // fn to_alist(other: &Self::T) -> Vec<(BigString, BigString)>;
}