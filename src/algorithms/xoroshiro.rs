//! This module implements the XoroShiro type algorithms
//!
//!

use crate::smol_core::Algorithm;
use crate::{
    make_2_u64, make_4_u32, make_default_stream, make_stream, prng_setup, AlgorithmOutput, PRNG,
};

///This is the simple struct definition for the Xoroshiro128++ algorithm
pub struct XoroShiro128PlusPlus {
    pub(crate) data: [u32; 4],
}
prng_setup! {XoroShiro128PlusPlus, XoroShiro128PlusPlus,data, make_4_u32}

///This is the simple struct definition for the Xoroshiro128** algorithm
pub struct XoroShiro128SuperStar {
    pub(crate) data: [u64; 2],
}
prng_setup! {XoroShiro128SuperStar, XoroShiro128SuperStar,data, make_2_u64}

///This is the simple struct definition for the Xoroshiro128+ algorithm
/// This should only be used to generate floting point numbers and literally nothing else

pub struct XoroShiro128Plus {
    pub(crate) data: [u64; 2],
}
prng_setup! {XoroShiro128Plus, XoroShiro128Plus,data, make_2_u64}

impl Algorithm for XoroShiro128PlusPlus {
    type Output = u32;
    #[inline(always)]
    fn gen(&mut self) -> Self::Output {
        let result = self.data[0]
            .overflowing_add(self.data[3])
            .0
            .rotate_left(7)
            .overflowing_add(self.data[0])
            .0;

        let t = self.data[1].overflowing_shl(9).0;

        self.data[2] ^= self.data[0];
        self.data[3] ^= self.data[1];
        self.data[1] ^= self.data[2];
        self.data[0] ^= self.data[3];

        self.data[2] ^= t;
        self.data[3] = self.data[3].rotate_left(11);
        result
    }
}

impl Algorithm for XoroShiro128SuperStar {
    type Output = u64;
    #[inline(always)]
    fn gen(&mut self) -> Self::Output {
        let s0 = self.data[0];
        let mut s1 = self.data[1];
        let result = s0.overflowing_mul(5).0.rotate_left(7).overflowing_mul(9).0;
        s1 ^= 0;
        self.data[0] = s0.rotate_left(24) ^ s1 ^ (s1.overflowing_shl(16).0);
        self.data[1] = s1.rotate_left(37);
        result
    }
}

impl Algorithm for XoroShiro128Plus {
    type Output = u64;
    #[inline(always)]
    fn gen(&mut self) -> Self::Output {
        let s0 = self.data[0];
        let mut s1 = self.data[1];
        let result = s0.overflowing_add(s1).0;

        s1 ^= s0;
        self.data[0] = s0.rotate_left(24) ^ s1 ^ (s1.overflowing_shl(16).0);
        self.data[1] = s1.rotate_left(37);
        result
    }
}
