//! This is the general module for implmentation of XoShiro type algorithms
//! Te Implmentations are primarilly based on translations of the C code from [Prof. Vigna](https://prng.di.unimi.it/).

use crate::prng_iter;
use crate::smol_core;
use crate::smol_core::Algorithm;

/// This is the struct definition for the state of the XoShiro256** algorithm
#[derive(Default)]
pub struct XoShiro256SuperStar {
    pub(crate) data: [u64; 4],
}

/// This is the struct definition of the state of Xoshiro256++ algorithm
#[derive(Default)]
pub struct XoShiro256PlusPlus{
    pub(crate) data:[u64;4],
}

/// This is the struct definition of the state of Xoshiro256+ algorithm
#[derive(Default)]
pub struct XoShiro256Plus{
    pub(crate) data: [u64; 4],
}



prng_iter! {XoShiro256SuperStar}
prng_iter! {XoShiro256PlusPlus}
prng_iter! {XoShiro256Plus}

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

impl Algorithm for XoShiro256Plus {
    type Output = u64;

    fn gen(&mut self) -> Self::Output {
        let result = self.data[0].overflowing_add(self.data[3]).0;
        let t = self.data[1].overflowing_shl(17).0;

        self.data[2] ^= self.data[0];
        self.data[3] ^= self.data[1];
        self.data[1] ^= self.data[2];
        self.data[0] ^= self.data[3];

        self.data[2] ^= t;
        self.data[3] = self.data[3].rotate_left(45);
        result
    }
}

impl Algorithm for XoShiro256PlusPlus {
    type Output = u64;

    fn gen(&mut self) -> Self::Output {
        let result = (self.data[0].overflowing_add(self.data[0]).0).rotate_left(23).overflowing_add(self.data[0]).0;
        let t = self.data[1].overflowing_shl(17).0;

        self.data[2] ^= self.data[0];
        self.data[3] ^= self.data[1];
        self.data[1] ^= self.data[2];
        self.data[0] ^= self.data[3];

        self.data[2] ^= t;
        self.data[3] = self.data[3].rotate_left(45);
        result
    }
}
