use crate::prng_iter;
use crate::smol_core;
use crate::smol_core::Algorithm;

#[derive(Default)]
pub struct LCG {
    pub(crate) data: u64,
}

prng_iter! {LCG}

impl Algorithm for LCG {
    type Output = u64;

    fn gen(&mut self) -> Self::Output {
        let m = 2862933555777941757u64;
        let a = 1013904243u64;
        let value = self.data;
        self.data = self.data.overflowing_mul(m).0.overflowing_add(a).0;
        value
    }
}
