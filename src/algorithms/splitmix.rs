//! The module contains the Definitions for the SplitMix Algorithm

use crate::prng_iter;
use crate::smol_core;
use crate::smol_core::Algorithm;

///Simple struct definition of the SplitMix32 Algorithm propsed by Guy Steele et al.
/// original C source code can be found [here](http://wwwlgis.informatik.uni-kl.de/cms/fileadmin/publications/2020/thesis.pdf).
/// The C++ that this package is based on can be found [here](https://github.com/DKenefake/SmallPRNG/blob/master/prng.h).
#[derive(Default)]
pub struct SplitMix32 {
    pub(crate) data: [u64; 2],
}

prng_iter! {SplitMix32}

impl Algorithm for SplitMix32 {
    type Output = u32;

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
