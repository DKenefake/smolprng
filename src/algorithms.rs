//! This is the module that contains all the algorithms implmented in the package

pub mod jsf;
pub mod lcg;
pub mod middle_square;
pub mod scf32;
pub mod splitmix;
pub mod step_generators;
pub mod xoroshiro;
pub mod xorshift;
pub mod xoshiro;

pub use jsf::*;
pub use lcg::*;
pub use middle_square::*;
pub use scf32::*;
pub use splitmix::*;
pub use step_generators::*;
pub use xoroshiro::*;
pub use xorshift::*;
pub use xoshiro::*;

use crate::{AlgorithmOutput, PRNG};

///A macro that will write the streaming building for us
#[macro_export]
macro_rules! prng_setup {
    ($algo_type:ty, $algo_name:ident, $data:ident, $data_maker:ident) => {
        ///Implements Iterator for the PRNG
        impl Iterator for $algo_type {
            type Item = <$algo_type as Algorithm>::Output;

            fn next(&mut self) -> Option<Self::Item> {
                Some(self.gen())
            }
        }

        impl Default for $algo_type {
            fn default() -> Self {
                let prng = PRNG {
                    generator: make_default_stream(),
                };
                $algo_name {
                    $data: $data_maker(prng),
                }
            }
        }
        impl<T: AlgorithmOutput> From<T> for $algo_type {
            fn from(value: T) -> Self {
                let prng = PRNG {
                    generator: make_stream::<T>(value),
                };
                $algo_name {
                    $data: $data_maker(prng),
                }
            }
        }
    };
}

///makes a data streamer from using some input data
pub fn make_stream<T: AlgorithmOutput>(value: T) -> SplitMix64 {
    let data_feed = match T::SIZE > 8 {
        false => 0x43d0f2c5f0c7e0a5 ^ value.cast_to_u64(),
        true => 0x43d0f2c5f0c7e0a5 ^ (value >> ((T::SIZE - 8) * 8) as u8).cast_to_u64(),
    };
    SplitMix64 { data: data_feed }
}

///makes the default data streamer
pub const fn make_default_stream() -> SplitMix64 {
    SplitMix64 {
        data: 0x43d0f2c5f0c7e0a5,
    }
}

//
// These are all helper function to generate state
//

///makes 1 u8
pub fn make_1_u8(mut prng: PRNG<SplitMix64>) -> u8 {
    prng.gen_u8()
}

///makes a u16
pub fn make_1_u16(mut prng: PRNG<SplitMix64>) -> u16 {
    prng.gen_u16()
}

///makes a u32
pub fn make_1_u32(mut prng: PRNG<SplitMix64>) -> u32 {
    prng.gen_u32()
}

///makes 4 u32
pub fn make_4_u32(mut prng: PRNG<SplitMix64>) -> [u32; 4] {
    [
        prng.gen_u32(),
        prng.gen_u32(),
        prng.gen_u32(),
        prng.gen_u32(),
    ]
}
///makes 1 u64
pub fn make_1_u64(mut prng: PRNG<SplitMix64>) -> u64 {
    prng.gen_u64()
}

///makes 2 u64
pub fn make_2_u64(mut prng: PRNG<SplitMix64>) -> [u64; 2] {
    [prng.gen_u64(), prng.gen_u64()]
}

///makes 4 u64
pub fn make_4_u64(mut prng: PRNG<SplitMix64>) -> [u64; 4] {
    [
        prng.gen_u64(),
        prng.gen_u64(),
        prng.gen_u64(),
        prng.gen_u64(),
    ]
}
/// makes a u128
pub fn make_1_u128(mut prng: PRNG<SplitMix64>) -> u128 {
    prng.gen_u128()
}
