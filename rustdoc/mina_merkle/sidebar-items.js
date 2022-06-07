initSidebarItems({"enum":[["MerkleProofError","Type that represents errors in calculating hashes for a merkle proof"]],"macro":[["impl_poseidon_legacy_hasher_pool_provider","Macro that auto-implements PoseidonLegacyHasherPoolProvider"]],"mod":[["macros","Re-exports external types that macro implementations depend on, so that the crate that uses the macros do not need to depend on these external types"],["prefixes","This module defines functions that generate domain prefix from merkle tree node height"]],"struct":[["DefaultMerkleProof","Merkle proof implementation of a single leaf node, for details refer to https://www.webopedia.com/definitions/merkle-proof/"],["FixedHeightMode","Type state mode for a tree with a fixed height"],["MerkleTreeNodeMetadata","Metadata of a give tree node, including index and depth in the merkle tree it belongs to, which can be used for calculating hash"],["MinaMerkleTree","Special complete binary merkle tree that is compatible with https://github.com/o1-labs/snarky/blob/master/src/base/merkle_tree.ml whose leaf nodes are at the same height"],["MinaPoseidonMerkleHasher","Hasher for mina binary merkle tree that uses poseidon hash"],["MinaPoseidonMerkleMerger","Merger for mina binary merkle tree that uses poseidon hash with mina specific domain string calculated from node height"],["VariableHeightMode","Type state mode for a tree with a variable height that increases as data is added"]],"trait":[["HeightMode","Type state mode for a tree"],["MaskableMerkleTree","A merkle tree that can be masked by [super::MaskingMerkleTree]"],["MaskingMerkleTree","A merkle tree that can be used to mask [super::MaskableMerkleTree]"],["MerkleHasher","Trait for implementing binary merkle tree hasher"],["MerkleMerger","Trait that merges the hashes of child nodes and calculates the hash of their parent"],["MerkleProof","Merkle proof trait of a single leaf node, for details refer to https://www.webopedia.com/definitions/merkle-proof/"],["MerkleTree","Trait for implementing binary merkle tree"],["PoseidonLegacyHasherPoolProvider","Trait that provides poseidon hasher pool as it’s expensive to create a new hasher"],["SparseMerkleTree","Trait for implementing sparse binary merkle tree. It is essentially a collection of [MerkleProof]"]]});