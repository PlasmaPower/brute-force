use crate::Advance;

/// Automatically advance the state by incrementing it. Note that you'll likely
/// need to use this adaptor before others, as others assume the function takes
/// a mutable input.
pub fn auto_advance<S, R, F>(f: F) -> impl Fn(&mut S) -> R
where
    S: Advance,
    F: Fn(&S) -> R,
{
    move |s| {
        s.advance();
        f(s)
    }
}

/// Instead of returning an optional output, return a success boolean.
/// If an execution is successful, the brute force output is that execution's
/// input.
pub fn output_input<S, F>(f: F) -> impl Fn(&mut S) -> Option<S>
where
    F: Fn(&mut S) -> bool,
    S: Clone,
{
    move |s| {
        if f(s) {
            Some(s.clone())
        } else {
            None
        }
    }
}

#[cfg(feature = "rand")]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RandomStart<S>(pub S);

#[cfg(feature = "rand")]
impl<S> crate::Start for RandomStart<S>
where
    rand::distributions::Standard: rand::distributions::Distribution<S>,
{
    fn start_for_thread(_thread: usize, _thread_count: usize) -> Self {
        RandomStart(rand::Rng::gen(&mut rand::rngs::OsRng))
    }
}

/// Starts each thread with a cryptographically secure random state. Note that
/// this is normally done for curve25519 scalars, but not normal integers.
#[cfg(feature = "rand")]
pub fn random_start<S, R, F>(f: F) -> impl Fn(&mut RandomStart<S>) -> R
where
    RandomStart<S>: crate::Start,
    F: Fn(&mut S) -> R,
{
    move |s| f(&mut s.0)
}
