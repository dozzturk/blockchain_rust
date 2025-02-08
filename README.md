hex crate: https://crates.io/crates/hex
crypto-hash crate: https://crates.io/crates/crypto-hash

#NOTES#

Hashing: SHA-256 is used - 32-byte hash

#Difficulty: (in this case) specifies the unsigned 128-bit integer that the most significant 16 bytes of the hash of a block must be less than before it is considered "valid"

- Difficulty could also be expressed as:
a. the first n bytes of the hash that must be zero
b. the number of bits or bytes at the beginning of the hash that must be zero
    
Check difficulty function implemented:
- Converting hash 16 bytes to an unsigned 128 bit integer and then doing > comparison.

#Nonce:
- unique to the data
- reproducible fingerprint for some data
- Therefore, to make a "valid" hash (per difficulty), we must change the bytes we send to the function (pre-image).
- Since we can't actually change the information stored in a block, we introduce an additional piece of data called a nonce: an arbitrary (but not necessarily random) value added as a field to each block, and hashed along with the data. 
- Since it has been declared arbitrary, we can change it as we please.
- Think of it like this: generating the correct hash for a block is like the puzzle, and the nonce is the key to that puzzle. The process of finding that key is called "mining".

#Mining Strategy: (Algorithm)
1. Generate new nonce
2. Hash bytes (this is the computationally heavy step)
3. Check hash against difficulty: 
    - a. Insufficient? -> Go back to step 1
    - b. Sufficient? -> Continue to step 4
4. Add block to chain
5. Submit to others (Networking is needed for this step but this is not implemented yet).


