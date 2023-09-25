# rs-merkle-tree Librairy

This is a rust librairy for Merkle trees, featuring building a Merkle tree,
creation and verification of Merkle proofs for a single node.

## Usage

```rs
let leaf_values = ["a", "b", "c", "d", "e", "f"];
let leaf_hashes: Vec<Hash> = leaf_values
    .iter()
    .map(|x| hash(&x.as_bytes().to_vec()))
    .collect();

let tree = MerkleTree::from_leaves(leaf_hashes);
let proof = tree.proof(leaf_hashes[0].clone()).unwrap();

assert_eq!(proof.compute_root().unwrap(), *tree.root().unwrap());
```
