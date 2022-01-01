//! This is the general implementation of XorShift Family PRNGs
//!
//!

use crate::prng_iter;
use crate::smol_core;
use crate::smol_core::Algorithm;

/// This is the struct definition for the XorShift32 algorithm
#[derive(Default)]
pub struct XorShift32 {
    pub(crate) data: u32,
}

/// This is the struct definition for the XorShift64 algorithm
#[derive(Default)]
pub struct XorShift64 {
    pub(crate) data: u64,
}

/// This is the struct definition for the XorShift128 algorithm
#[derive(Default)]
pub struct XorShift128 {
    pub(crate) data: [u32; 4],
}

/// This is the struct definition for the XorShift128+ algorithm
#[derive(Default)]
pub struct XorShift128Plus {
    pub(crate) data: [u64; 2],
}

prng_iter! {XorShift32}
prng_iter! {XorShift64}
prng_iter! {XorShift128}
prng_iter! {XorShift128Plus}

impl Algorithm for XorShift32 {
    type Output = u32;

    fn gen(&mut self) -> Self::Output {
        let mut x = self.data;
        x ^= x.overflowing_shl(13).0;
        x ^= x >> 17;
        x ^= x.overflowing_shl(5).0;
        self.data = x;
        x
    }
}

impl Algorithm for XorShift64 {
    type Output = u64;

    fn gen(&mut self) -> Self::Output {
        let mut x = self.data;
        x ^= x.overflowing_shl(13).0;
        x ^= x >> 7;
        x ^= x.overflowing_shl(17).0;
        self.data = x;
        x
    }
}

impl Algorithm for XorShift128 {
    type Output = u32;

    fn gen(&mut self) -> Self::Output {
        let mut t = self.data[3];
        let s_ = self.data[0];

        self.data[3] = self.data[2];
        self.data[2] = self.data[1];
        self.data[1] = s_;

        t ^= t.overflowing_shl(11).0;
        t ^= t >> 8;
        self.data[0] = t ^ s_ ^ (s_ >> 19);

        self.data[0]
    }
}

impl Algorithm for XorShift128Plus {
    type Output = u64;

    fn gen(&mut self) -> Self::Output {
        let mut s1 = self.data[0];
        let s0 = self.data[1];
        self.data[0] = s0;

        s1 ^= s1.overflowing_shl(23).0;
        s1 ^= s1 >> 17;
        s1 ^= s0;
        s1 ^= s0 >> 26;
        self.data[1] = s1;

        s1.overflowing_add(s0).0
    }
}
