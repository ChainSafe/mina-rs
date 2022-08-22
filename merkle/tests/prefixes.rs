// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use anyhow::*;
    use mina_merkle::prefixes::*;

    #[test]
    fn make_merkle_tree_hash_3() -> Result<()> {
        let prefix_at_3 = make_prefix_merkle_tree(3);
        ensure!(prefix_at_3 == "CodaMklTree003");
        Ok(())
    }

    #[test]
    fn make_merkle_tree_hash_13() -> Result<()> {
        let prefix_at_3 = make_prefix_merkle_tree(13);
        ensure!(prefix_at_3 == "CodaMklTree013");
        Ok(())
    }

    #[test]
    fn make_merkle_tree_hash_113() -> Result<()> {
        let prefix_at_3 = make_prefix_merkle_tree(113);
        ensure!(prefix_at_3 == "CodaMklTree113");
        Ok(())
    }

    #[test]
    fn make_coinbase_merkle_tree_hash() -> Result<()> {
        let prefix_at_3 = make_prefix_coinbase_merkle_tree(3);
        ensure!(prefix_at_3 == "CodaCbMklTree003");
        Ok(())
    }
}
