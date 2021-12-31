use crate::prng_iter;
use crate::smol_core;
use crate::smol_core::Algorithm;

/// This is the simple
#[derive(Default)]
pub struct JsfGenerator {
    pub(crate) data: [u64; 4],
}

prng_iter! {JsfGenerator}

impl Algorithm for JsfGenerator {
    type Output = u64;
    /// The JSF algorithm is a variant of the Bob  PRNG
    ///
    /// Generates a random ``u64`` based on the JSF algorithm proposed by
    /// XXX.
    ///
    fn gen(&mut self) -> u64 {
        assert!(3 <= self.data.len());
        let e = self.data[0].overflowing_sub(self.data[1].rotate_left(7)).0;

        self.data[0] = self.data[1] ^ self.data[2].rotate_left(13);
        self.data[1] = self.data[1].overflowing_add(self.data[3].rotate_left(37)).0;
        self.data[2] = self.data[1].overflowing_add(e).0;
        self.data[3] = e.overflowing_add(self.data[0]).0;

        self.data[3]
    }
}
