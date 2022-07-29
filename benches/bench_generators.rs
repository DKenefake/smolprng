// bench.rs
#![feature(test)]
extern crate test;

use smolprng::{
    JsfLarge, MiddleSquare, Sfc32, Sfc32Small, SplitMix32, SplitMix64, StepGenerator128,
    StepGenerator16, StepGenerator32, StepGenerator64, StepGenerator8, XoShiro256Plus,
    XoShiro256PlusPlus, XoShiro256SuperStar, XorShift128, XorShift128Plus, XorShift32, XorShift64,
    XoroShiro128Plus, XoroShiro128PlusPlus, XoroShiro128SuperStar, LCG, PRNG,
};
use test::Bencher;

use std::time::SystemTime;

///Default test it to generate 1kB of random bits using 64 bit generator

macro_rules! bench_maker_1024_byte_by_64_byte {
    ($fn_name:ident, $generator_type:ident) => {
        #[bench]
        fn $fn_name(b: &mut Bencher) {
            let mut buffer = [0u64; SIZE];
            let a = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let mut prng = PRNG {
                generator: $generator_type::from(a),
            };
            const SIZE: usize = 128;
            b.iter(|| {
                for i in 0..SIZE {
                    buffer[i] = prng.gen_u64();
                }
            })
        }
    };
}

macro_rules! bench_maker_1_000_000_fp {
    ($fn_name_1:ident,$fn_name_2:ident, $generator_type:ident) => {
        #[bench]
        fn $fn_name_1(b: &mut Bencher) {
            let mut prng = PRNG {
                generator: $generator_type::default(),
            };
            const SIZE: usize = 1_000_000;
            let mut buffer = 0f32;

            b.iter(|| {
                for _i in 0..SIZE {
                    buffer += prng.gen_f32();
                }
                buffer = 0f32;
            })
        }

        #[bench]
        fn $fn_name_2(b: &mut Bencher) {
            let mut prng = PRNG {
                generator: $generator_type::default(),
            };
            const SIZE: usize = 1_000_000;
            let mut buffer = 0f64;

            b.iter(|| {
                for _i in 0..SIZE {
                    buffer += prng.gen_f64();
                }
                buffer = 0f64;
            })
        }
    };
}

bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_jsf_large, JsfLarge}
bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_lcg, LCG}
bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_middle_square, MiddleSquare}
bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_sfc32, Sfc32}
bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_sfc_small, Sfc32Small}
bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_splitmix_32, SplitMix32}
bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_splitmix_64, SplitMix64}

bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_stepgenerator_8, StepGenerator8}
bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_stepgenerator_16, StepGenerator16}
bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_stepgenerator_32, StepGenerator32}
bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_stepgenerator_64, StepGenerator64}
bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_stepgenerator_128, StepGenerator128}

bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_xoroshiro_super_star, XoroShiro128SuperStar}
bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_xoroshiro_plus_plus, XoroShiro128PlusPlus}
bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_xoroshiro_plus, XoroShiro128Plus}

bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_xorshift_32, XorShift32}
bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_xorshift_64, XorShift64}
bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_xorshift_128, XorShift128}
bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_xorshift_128_plus, XorShift128Plus}

bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_xoshiro_256_plus_plus, XoShiro256PlusPlus}
bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_xoshiro_256_plus, XoShiro256Plus}
bench_maker_1024_byte_by_64_byte! {bench_1024_bytes_xoshiro_256_super_star, XoShiro256SuperStar}

bench_maker_1_000_000_fp! {bench_1m_fp64_jsf_large,bench_1m_fp32_jsf_large, JsfLarge}
bench_maker_1_000_000_fp! {bench_1m_fp64_lcg,bench_1m_fp32_lcg, LCG}
bench_maker_1_000_000_fp! {bench_1m_fp64_middle_square,bench_1m_fp32_middle_square, MiddleSquare}
bench_maker_1_000_000_fp! {bench_1m_fp64_sfc32,bench_1m_fp32_sfc32, Sfc32}
bench_maker_1_000_000_fp! {bench_1m_fp64_sfc_small,bench_1m_fp32_sfc_small, Sfc32Small}
bench_maker_1_000_000_fp! {bench_1m_fp64_splitmix_32,bench_1m_fp32_splitmix_32, SplitMix32}
bench_maker_1_000_000_fp! {bench_1m_fp64_splitmix_64,bench_1m_fp32_splitmix_64, SplitMix64}

bench_maker_1_000_000_fp! {bench_1m_fp64_stepgenerator_8,bench_1m_fp32_stepgenerator_8, StepGenerator8}
bench_maker_1_000_000_fp! {bench_1m_fp64_stepgenerator_16,bench_1m_fp32_stepgenerator_16, StepGenerator16}
bench_maker_1_000_000_fp! {bench_1m_fp64_stepgenerator_32,bench_1m_fp32_stepgenerator_32, StepGenerator32}
bench_maker_1_000_000_fp! {bench_1m_fp64_stepgenerator_64,bench_1m_fp32_stepgenerator_64, StepGenerator64}
bench_maker_1_000_000_fp! {bench_1m_fp64_stepgenerator_128,bench_1m_fp32_stepgenerator_128, StepGenerator128}

bench_maker_1_000_000_fp! {bench_1m_fp64_xoroshiro_super_star,bench_1m_fp32_xoroshiro_super_star, XoroShiro128SuperStar}
bench_maker_1_000_000_fp! {bench_1m_fp64_xoroshiro_plus_plus,bench_1m_fp32_xoroshiro_plus_plus, XoroShiro128PlusPlus}
bench_maker_1_000_000_fp! {bench_1m_fp64_xoroshiro_plus,bench_1m_fp32_xoroshiro_plus, XoroShiro128Plus}

bench_maker_1_000_000_fp! {bench_1m_fp64_xorshift_32,bench_1m_fp32_xorshift_32, XorShift32}
bench_maker_1_000_000_fp! {bench_1m_fp64_xorshift_64,bench_1m_fp32_xorshift_64, XorShift64}
bench_maker_1_000_000_fp! {bench_1m_fp64_xorshift_128,bench_1m_fp32_xorshift_128, XorShift128}
bench_maker_1_000_000_fp! {bench_1m_fp64_xorshift_128_plus,bench_1m_fp32_xorshift_128_plus, XorShift128Plus}

bench_maker_1_000_000_fp! {bench_1m_fp64_xoshiro_256_plus_plus,bench_1m_fp32_xoshiro_256_plus_plus, XoShiro256PlusPlus}
bench_maker_1_000_000_fp! {bench_1m_fp64_xoshiro_256_plus,bench_1m_fp32_xoshiro_256_plus, XoShiro256Plus}
bench_maker_1_000_000_fp! {bench_1m_fp64_xoshiro_256_super_star,bench_1m_fp32_xoshiro_256_super_star, XoShiro256SuperStar}
