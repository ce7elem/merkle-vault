use crate::merkle_tree::{Direction, MerkleNode};
use crate::utils::crypto::{hash, Hash};
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Represents a Merkle proof, which is a list of Merkle nodes.
#[derive(Serialize, Deserialize, Debug)]
pub struct MerkleProof {
    pub nodes: Vec<MerkleNode>,
}

impl MerkleProof {
    pub fn new(hashes: Vec<MerkleNode>) -> Self {
        Self { nodes: hashes }
    }

    /// Computes the Merkle root hash using the Merkle proof.
    ///
    /// # Errors
    ///
    /// Returns an error if the Merkle proof is empty or if there's an issue computing the root hash.
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

    /// Computes the hexadecimal representation of the Merkle root hash using the Merkle proof.
    ///
    /// # Errors
    ///
    /// Returns an error if there's an issue computing the root hash or encoding it as hexadecimal.
    pub fn compute_root_hex(&self) -> Result<String, Box<dyn Error>> {
        Ok(hex::encode(self.compute_root()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::merkle_tree::MerkleTree;

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
    }
}
