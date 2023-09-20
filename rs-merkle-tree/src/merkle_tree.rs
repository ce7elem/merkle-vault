use utils::hash;
pub type Hash = Vec<u8>;

struct MerkleTree {
    hashes: Vec<Hash>,
}

impl MerkleTree {
    pub fn from_leaves(leaves: Vec<Hash>) -> Self {
        if leaves.len() == 0 {
            return Self { hashes: Vec::new() };
        }

        let mut tree = Self { hashes: Vec::new() };
        tree.hashes.append(&mut leaves.clone());

        fn generate_next_layer(hashes: Vec<Hash>, tree: &mut Vec<Hash>) -> Vec<Hash> {
            println!("CALL {}", hashes.len());
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

                layer.push(hash([left, right].concat()));
            }

            if last_even_index != hashes.len() {
                let last = hashes.last().unwrap();
                layer.push(hash([last.clone(), last.clone()].concat()));
            }

            tree.append(&mut layer.clone());
            return generate_next_layer(layer, tree);
        }

        generate_next_layer(leaves, &mut tree.hashes);
        return tree;
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
            .map(|x| hash(x.as_bytes().to_vec()))
            .collect();

        let tree = MerkleTree::from_leaves(leaf_hashes);

        assert_eq!(tree.hashes.len(), 12);
    }
}
