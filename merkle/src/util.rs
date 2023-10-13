use sha2::{Sha256, Digest};

use base64::engine::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;


pub type Hash32Bytes = [u8; 32];
#[derive(Serialize, Deserialize, Debug)]
pub struct MerkleProof {
    pub leaf_position: usize,
    pub leaf_value: String,
    pub proof_hash_values_base64: Vec<String>,
    pub proof_hash_values: Option<Vec<Hash32Bytes>>
}

pub fn hash_leaf(leaf_value: String) -> Hash32Bytes {
    let mut hasher = Sha256::new();
    hasher.update("leaf:");
    hasher.update(leaf_value);
    return hasher.finalize().to_vec().try_into().expect("can't convert to static array");
}

pub fn hash_internal(left: Hash32Bytes, right: Hash32Bytes) -> Hash32Bytes {
    let mut hasher = Sha256::new();
    hasher.update("node:");
    hasher.update(left);
    hasher.update(right);
    return hasher.finalize().to_vec().try_into().expect("can't convert to static array");
}

pub fn decode_hash(hash_base64: &str) -> Hash32Bytes {
    let hash_vec = BASE64.decode(hash_base64).expect("invalid hash");
    assert_eq!(hash_vec.len(), 32);
    let hash: Hash32Bytes = hash_vec.try_into().expect("can't convert to static array");
    hash
}
pub fn encode_hash(hash: Hash32Bytes) -> String {
    BASE64.encode(hash)
}


pub fn read_merkle_proof(proof_file: &String) -> Box<MerkleProof> {
    let contents = fs::read_to_string(proof_file)
        .expect("Something went wrong reading the file");

    // Deserialize the YAML content into a Config struct
    let mut merkle_proof: Box<MerkleProof> = serde_yaml::from_str(&contents)
        .expect("Error parsing the YAML file");

    assert!(merkle_proof.proof_hash_values.is_none());

    let mut decoded_hash = Vec::new();
    for hash_base64 in &merkle_proof.proof_hash_values_base64 {
        decoded_hash.push(decode_hash(hash_base64));
    }
    merkle_proof.proof_hash_values = Some(decoded_hash);
    merkle_proof
}

pub fn write_merkle_proof(proof: &MerkleProof, proof_file: &str) {
    let ser = serde_yaml::to_string(proof).expect("couldn't serialize");

    let mut file = File::create(proof_file).expect("Unable to create file");
    file.write_all(ser.as_bytes()).expect("Unable to write to file");
    file.flush().expect("Unable to flush buffer");

}