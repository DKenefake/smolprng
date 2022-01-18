//! Implements the `StepGenerators` algorithms, these are not ment to be used for tasks that require PRNG.

use crate::smol_core::Algorithm;
use crate::{
    make_1_u128, make_1_u16, make_1_u32, make_1_u64, make_1_u8, make_default_stream, make_stream,
    prng_setup, AlgorithmOutput, PRNG,
};

macro_rules! make_step {
    ($name:ident,$typing:ty) => {
        /// Simple Struct definition for a `StepGenerator`
        /// Obviouslt this has no random qualities at all and is only here for benchmarking purposes.
        pub struct $name {
            pub(crate) data: $typing,
        }

        impl Algorithm for $name {
            type Output = $typing;

            #[inline(always)]
            fn gen(&mut self) -> Self::Output {
                self.data = self.data.overflowing_add(1).0;
                self.data
            }
        }
    };
}

make_step! {StepGenerator8, u8}
make_step! {StepGenerator16, u16}
make_step! {StepGenerator32, u32}
make_step! {StepGenerator64, u64}
make_step! {StepGenerator128, u128}

prng_setup! {StepGenerator128, StepGenerator128,data, make_1_u128}
prng_setup! {StepGenerator64, StepGenerator64,data, make_1_u64}
prng_setup! {StepGenerator32, StepGenerator32,data, make_1_u32}
prng_setup! {StepGenerator16, StepGenerator16,data, make_1_u16}
prng_setup! {StepGenerator8, StepGenerator8,data, make_1_u8}
