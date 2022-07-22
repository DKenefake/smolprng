use crate::Algorithm;
//
// struct Pcg32{
//     data: [u64;2],
// }
//
// impl Algorithm for Pcg32 {
//     type Output = u32;
//
//     fn gen(&mut self) -> Self::Output {
//         let old_state = self.data[0];
//
//         self.data[0] = old_state * 6364136223846793005u64 + self.data[1] | 1;
//         let xorshifter = ((old_state >> 18)^old_state) >> 27;
//         let rot =  old_state >> 59;
//
//     }
// }

