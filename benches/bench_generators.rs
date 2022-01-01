// bench.rs
#![feature(test)]
extern crate test;

use test::Bencher;
use smolprng::{PRNG, JsfLarge, LCG, Sfc32Small, XoroShiro128SuperStar, XoroShiro128PlusPlus, XoroShiro128Plus, XorShift32, XorShift128Plus, XoShiro256PlusPlus, XoShiro256Plus, XoShiro256SuperStar, XorShift64, XorShift128, StepGenerator128, StepGenerator64, StepGenerator32, StepGenerator16, StepGenerator8, SplitMix64, SplitMix32, Sfc32, MiddleSquare};

///Default test it to generate 1kB of random bits using 64 bit generator

macro_rules! bench_maker_1024_byte_by_64_byte{
    ($fn_name:ident, $generator_type:ident)=>{
        #[bench]
        fn $fn_name(b:&mut Bencher){
            b.iter(|| {
                let mut prng = PRNG{generator:$generator_type::default()};
                const SIZE:usize = 128;
                let mut buffer = [0u64;SIZE];
                for i in 0..SIZE{
                    buffer[i] = prng.gen_u64();
                }
            })
        }

    }
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

