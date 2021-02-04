use crate::{brute_force, Config, Start};
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use std::{collections::HashMap, sync::Mutex};

const LEN: usize = 5;
const CHAIN_SEGMENT_LEN: usize = 2;

pub struct PollardRhoState {
    prev_input: [u8; LEN],
    current_input: [u8; LEN],
}

impl PollardRhoState {
    fn advance(&mut self, current_output: [u8; LEN]) -> Option<[u8; LEN]> {
        let mut ret = None;
        if current_output[..CHAIN_SEGMENT_LEN] == [0; CHAIN_SEGMENT_LEN] {
            ret = Some(self.prev_input);
            self.prev_input = current_output;
        }
        self.current_input = current_output;
        ret
    }
}

impl Start for PollardRhoState {
    fn start_for_thread(thread: usize, thread_count: usize) -> Self {
        let start = <[u8; LEN]>::start_for_thread(thread, thread_count);
        Self {
            prev_input: start,
            current_input: start,
        }
    }
}

fn hash(input: [u8; LEN]) -> [u8; LEN] {
    let mut hasher = VarBlake2b::new(LEN).unwrap();
    hasher.update(&input);
    let mut hash = [0u8; LEN];
    hasher.finalize_variable(|h| hash.copy_from_slice(h));
    hash
}

#[test]
fn polard_rho_search() {
    // Note: not even close to an efficient implementation of a pollard rho
    // search! In practice you'd want a fixed size array of atomic values, not
    // a hash map behind a mutex.
    let config = Config::default();
    let map = Mutex::new(HashMap::new());
    let f = |state: &mut PollardRhoState| {
        let hash = hash(state.current_input);
        if let Some(prev_input) = state.advance(hash) {
            let mut map = map.lock().unwrap();
            if let Some(other_input) = map.insert(hash, prev_input) {
                if other_input != prev_input {
                    return Some((other_input, prev_input));
                }
            }
        }
        None
    };
    let (first_input, second_input) = brute_force(config, f);
    println!(
        "found colliding hash chains with starts {} and {}",
        hex::encode(first_input),
        hex::encode(second_input),
    );
    // Again, an inefficient algorithm to find the collision
    let mut out_to_in = HashMap::new();
    let mut input = first_input;
    let mut output = hash(input);
    let mut first_iter = true;
    while first_iter || input[..CHAIN_SEGMENT_LEN] != [0; CHAIN_SEGMENT_LEN] {
        out_to_in.insert(output, input);
        input = output;
        output = hash(input);
        first_iter = false;
    }
    input = second_input;
    output = hash(input);
    first_iter = true;
    while first_iter || input[..CHAIN_SEGMENT_LEN] != [0; CHAIN_SEGMENT_LEN] {
        if let Some(&other_input) = out_to_in.get(&output) {
            assert!(input != other_input);
            let other_output = hash(other_input);
            assert_eq!(output, other_output);
            println!(
                "Found collision! {} and {} both output {}",
                hex::encode(input),
                hex::encode(other_input),
                hex::encode(output),
            );
            return;
        }
        input = output;
        output = hash(input);
        first_iter = false;
    }
    panic!("Didn't find collision in brute force result hash chain sections");
}
