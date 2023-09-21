use crate::merkle_proof::MerkleProof;
use crate::utils::crypto::hash;
use std::error::Error;
pub type Hash = Vec<u8>;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}
#[derive(Debug)]
pub struct MerkleNode {
    pub hash: Hash,
    pub direction: Direction,
}

pub struct MerkleTree {
    pub hashes: Vec<Hash>,
    levels_indices: Vec<usize>,
}

impl MerkleTree {
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
            tree.levels_indices.push(hashes.len() + tree.levels_indices.last().unwrap());
            return generate_next_layer(layer, tree);
        }

        generate_next_layer(leaves, &mut tree);
        tree.levels_indices.pop();
        return tree;
    }

    fn get_node_direction(&self, index: usize) -> Direction {
        if index % 2 == 0 {
            Direction::Right
        } else {
            Direction::Left
        }
    }

    fn get_hash_index(&self, hash: &Hash) -> Option<usize> {
        self.hashes.iter().position(|h| h == hash)
    }

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
        Ok(MerkleProof::new(
            proof_elements,
            self.root().unwrap().clone(),
        ))
    }

    pub fn root(&self) -> Option<&Hash> {
        return self.hashes.last();
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
