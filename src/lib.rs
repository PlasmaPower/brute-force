//! A library for brute forcing arbitrary computations.
//! Check out the main entrypoint, [brute_force], or the various [adaptors] you
//! can use to write simpler checking functions.
//! For complete examples, look at
//! [the tests directory](https://github.com/PlasmaPower/brute-force/tree/master/src/tests).

use log::warn;
use std::{
    sync::atomic::{self, AtomicBool},
    time::Duration,
};

pub mod adaptors;
mod traits;

#[cfg(test)]
mod tests;

pub use traits::{Advance, Start};

#[non_exhaustive]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Config {
    /// Number of threads to use.
    /// Falls back on `BRUTE_FORCE_THREADS`, or if that doesn't exist, the
    /// number of logical CPU cores.
    pub threads: Option<usize>,
    /// The number of iterations to perform between checking if the computation
    /// is done (or timed out). Defaults to 512.
    pub iters_per_stop_check: usize,
}

impl Config {
    fn get_threads(&self) -> usize {
        if let Some(threads) = self.threads {
            return threads;
        }
        if let Ok(s) = std::env::var("BRUTE_FORCE_THREADS") {
            match s.parse() {
                Ok(t) => return t,
                Err(err) => {
                    warn!(
                        "Failed to parse BRUTE_FORCE_THREADS environment variable: {}",
                        err,
                    );
                }
            }
        }
        num_cpus::get()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            threads: None,
            iters_per_stop_check: 512,
        }
    }
}

fn brute_force_core<F, S, R, RF>(options: Config, f: F, do_recv: RF) -> Option<R>
where
    F: Fn(&mut S) -> Option<R> + Send + Sync,
    S: Start,
    R: Send + Sync,
    RF: FnOnce(&crossbeam_channel::Receiver<R>) -> Option<R>,
{
    let (send, recv) = crossbeam_channel::bounded(1);
    let stopped = AtomicBool::new(false);
    crossbeam_utils::thread::scope(|s| {
        let thread_count = options.get_threads();
        let f = &f;
        let stopped = &stopped;
        for thread in 0..thread_count {
            let send = send.clone();
            s.spawn(move |_| {
                let mut state = S::start_for_thread(thread, thread_count);
                loop {
                    for _ in 0..options.iters_per_stop_check {
                        if let Some(result) = f(&mut state) {
                            let _ = send.try_send(result);
                            return;
                        }
                    }
                    if stopped.load(atomic::Ordering::Relaxed) {
                        return;
                    }
                }
            });
        }
        drop(send); // Ensure panics propagate
        let r = do_recv(&recv);
        stopped.store(true, atomic::Ordering::Relaxed);
        r
    })
    .expect("Brute force host panicked")
}

/// Start a brute force that will run until finding a solution.
pub fn brute_force<F, S, R>(options: Config, f: F) -> R
where
    F: Fn(&mut S) -> Option<R> + Send + Sync,
    S: Start,
    R: Send + Sync,
{
    brute_force_core(options, f, |r| r.recv().ok()).expect("Brute force workers died")
}

/// Start a brute force that will run until finding a solution or timing out.
pub fn brute_force_with_timeout<F, S, R>(options: Config, timeout: Duration, f: F) -> Option<R>
where
    F: Fn(&mut S) -> Option<R> + Send + Sync,
    S: Start,
    R: Send + Sync,
{
    brute_force_core(options, f, |r| r.recv_timeout(timeout).ok())
}
