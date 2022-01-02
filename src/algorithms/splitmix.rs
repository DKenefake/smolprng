//! The module contains the Definitions for the SplitMix Algorithm

use crate::smol_core::Algorithm;
use crate::{
    make_1_u64, make_2_u64, make_default_stream, make_stream, prng_setup, AlgorithmOutput, PRNG,
};

/// Simple struct definition of the SplitMix32 Algorithm propsed by Guy Steele et al.
/// original C source code can be found [here](http://wwwlgis.informatik.uni-kl.de/cms/fileadmin/publications/2020/thesis.pdf).
/// The C++ that this package is based on can be found [here](https://github.com/DKenefake/SmallPRNG/blob/master/prng.h).

pub struct SplitMix32 {
    pub(crate) data: [u64; 2],
}

///Simple struct definition of the SplitMix64 Algorithm
pub struct SplitMix64 {
    pub(crate) data: u64,
}

prng_setup! {SplitMix32, SplitMix32,data, make_2_u64}
prng_setup! {SplitMix64, SplitMix64,data, make_1_u64}

impl Algorithm for SplitMix32 {
    type Output = u32;
    #[inline(always)]
    fn gen(&mut self) -> Self::Output {
        //essential for prng quality
        self.data[1] |= 1;

        let mut seed = self.data[0];
        self.data[0] = self.data[0].overflowing_add(self.data[1]).0;
        seed ^= seed >> 33;
        seed = seed.overflowing_mul(0x62a9d9ed799705f5u64).0;
        seed ^= seed >> 28;
        seed = seed.overflowing_mul(0xcb24d0a5c88c35b3u64).0;

        (seed >> 32) as u32
    }
}

impl Algorithm for SplitMix64 {
    type Output = u64;
    #[inline(always)]
    fn gen(&mut self) -> Self::Output {
        self.data = self.data.overflowing_add(0x9E3779B97f4A7C15u64).0;
        let mut result = self.data;
        result = result ^ (result >> 30);
        result = result.overflowing_mul(0xBF58476D1CE4E5B9u64).0;
        result = result ^ (result >> 27);
        result = result.overflowing_mul(0x94D049BB133111EBu64).0;
        result ^ (result >> 31)
    }
}
