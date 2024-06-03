#![no_main]
sp1_zkvm::entrypoint!(main);
use sha2::{Digest, Sha256};
use tiny_keccak::{Hasher, Keccak};

pub fn main() {
    {
        let mut hasher = Sha256::new();
        let data = b"hello world";
        hasher.update(data);
        // Note that calling `finalize()` consumes hasher
        let hash = hasher.finalize();
        // Output should be b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
        sp1_zkvm::io::commit(&hash.as_slice());
    }
    {
        let mut sha3 = Keccak::v256();
        let mut output = [0u8; 32];
        sha3.update(b"hello");
        sha3.update(b" ");
        sha3.update(b"world");
        sha3.finalize(&mut output);
        // Output should be 644bcc7e564373040999aac89e7622f3ca71fba1d972fd94a31c3bfbf24e3938
        sp1_zkvm::io::commit(&output);
    }
}
