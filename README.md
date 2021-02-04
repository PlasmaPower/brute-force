# brute-force: A library for brute forcing arbitrary computations in Rust

This is a library meant to take care of the repetitive tasks of spinning up
threads, checking if the computation is finished, returning the result, and
even generating the inputs.
The adaptor system allows you to compose different helpers in a modular way,
as not all brute forcing is as simple as proof of work.
The common assumption of this library is that each thread will be working off a
state to be the first to find a result, and the computation will end once a
result is found.

## Simple example

```rust
use brute_force::{brute_force, adaptors};
use blake2::{Blake2b, Digest};

#[test]
fn test_proof_of_work() {
    let config = brute_force::Config::default();
    let f = |nonce: &u64| {
        let digest = Blake2b::digest(&nonce.to_le_bytes());
        digest.as_slice()[..3] == [0; 3]
    };
    let nonce = brute_force(config, adaptors::output_input(adaptors::auto_advance(f)));
    let digest = Blake2b::digest(&nonce.to_le_bytes());
    assert!(digest.as_slice()[..3] == [0; 3])
}
```

Here, we use the `auto_advance` adaptor to automatically generate nonces for us,
and we use the `output_input` adaptor to automatically return the input nonce
used if a computation succeeds (instead of manually specifying the output).

For more examples, see
[the src/tests directory](https://github.com/PlasmaPower/brute-force/tree/master/src/tests).
For documentation on the config or adaptors, see
[the docs](https://docs.rs/brute-force).
