use crate::merkle_tree::{Direction, MerkleNode};
use crate::utils::crypto::{hash, Hash};
use std::error::Error;

#[derive(Debug)]
pub struct MerkleProof {
    pub nodes: Vec<MerkleNode>,
    tree_root: Hash,
}

impl MerkleProof {
    pub fn new(hashes: Vec<MerkleNode>, expected_root: Hash) -> Self {
        Self {
            nodes: hashes,
            tree_root: expected_root,
        }
    }

    pub fn compute_root(&self) -> Result<Hash, Box<dyn Error>> {
        if self.nodes.is_empty() {
            return Err("missing proof".into());
        }

        let mut merkle_root_from_proof = self.nodes[0].hash.clone();

        for i in 1..self.nodes.len() {
            if self.nodes[i].direction == Direction::Right {
                merkle_root_from_proof =
                    hash(&[merkle_root_from_proof, self.nodes[i].hash.clone()].concat());
            } else {
                merkle_root_from_proof =
                    hash(&[self.nodes[i].hash.clone(), merkle_root_from_proof].concat());
            }
        }
        return Ok(merkle_root_from_proof);
    }

    pub fn verify(&self) -> bool {
        self.compute_root().unwrap() == self.tree_root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::merkle_tree::MerkleTree;
    use hex;

    #[test]
    fn should_generate_proof() {
        let leaf_values = ["a", "b", "c", "d", "e", "f"];
        let leaf_hashes: Vec<Hash> = leaf_values
            .iter()
            .map(|x| hash(&x.as_bytes().to_vec()))
            .collect();

        let tree = MerkleTree::from_leaves(leaf_hashes.clone());
        let proof = tree.proof(leaf_hashes[0].clone()).unwrap();

        assert_eq!(proof.compute_root().unwrap(), *tree.root().unwrap());
        assert!(proof.verify());
    }
}
