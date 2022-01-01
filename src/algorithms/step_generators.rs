//! Implements the ``StepGenerators`` algorithms, these are not ment to be used for tasks that require PRNG.

use crate::prng_iter;
use crate::smol_core;
use crate::smol_core::Algorithm;

macro_rules! make_step {
    ($name:ident,$typing:ty) => {
        /// Simple Struct definition for a StepGenerator
        /// Obviouslt this has no random qualities at all and is only here for benchmarking purposes.
        #[derive(Default)]
        pub struct $name {
            pub(crate) data: $typing,
        }

        impl Algorithm for $name {
            type Output = $typing;

            fn gen(&mut self) -> Self::Output {
                self.data = self.data.overflowing_add(1).0;
                self.data
            }
        }

        prng_iter! {$name}
    };
}

make_step! {StepGenerator8, u8}
make_step! {StepGenerator16, u16}
make_step! {StepGenerator32, u32}
make_step! {StepGenerator64, u64}
make_step! {StepGenerator128, u128}
