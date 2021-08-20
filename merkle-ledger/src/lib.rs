pub mod address;
pub mod path;
pub mod location;
pub mod sincable;
pub mod base_ledger;
pub mod intf;
pub mod base_input;
pub mod database;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
