use std::fmt::{ self, Debug, Formatter };
use super::*;


pub struct Block {
    pub index: u32, // public unsigned 32-bit index that tells where this Block lies in Blockchain
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64, // nonce is a random number that is used only once in a blockchain to create a block.
    pub payload: String, // Later this will be changed to transactions
    pub difficulty: u128
}

impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[index:{}] [hash: {}] [time stamp] at: {} with: {} [nonce: {}] ", // similar to printf method
            &self.index, 
            &hex::encode(&self.hash), // crate hex - similar to packages in Java
            &self.timestamp,
            &self.payload,
            &self.nonce,
        )
    }
}

impl Block {
    pub fn new (index: u32, timestamp: u128, prev_block_hash: BlockHash, nonce: u64,  payload: String, difficulty: u128) -> Self { // -> returns Block
        return Block { 
                index,
                timestamp,
                hash: vec![0; 32],
                prev_block_hash,
                nonce,
                payload,
                difficulty,
        }
    }

    // Mining function
    pub fn mine (&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            // loop though until find one
            self.nonce = nonce_attempt;
            let hash = self.hash();

            //check hash to see if it matches the difficulty
            if check_difficulty(&hash, self.difficulty) {
                // if this returns true, then hash matches the difficulty
                self.hash = hash;
                return;
            }
        }
    }
}


// Hash function - takes &self as a parameter and returns vector of bytes (Vec<u8>)
impl Hashable for Block {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![]; // mutable
        bytes.extend(&u32_bytes(&self.index)); // uses pre-defined functions
        bytes.extend(&u128_bytes(&self.timestamp)); // uses pre-defined functions
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce)); // uses pre-defined functions
        bytes.extend(self.payload.as_bytes()); //uses .as_bytes() because payload is String
        bytes.extend(&u128_bytes(&self.difficulty)); // uses pre-defined functions

        return bytes
    }
}

// hash: reference to BlockHash (hash: &BlockHash)
pub fn check_difficulty(hash: &BlockHash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}


