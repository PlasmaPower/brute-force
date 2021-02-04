use std::convert::TryFrom;

pub trait Start {
    fn start_for_thread(thread: usize, thread_count: usize) -> Self;
}

pub trait Advance {
    fn advance(&mut self);
}

macro_rules! impl_for_primitive {
    ($t:ty, $s:ty) => {
        impl Start for $t {
            fn start_for_thread(thread: usize, thread_count: usize) -> Self {
                if let Ok(thread) = Self::try_from(thread) {
                    if let Ok(thread_count) = Self::try_from(thread_count) {
                        (Self::MAX / thread_count) * thread
                    } else {
                        thread
                    }
                } else {
                    0
                }
            }
        }

        impl Advance for $t {
            fn advance(&mut self) {
                *self = self.wrapping_add(1);
            }
        }

        impl Start for $s {
            fn start_for_thread(thread: usize, thread_count: usize) -> Self {
                <$t>::start_for_thread(thread, thread_count) as Self
            }
        }

        impl Advance for $s {
            fn advance(&mut self) {
                *self = self.wrapping_add(1);
            }
        }
    };
}

impl_for_primitive!(u8, i8);
impl_for_primitive!(u16, i16);
impl_for_primitive!(u32, i32);
impl_for_primitive!(u64, i64);
impl_for_primitive!(u128, i128);

#[cfg(feature = "curve25519")]
impl Start for curve25519_dalek::scalar::Scalar {
    fn start_for_thread(_thread: usize, _thread_count: usize) -> Self {
        Self::random(&mut rand::rngs::OsRng)
    }
}

#[cfg(feature = "curve25519")]
impl Advance for curve25519_dalek::scalar::Scalar {
    fn advance(&mut self) {
        *self += curve25519_dalek::scalar::Scalar::one();
    }
}

macro_rules! impl_for_bytes {
    ($n:tt, $($t:tt)*) => {
        impl<$($t)*> Start for [u8; $n] {
            fn start_for_thread(thread: usize, thread_count: usize) -> Self {
                let mut ret = [0u8; $n];
                if $n >= 4 {
                    #[allow(clippy::out_of_bounds_indexing)]
                    ret[..4].copy_from_slice(&u32::start_for_thread(thread, thread_count).to_be_bytes());
                } else {
                    ret[0] = u8::start_for_thread(thread, thread_count);
                }
                ret
            }
        }

        impl<$($t)*> Advance for [u8; $n] {
            fn advance(&mut self) {
                for byte in self.iter_mut().rev() {
                    if *byte < u8::MAX {
                        *byte += 1;
                        return;
                    } else {
                        *byte = 0;
                    }
                }
            }
        }
    }
}

#[cfg(feature = "nightly")]
impl_for_bytes!(N, const N: usize);

#[cfg(not(feature = "nightly"))]
mod impl_bytes {
    use super::*;
    impl_for_bytes!(1,);
    impl_for_bytes!(2,);
    impl_for_bytes!(3,);
    impl_for_bytes!(4,);
    impl_for_bytes!(5,);
    impl_for_bytes!(6,);
    impl_for_bytes!(7,);
    impl_for_bytes!(8,);
    impl_for_bytes!(9,);
    impl_for_bytes!(10,);
    impl_for_bytes!(11,);
    impl_for_bytes!(12,);
    impl_for_bytes!(13,);
    impl_for_bytes!(14,);
    impl_for_bytes!(15,);
    impl_for_bytes!(16,);
    impl_for_bytes!(17,);
    impl_for_bytes!(18,);
    impl_for_bytes!(19,);
    impl_for_bytes!(20,);
    impl_for_bytes!(21,);
    impl_for_bytes!(22,);
    impl_for_bytes!(23,);
    impl_for_bytes!(24,);
    impl_for_bytes!(25,);
    impl_for_bytes!(26,);
    impl_for_bytes!(27,);
    impl_for_bytes!(28,);
    impl_for_bytes!(29,);
    impl_for_bytes!(30,);
    impl_for_bytes!(32,);

    impl_for_bytes!(64,);
    impl_for_bytes!(128,);
    impl_for_bytes!(256,);
}
