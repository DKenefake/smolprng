//! This is the module that contains all the algorithms implmented in the package

pub mod jsf;
pub mod lcg;
pub mod middle_square;
pub mod scf32;
pub mod splitmix;
pub mod step_generators;
pub mod xorshift;
pub mod xoshiro;
pub mod xoroshiro;

pub use jsf::*;
pub use lcg::*;
pub use middle_square::*;
pub use scf32::*;
pub use splitmix::*;
pub use step_generators::*;
pub use xorshift::*;
pub use xoshiro::*;
pub use xoroshiro::*;

pub use crate::smol_core::*;

///A macro that will add the implmentation of ``Iterator`` for a given PRNG
#[macro_export]
macro_rules! prng_iter {
    ($algo_struct_name:ty) => {
        ///Implements Iterator for the PRNG
        impl Iterator for $algo_struct_name {
            type Item = <$algo_struct_name as smol_core::Algorithm>::Output;

            fn next(&mut self) -> Option<Self::Item> {
                Some(self.gen())
            }
        }
    };
}
