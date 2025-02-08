hex crate: https://crates.io/crates/hex
crypto-hash crate: https://crates.io/crates/crypto-hash


Hashing: SHA-256 is used - 32-byte hash


Difficulty: (in this case) specifies the unsigned 128-bit integer that the most significant 16 bytes of the hash of a block must be less than before it is considered "valid"

- Difficulty could also be expressed as:
a. the first n bytes of the hash that must be zero
b. the number of bits or bytes at the beginning of the hash that must be zero
    
Check difficulty function implemented:
- Converting hash 16 bytes to an unsigned 128 bit integer and then doing > comparison.