use std::ops::{BitAnd, BitOrAssign, Shl};

/// PRNG is the
pub struct PRNG<T: Algorithm> {
    pub(crate) generator: T,
}

pub trait Algorithm {
    type Output: AlgorithmOutput;

    fn gen(&mut self) -> Self::Output;
}

pub trait AlgorithmOutput:
    std::ops::Shr<u8, Output = Self> + Shl<u8, Output = Self> + BitOrAssign<Self> + Sized + BitAnd<Self>
{
    const SIZE: usize;

    fn cast_to_u8(self) -> u8;
    fn cast_to_u16(self) -> u16;
    fn cast_to_u32(self) -> u32;
    fn cast_to_u64(self) -> u64;
    fn cast_to_u128(self) -> u128;
    fn get_low(self) -> bool;
}

// This is just so you don't have to type out what will be the identical implementation of the trait for all the desired types
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

macro_rules! make_gen {
    ($fn_name:ident, $output:ty, $gen_from:ident, $cast_to:ident) => {
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

impl<T: Algorithm> PRNG<T> {
    pub fn gen_bool(&mut self) -> bool {
        self.generator.gen().get_low()
    }

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

    pub fn gen_f64(&mut self) -> f64 {
        let val = 0x3FFu64 << 52 | self.gen_u64() >> 12;
        f64::from_bits(val) - 1.0f64
    }

    pub fn gen_f32(&mut self) -> f32 {
        let val = 0x1FFu32 | self.gen_u32() >> 9;
        f32::from_bits(val) - 1.0f32
    }
}

pub trait Distribution<T: Algorithm, N> {
    fn sample(&self, rng: &mut PRNG<T>) -> N;
}
