//! The Constant Module contains the distributions with constant output

use crate::{Algorithm, Distribution, PRNG};

/// This Defines the Uniform distribution of the range
/// [low, high)
struct Constant<L> {
    value: L,
}

impl<T: Algorithm, N: Copy> Distribution<T, N> for Constant<N> {
    fn sample(&self, _rng: &mut PRNG<T>) -> N {
        self.value
    }
}
