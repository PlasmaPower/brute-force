use crate::{brute_force, Config};

#[test]
#[should_panic]
fn propagate_panics() {
    brute_force(Config::default(), |_: &mut u8| -> Option<()> {
        panic!("Test");
    });
}
