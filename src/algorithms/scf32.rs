//! This module contains the implmentations for the SFC family of PRNG algorithms

use crate::smol_core::Algorithm;
use crate::{
    make_1_u32, make_4_u32, make_default_stream, make_stream, prng_setup, AlgorithmOutput, PRNG,
};

/// Simple struct definition for the SFC32 algorithm
/// The translated tree is C -> C++ -> Rust. The original source can be found
/// [here](http://wwwlgis.informatik.uni-kl.de/cms/fileadmin/publications/2020/thesis.pdf)

pub struct Sfc32 {
    pub(crate) data: [u32; 4],
}

prng_setup! {Sfc32, Sfc32,data, make_4_u32}

/// Simple struct definition for the SFC32_Small algorithm
/// The translated tree is C -> C++ -> Rust. The original source can be found
/// [here](http://wwwlgis.informatik.uni-kl.de/cms/fileadmin/publications/2020/thesis.pdf)

pub struct Sfc32Small {
    pub(crate) data: u32,
}

prng_setup! {Sfc32Small, Sfc32Small,data, make_1_u32}

impl Algorithm for Sfc32 {
    type Output = u32;
    #[inline(always)]
    fn gen(&mut self) -> Self::Output {
        self.data[3] = self.data[3].overflowing_add(1).0;
        let t = self.data[0]
            .overflowing_add(self.data[1])
            .0
            .overflowing_add(self.data[2])
            .0;
        self.data[0] ^= self.data[1] >> 9;
        self.data[1] ^= self.data[2].overflowing_shl(3).0;
        self.data[2] = (self.data[2].overflowing_shl(21).0 | (self.data[1] >> 11))
            .overflowing_add(t)
            .0;
        t
    }
}

impl Algorithm for Sfc32Small {
    type Output = u32;
    #[inline(always)]
    fn gen(&mut self) -> Self::Output {
        self.data = self.data.overflowing_add(0x9e3779b9u32).0;
        let mut z = self.data;
        z = z.overflowing_mul(0x85ebca6bu32).0;
        z ^= z >> 13;
        z = z.overflowing_mul(0xc2b2ae35u32).0;
        z ^= z >> 16;
        z
    }
}
