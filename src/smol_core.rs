//! This is the central module in the SmolPRNG crate, as this is where the definitions of PRNG, Algorithm, and AlgorithmOutput reside

use std::ops::{BitAnd, BitOrAssign, Shl};

/// PRNG is the central pseudo-random number generating front end.
/// This is the front end for the enture package.
pub struct PRNG<T: Algorithm> {
    pub(crate) generator: T,
}

/// This is the central algorithm trait that is implemented bt the generators of the package
pub trait Algorithm {
    /// The ouput type of the Algorithm such as ``u32``, ect.
    type Output: AlgorithmOutput;
    ///
    /// Central generation function of the Algorithm trait, this take in a state struct and returns the ouput of the algorithm
    fn gen(&mut self) -> Self::Output;
}
/// This is the helper trait that that all Algorithms must output a type of
/// ``u8``,``u16``,``u32``,``u64``,``u128``,
pub trait AlgorithmOutput:
    std::ops::Shr<u8, Output = Self> + Shl<u8, Output = Self> + BitOrAssign<Self> + Sized + BitAnd<Self>
{
    /// The size in bytes of the algorithm output
    const SIZE: usize;

    /// Helper function that converts to ``u8``
    fn cast_to_u8(self) -> u8;

    /// Helper function that converts to ``u16``
    fn cast_to_u16(self) -> u16;

    /// Helper function that converts to ``u32``
    fn cast_to_u32(self) -> u32;

    /// Helper function that converts to ``u64``
    fn cast_to_u64(self) -> u64;

    /// Helper function that converts to ``u128``
    fn cast_to_u128(self) -> u128;

    /// Helper function returns that returns the lowest bit of an algorithm output as a bool
    fn get_low(self) -> bool;
}

/// This is a macro that implements the AlgorithmOutput trait for all primitive unsigned integers
macro_rules! algorithm_output {
    ($($t:ty) +) => {
        $(
            impl AlgorithmOutput for $t {

                const SIZE: usize = std::mem::size_of::<$t>();

                fn cast_to_u8(self) -> u8{
                    self as u8
                }

                fn cast_to_u16(self) -> u16{
                    self as u16
                }

                fn cast_to_u32(self) -> u32 {
                    self as u32
                }
                fn cast_to_u64(self) -> u64{
                    self as u64
                }

                fn cast_to_u128(self) -> u128{
                    self as u128
                }

                fn get_low(self) -> bool{
                    self & 1 == 1
                }
            }
        )+
    }
}

algorithm_output! { u8 u16 u32 u64 u128 }

/// This is a macro to generate the generation function for the following types ``u16``,``u32``,``u64``,``u128``,
macro_rules! make_gen {
    ($fn_name:ident, $output:ty, $gen_from:ident, $cast_to:ident) => {
        /// generates a random unsigned integer of appropriate type from algorithm output
        pub fn $fn_name(&mut self) -> $output {
            assert!(T::Output::SIZE.count_ones() == 1);
            const N_SIZE: usize = std::mem::size_of::<$output>();
            if T::Output::SIZE < N_SIZE {
                return (self.$gen_from().$cast_to() << (4 * N_SIZE as u8))
                    | self.$gen_from().$cast_to();
            }
            let val = self.generator.gen();
            let r_shift = ((T::Output::SIZE - N_SIZE) * 8) as u8;
            (val >> r_shift).$cast_to()
        }
    };
}

/// Implementation of the PRNG class over generic trait Algorithm
///
///
///
/// Can be used in the following ways.
/// ```rust
/// let prng = PRNG{generator: JsfGenerator::default()};
///
///let rand_bool = prng.gen_bool(); // Generates a random bool
///
/// let rand_u8 = prng.gen_u8();      //Generates a random u8
/// let rand_u16 = prng.gen_u16();    //Generates a random u16
/// let rand_u32 = prng.gen_u32();    //Generates a random u32
/// let rand_u64 = prng.gen_u64();    //Generates a random u64
/// let rand_u128 = prng.gen_u128();  //Generates a random u128
///
/// let rand_f32 = prng.gen_f32();    //Generates a random f32
/// let rand_f64 = prng.gen_f64();    //Generates a random f64
/// ```
///
///
///
impl<T: Algorithm> PRNG<T> {
    ///Generates a random bool based on Algorithm output
    pub fn gen_bool(&mut self) -> bool {
        self.generator.gen().get_low()
    }

    ///Generates a random ``u8`` based on Algorithm output
    pub fn gen_u8(&mut self) -> u8 {
        assert!(T::Output::SIZE.count_ones() == 1);
        let val = self.generator.gen();
        let r_shift = (T::Output::SIZE as u8 - 1) * 8;
        (val >> r_shift).cast_to_u8()
    }

    make_gen! {gen_u16, u16, gen_u8, cast_to_u16}

    make_gen! {gen_u32, u32, gen_u16, cast_to_u32}

    make_gen! {gen_u64, u64, gen_u32, cast_to_u64}

    make_gen! {gen_u128, u128, gen_u64, cast_to_u128}

    ///Generates a random ``f64`` uniformly distributed on [0,1)
    pub fn gen_f64(&mut self) -> f64 {
        let val = 0x3FFu64 << 52 | self.gen_u64() >> 12;
        f64::from_bits(val) - 1.0f64
    }

    ///Generates a random ``f32`` uniformly distributed on [0,1)
    pub fn gen_f32(&mut self) -> f32 {
        let val = 0x1FFu32 | self.gen_u32() >> 9;
        f32::from_bits(val) - 1.0f32
    }
}

/// Trait signature of the Distribution trait, might be removed and incorperatoed into the PRNG class
pub trait Distribution<T: Algorithm, N> {
    ///Samples a specified distribution
    fn sample(&self, rng: &mut PRNG<T>) -> N;
}
