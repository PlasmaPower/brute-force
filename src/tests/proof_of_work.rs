use crate::{adaptors, brute_force, Config};
use blake2::{Blake2b, Digest};

#[test]
fn proof_of_work() {
    let config = Config::default();
    let f = |nonce: &u64| {
        let digest = Blake2b::digest(&nonce.to_le_bytes());
        digest.as_slice()[..3] == [0; 3]
    };
    let nonce = brute_force(config, adaptors::output_input(adaptors::auto_advance(f)));
    let digest = Blake2b::digest(&nonce.to_le_bytes());
    assert!(digest.as_slice()[..3] == [0; 3])
}
