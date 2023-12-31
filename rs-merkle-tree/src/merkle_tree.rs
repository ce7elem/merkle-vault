use crate::merkle_proof::MerkleProof;
use crate::utils::crypto::{hash, Hash};
use hex;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Represents the direction of a node in the Merkle tree.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Direction {
    Left,
    Right,
}

/// Represents a node in the Merkle tree.
#[derive(Serialize, Deserialize, Debug)]
pub struct MerkleNode {
    pub hash: Hash,
    pub direction: Direction,
}

/// Represents a Merkle tree.
pub struct MerkleTree {
    pub hashes: Vec<Hash>,
    levels_indices: Vec<usize>,
}

impl MerkleTree {
    /// Creates a new Merkle tree from a list of leaf hashes.
    pub fn from_leaves(leaves: Vec<Hash>) -> Self {
        if leaves.is_empty() {
            return Self {
                hashes: Vec::new(),
                levels_indices: vec![0],
            };
        }

        let mut tree = Self {
            hashes: Vec::from(leaves.clone()),
            levels_indices: vec![0],
        };

        fn generate_next_layer(hashes: Vec<Hash>, tree: &mut MerkleTree) -> Vec<Hash> {
            if hashes.len() == 1 {
                return hashes;
            }

            let last_even_index = if hashes.len() % 2 == 0 {
                hashes.len()
            } else {
                hashes.len() - 1
            };

            let mut layer = Vec::new();
            for i in (0..last_even_index - 1).step_by(2) {
                let left = hashes[i].clone();
                let right = hashes[i + 1].clone();

                layer.push(hash(&[left, right].concat()));
            }

            if last_even_index != hashes.len() {
                let last = hashes.last().unwrap();
                layer.push(hash(&[last.clone(), last.clone()].concat()));
            }

            tree.hashes.append(&mut layer.clone());
            tree.levels_indices
                .push(hashes.len() + tree.levels_indices.last().unwrap());
            return generate_next_layer(layer, tree);
        }

        generate_next_layer(leaves, &mut tree);
        tree.levels_indices.pop();
        return tree;
    }

    /// Returns the direction (Left or Right) of a node at the given index.
    fn get_node_direction(&self, index: usize) -> Direction {
        if index % 2 == 0 {
            Direction::Right
        } else {
            Direction::Left
        }
    }

    /// Returns the index of a hash in the list of hashes.
    fn get_hash_index(&self, hash: &Hash) -> Option<usize> {
        self.hashes.iter().position(|h| h == hash)
    }

    /// Generates a Merkle proof for a given hash.
    // TODO: Move it to MerkleProof
    pub fn proof(&self, hash: Hash) -> Result<MerkleProof, Box<dyn Error>> {
        if self.hashes.is_empty() {
            return Err("Tree is empty".into());
        }

        let mut hash_index = self.get_hash_index(&hash).unwrap();

        let mut proof_elements = vec![MerkleNode {
            hash: hash,
            direction: self.get_node_direction(hash_index),
        }];

        for level in self.levels_indices.iter() {
            let direction = self.get_node_direction(hash_index);
            let index = match direction {
                Direction::Right => hash_index + 1,
                Direction::Left => hash_index - 1,
            };
            let sibling_node = MerkleNode {
                hash: self.hashes[level + index].clone(),
                direction: direction,
            };
            proof_elements.push(sibling_node);
            hash_index = hash_index / 2;
        }
        Ok(MerkleProof::new(proof_elements))
    }

    /// Returns the root hash of the Merkle tree.
    pub fn root(&self) -> Option<&Hash> {
        return self.hashes.last();
    }

    /// Returns the hexadecimal representation of the root hash of the Merkle tree.
    pub fn root_hex(&self) -> Option<String> {
        match self.root() {
            Some(r) => Some(hex::encode(r)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_correct_tree_shape() {
        let leaf_values = ["a", "b", "c", "d", "e", "f"];
        let leaf_hashes: Vec<Hash> = leaf_values
            .iter()
            .map(|x| hash(&x.as_bytes().to_vec()))
            .collect();

        let tree = MerkleTree::from_leaves(leaf_hashes);

        assert_eq!(tree.hashes.len(), 12);
    }

    #[test]
    fn should_generate_proof() {
        let leaf_values = ["a", "b", "c", "d", "e", "f"];
        let leaf_hashes: Vec<Hash> = leaf_values
            .iter()
            .map(|x| hash(&x.as_bytes().to_vec()))
            .collect();

        let tree = MerkleTree::from_leaves(leaf_hashes.clone());
        let proof = tree.proof(leaf_hashes[0].clone()).unwrap();

        assert_eq!(proof.nodes.len(), 4);

        assert_eq!(tree.get_hash_index(&proof.nodes[0].hash).unwrap(), 0);
        assert_eq!(tree.get_hash_index(&proof.nodes[1].hash).unwrap(), 1);
        assert_eq!(tree.get_hash_index(&proof.nodes[2].hash).unwrap(), 7);
        assert_eq!(tree.get_hash_index(&proof.nodes[3].hash).unwrap(), 10);
    }
}
