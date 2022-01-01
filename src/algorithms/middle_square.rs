//! This module implments improved versions of the middle square algorithm

use crate::smol_core::Algorithm;
use crate::{make_4_u64, make_default_stream, make_stream, prng_setup, AlgorithmOutput, PRNG};

/// This is the simple struct definition for the improved version of the middle squares algorithm
/// this has very strong randomness properties and is very fast in the general setting.

pub struct MiddleSquare {
    pub(crate) data: [u64; 4],
}
// prng_iter! {MiddleSquare}

prng_setup! {MiddleSquare, MiddleSquare,data, make_4_u64}

impl Algorithm for MiddleSquare {
    type Output = u32;

    #[inline(always)]
    fn gen(&mut self) -> Self::Output {
        let mut x = self.data[0];
        let mut w = self.data[1];
        w = w.overflowing_add(self.data[2]).0;
        x = x.overflowing_mul(x).0;
        x = x.overflowing_add(w).0;

        x = (x >> 32) | (x.overflowing_shl(32).0);
        self.data[0] = x;
        self.data[1] = w;
        (x >> 32) as u32
    }
}
