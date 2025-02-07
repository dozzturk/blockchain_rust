use std::fmt::{ self, Debug, Formatter };
use super::*;


pub struct Block {
    pub index: u32, // public unsigned 32-bit index that tells where this Block lies in Blockchain
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String, // Later this will be changed to transactions
}

impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {} ", // similar to printf method
            &self.index, 
            &hex::encode(&self.hash), // crate hex - similar to packages in Java
            &self.timestamp,
            &self.payload
        )
    }
}

impl Block {
    pub fn new (index: u32, timestamp: u128, prev_block_hash: BlockHash, nonce: u64,  payload: String) -> Self { // -> returns Block
        Block { // Return Block
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce,
            payload,
        }
    }
}


