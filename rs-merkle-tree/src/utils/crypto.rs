use sha2::{Digest, Sha256};
pub type Hash = Vec<u8>;

pub fn hash(value: &Vec<u8>) -> Hash {
    return Sha256::digest(value).to_vec();
}
