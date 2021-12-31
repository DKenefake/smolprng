use crate::prng_iter;
use crate::smol_core;
use crate::smol_core::Algorithm;

#[derive(Default)]
pub struct SplitMix32 {
    pub(crate) data: [u64; 2],
}

prng_iter! {SplitMix32}

impl Algorithm for SplitMix32 {
    type Output = u32;

    fn gen(&mut self) -> Self::Output {
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
