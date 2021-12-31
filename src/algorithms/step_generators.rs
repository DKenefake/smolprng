use crate::prng_iter;
use crate::smol_core;
use crate::smol_core::Algorithm;

macro_rules! make_step {
    ($name:ident,$typing:ty) => {
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
