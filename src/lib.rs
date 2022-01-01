#![crate_name = "smolprng"]
#![warn(missing_docs)]
//! This is a PRNG Package SmolPRNG a small Rust package for pseudo-random  number generation
//!
// mod algorithms::jsf;
pub mod algorithms;
pub mod distributions;
pub mod smol_core;

pub use algorithms::*;
pub use distributions::*;

#[cfg(test)]
mod tests {
    use crate::*;

    macro_rules! prng_gen {
        ($generator:ident) => {
            let mut prng = PRNG {
                generator: $generator,
            };
            prng.gen_u8();
            prng.gen_u16();
            prng.gen_u32();
            prng.gen_u64();
            prng.gen_u128();

            prng.gen_f64();
            prng.gen_f32();

            prng.gen_bool();

            let _v = prng.generator.next();
        };
    }

    // prng_test!{test_step_generator_8, StepGenerator8}

    #[test]
    fn test_step_generator_8() {
        let step = StepGenerator8::default();
        prng_gen!(step);
    }

    #[test]
    fn test_step_generator_16() {
        let step = StepGenerator16::default();
        prng_gen!(step);
    }

    #[test]
    fn test_step_generator_32() {
        let step = StepGenerator32::default();
        prng_gen!(step);
    }

    #[test]
    fn test_step_generator_64() {
        let step = StepGenerator64::default();
        prng_gen!(step);
    }

    #[test]
    fn test_step_generator_128() {
        let step = StepGenerator128::default();
        prng_gen!(step);
    }

    #[test]
    fn test_jsf() {
        let jsf = JsfLarge::default();
        prng_gen!(jsf);
    }

    #[test]
    fn middle_square_test() {
        let middle_square = MiddleSquare::default();
        prng_gen!(middle_square);
    }

    #[test]
    fn lcg_test() {
        let lcg = LCG::default();
        prng_gen!(lcg);
    }

    #[test]
    fn scf32_test() {
        let scf32 = Sfc32::default();
        prng_gen!(scf32);
    }

    #[test]
    fn scf32_small_test() {
        let scf32_small = Sfc32Small::default();
        prng_gen!(scf32_small);
    }

    #[test]
    fn xor32_test() {
        let xor32 = XorShift32::default();
        prng_gen!(xor32);
    }
    #[test]
    fn xor64_test() {
        let xor64 = XorShift64::default();
        prng_gen!(xor64);
    }
    #[test]
    fn xor128_test() {
        let xor128 = XorShift128::default();
        prng_gen!(xor128);
    }
    #[test]
    fn xor128_plus_test() {
        let xor128plus = XorShift128Plus::default();
        prng_gen!(xor128plus);
    }

    #[test]
    fn xoshiro256_super_star_test() {
        let xor128plus = XoShiro256SuperStar::default();
        prng_gen!(xor128plus);
    }

    #[test]
    fn splitmix_32_test() {
        let splitmix = SplitMix32::default();
        prng_gen!(splitmix);
    }

    #[test]
    fn splitmix_64_test(){
        let splitmix = SplitMix64::default();
        prng_gen!(splitmix);
    }

    #[test]
    fn xoshiro_256_plusplus_test(){
        let xorshiro = XoShiro256PlusPlus::default();
        prng_gen!(xorshiro);
    }

    #[test]
    fn xoshiro_256_plus_test(){
        let xorshiro = XoShiro256Plus::default();
        prng_gen!(xorshiro);
    }

    #[test]
    fn xoroshiro_128_plusplus_test(){
        let xorshiro = XoroShiro128PlusPlus::default();
        prng_gen!(xorshiro);
    }


    #[test]
    fn xoroshiro_128_plus_test(){
        let xorshiro = XoroShiro128Plus::default();
        prng_gen!(xorshiro);
    }


    #[test]
    fn xoroshiro_128_superstar_test(){
        let xorshiro = XoroShiro128SuperStar::default();
        prng_gen!(xorshiro);
    }
}
