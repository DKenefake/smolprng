use crate::prng_iter;
use crate::smol_core;
use crate::smol_core::Algorithm;

#[derive(Default)]
pub struct XoShiro256SuperStar {
    pub(crate) data: [u64; 4],
}

prng_iter! {XoShiro256SuperStar}

impl Algorithm for XoShiro256SuperStar {
    type Output = u64;

    fn gen(&mut self) -> Self::Output {
        let result = self.data[1]
            .overflowing_mul(5)
            .0
            .rotate_right(7)
            .overflowing_mul(9)
            .0;
        let t = self.data[1].overflowing_shl(17).0;

        self.data[2] ^= self.data[0];
        self.data[3] ^= self.data[1];
        self.data[1] ^= self.data[2];
        self.data[0] ^= self.data[3];

        self.data[2] ^= t;
        self.data[3] = self.data[3].rotate_right(45);
        result
    }
}
