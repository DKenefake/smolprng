//! This module implments the LCG family of PRNG generators

use crate::smol_core::Algorithm;
use crate::{make_1_u64, make_default_stream, make_stream, prng_setup, AlgorithmOutput, PRNG};

/// This is the simple struct definition for the 64 Bit Linear Congruential Generator (LCG) algorithm, with parameters as definied by Knuth
///
/// This is overall a very fast algorithm, but LCGs have fairly bad randomness properties and should be avoided for applications that need high quility random numbers
///

pub struct LCG {
    pub(crate) data: u64,
}

// prng_iter! {LCG}
prng_setup! {LCG, LCG,data, make_1_u64}

impl Algorithm for LCG {
    type Output = u64;
    /// The general LCG Algorithm is the following
    /// X_+ = (mX + a)
    ///
    /// ```rust
    /// let m = 2862933555777941757u64;
    /// let a = 1013904243u64;
    /// ```
    #[inline(always)]
    fn gen(&mut self) -> Self::Output {
        let m = 2862933555777941757u64;
        let a = 1013904243u64;
        let value = self.data;
        self.data = self.data.overflowing_mul(m).0.overflowing_add(a).0;
        value
    }
}
