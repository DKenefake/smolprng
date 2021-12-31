use crate::prng_iter;
use crate::smol_core;
use crate::smol_core::Algorithm;

#[derive(Default)]
pub struct Sfc32 {
    pub(crate) data: [u32; 4],
}
prng_iter! {Sfc32}

#[derive(Default)]
pub struct Sfc32Small {
    pub(crate) data: u32,
}

prng_iter! {Sfc32Small}

impl Algorithm for Sfc32 {
    type Output = u32;

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
