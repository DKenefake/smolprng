#![crate_name = "smolprng"]
#![warn(missing_docs)]
#![forbid(unsafe_code)] // forbid unsafe code

#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::implicit_return)]
#![allow(clippy::float_arithmetic)]
#![allow(clippy::inline_always)]
#![allow(clippy::suboptimal_flops)]
#![allow(clippy::use_self)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::unreadable_literal)]

//! This is a PRNG Package `SmolPRNG` a small Rust package for pseudo-random  number generation
//!
extern crate alloc;

// mod algorithms::jsf;
pub mod algorithms;
pub mod smol_core;

pub use algorithms::*;
pub use smol_core::*;
#[cfg(test)]
mod tests {
    use crate::*;

    macro_rules! prng_gen {
        ($generator:ident, $generator_type:ty, $dist_run:expr) => {
            let mut prng = PRNG {
                generator: $generator,
            };

            prng.gen_u8();

            prng = PRNG {
                generator: <$generator_type>::from(5u8),
            };

            prng.gen_u16();
            prng = PRNG {
                generator: <$generator_type>::from(5u16),
            };

            prng.gen_u32();
            prng = PRNG {
                generator: <$generator_type>::from(5u32),
            };

            prng.gen_u64();
            prng = PRNG {
                generator: <$generator_type>::from(5u64),
            };

            prng.gen_u128();
            prng = PRNG {
                generator: <$generator_type>::from(5u128),
            };

            prng.gen_f64();
            prng.gen_f32();

            prng.gen_bool();

            let _v = prng.generator.next();

            if $dist_run {

                prng.poisson(1f64);
                prng.poisson(12f64);
                #[cfg(feature = "std")]
                prng.gamma(0.5, 0.5);
                #[cfg(feature = "std")]
                prng.chi_squared(10.0);
                #[cfg(feature = "std")]
                prng.normal();
                prng.bernoulli(0.5);
                #[cfg(feature = "std")]
                prng.beta(1.0, 1.0);
                prng.binomial(10, 0.3);
                #[cfg(feature = "std")]
                prng.cauchy();
                #[cfg(feature = "std")]
                prng.exponential(-10.0);
                #[cfg(feature = "std")]
                prng.fischer(25f64, 10f64);
                #[cfg(feature = "std")]
                prng.logistic(0.0, 10.0);
                #[cfg(feature = "std")]
                prng.lognormal();
                #[cfg(feature = "std")]
                prng.negative_binomial(10.0, 2.0);
            }
        };
    }

    // prng_test!{test_step_generator_8, StepGenerator8}

    macro_rules! gen_init_test {
        ($fn_name:ident, $gen_type:ty, $check_dist:expr) => {
            #[test]
            fn $fn_name() {
                let prng = <$gen_type>::default();
                prng_gen! {prng, $gen_type, $check_dist};
            }
        };
    }

    gen_init_test! {test_step_generator_8, StepGenerator8, false}
    gen_init_test! {test_step_generator_16, StepGenerator16, false}
    gen_init_test! {test_step_generator_32, StepGenerator32, false}
    gen_init_test! {test_step_generator_64, StepGenerator64, false}
    gen_init_test! {test_step_generator_128, StepGenerator128, false}

    gen_init_test! {test_jsf_large, JsfLarge, true}
    gen_init_test! {test_middle_square, MiddleSquare, false}
    gen_init_test! {test_lcg, LCG, false}

    gen_init_test! {test_scf_32, Sfc32, false}
    gen_init_test! {test_scf_32_small, Sfc32Small, false}

    gen_init_test! {test_xor32, XorShift32, false}
    gen_init_test! {test_xor64, XorShift64, false}
    gen_init_test! {test_xor128, XorShift128, false}
    gen_init_test! {test_xor128_plus, XorShift128Plus, false}

    gen_init_test! {test_xoshiro_256_super_star, XoShiro256SuperStar, false}

    gen_init_test! {test_splitmix_32, SplitMix32, false}
    gen_init_test! {test_splitmix_64, SplitMix64, false}

    gen_init_test! {test_xoshiro_256_plus, XoShiro256Plus, false}
    gen_init_test! {test_xoshiro_256_plus_plus, XoShiro256PlusPlus, false}

    gen_init_test! {test_xoroshiro_256_plus_plus, XoroShiro128PlusPlus, false}
    gen_init_test! {test_xoroshiro_256_plus, XoroShiro128Plus, false}
    gen_init_test! {test_xorpshiro_256_super_star, XoroShiro128SuperStar, false}
}
