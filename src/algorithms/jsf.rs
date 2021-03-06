//! This module impliments JSF type algorithms

use crate::smol_core::Algorithm;
use crate::{make_4_u64, make_default_stream, make_stream, prng_setup, AlgorithmOutput, PRNG};

/// This is the simple struct definition for the 64 Bit JSF Algorithm originally proposed by Bob Jenkins

pub struct JsfLarge {
    pub(crate) data: [u64; 4],
}

prng_setup! {JsfLarge, JsfLarge,data, make_4_u64}

impl Algorithm for JsfLarge {
    type Output = u64;

    /// Translated from original C Source that can be found [here](https://burtleburtle.net/bob/rand/smallprng.html).
    ///
    /// A copy of the original included here for preservation and verification of correctness.
    ///
    ///```C
    ///typedef unsigned long long u8;
    ///typedef struct ranctx { u8 a; u8 b; u8 c; u8 d; } ranctx;
    ///
    /// #define rot(x,k) (((x)<<(k))|((x)>>(64-(k))))
    /// u8 ranval( ranctx *x ) {
    ///     u8 e = x->a - rot(x->b, 7);
    ///     x->a = x->b ^ rot(x->c, 13);
    ///     x->b = x->c + rot(x->d, 37);
    ///     x->c = x->d + e;
    ///    x->d = e + x->a;
    ///     return x->d;
    ///}
    /// ```
    ///
    #[inline(always)]
    fn gen(&mut self) -> u64 {
        assert!(3 <= self.data.len());

        let e = self.data[0].overflowing_sub(self.data[1].rotate_left(7)).0;
        self.data[0] = self.data[1] ^ self.data[2].rotate_left(13);
        self.data[1] = self.data[1].overflowing_add(self.data[3].rotate_left(37)).0;
        self.data[2] = self.data[1].overflowing_add(e).0;
        self.data[3] = e.overflowing_add(self.data[0]).0;

        self.data[3]
    }
}
