// similar to interface
pub trait Hashable {  
    // self similar to "this"
    // -> Vec<u8> : this will give to use vector of bites
    fn bytes (&self) ->  Vec<u8>;

    // Crate will be used
    fn hash (&self) -> Vec<u8> {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes()) // &self.bytes() means hash this
    }

}