pub mod jsf;
pub mod lcg;
pub mod middle_square;
pub mod scf32;
pub mod splitmix;
pub mod step_generators;
pub mod xorshift;
pub mod xoshiro;

pub use jsf::*;
pub use lcg::*;
pub use middle_square::*;
pub use scf32::*;
pub use splitmix::*;
pub use step_generators::*;
pub use xorshift::*;
pub use xoshiro::*;

pub use crate::smol_core::*;

#[macro_export]
macro_rules! prng_iter {
    ($algo_struct_name:ty) => {
        impl Iterator for $algo_struct_name {
            type Item = <$algo_struct_name as smol_core::Algorithm>::Output;

            fn next(&mut self) -> Option<Self::Item> {
                Some(self.gen())
            }
        }
    };
}
