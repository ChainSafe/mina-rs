// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub mod address;
pub mod base_input;
pub mod base_ledger;
pub mod database;
pub mod location;
pub mod mask;
pub mod path;
pub mod sincable;
pub mod traits;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
