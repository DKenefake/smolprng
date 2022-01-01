//! This module implments improved versions of the middle square algorithm

use crate::prng_iter;
use crate::smol_core;
use crate::smol_core::Algorithm;

/// This is the simple struct definition for the improved version of the middle squares algorithm
/// this has very strong randomness properties and is very fast in the general setting.

#[derive(Default)]
pub struct MiddleSquare {
    pub(crate) data: [u64; 4],
}
prng_iter! {MiddleSquare}

impl Algorithm for MiddleSquare {
    type Output = u32;

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
