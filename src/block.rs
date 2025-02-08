use std::fmt::{ self, Debug, Formatter };
use super::*;


pub struct Block {
    pub index: u32, // public unsigned 32-bit index that tells where this Block lies in Blockchain
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64, // nonce is a random number that is used only once in a blockchain to create a block.
    pub payload: String, // Later this will be changed to transactions
}

impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[index:{}]: {} [time stamp] at: {} with: {} ", // similar to printf method
            &self.index, 
            &hex::encode(&self.hash), // crate hex - similar to packages in Java
            &self.timestamp,
            &self.payload
        )
    }
}

impl Block {
    pub fn new (index: u32, timestamp: u128, prev_block_hash: BlockHash, nonce: u64,  payload: String) -> Self { // -> returns Block
        return Block { 
                index,
                timestamp,
                hash: vec![0; 32],
                prev_block_hash,
                nonce,
                payload,
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

        return bytes
    }
}


