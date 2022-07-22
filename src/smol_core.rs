//! This is the central module in the `SmolPRNG` crate, as this is where the definitions of `PRNG`, `Algorithm`, and `AlgorithmOutput` reside

use std::f64::consts::E;
use std::ops::{BitAnd, BitOrAssign, Shl};

/// PRNG is the central pseudo-random number generating front end.
/// This is the front end for the enture package.
pub struct PRNG<T: Algorithm> {
    /// The algorithm that is inserted to the generator
    pub generator: T,
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

/// This is a macro that implements the `AlgorithmOutput` trait for all primitive unsigned integers
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
        #[inline(always)]
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
/// use smolprng::JsfLarge;
/// use smolprng::PRNG;
/// let mut prng = PRNG{generator: JsfLarge::default()};
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
    #[inline(always)]
    pub fn gen_bool(&mut self) -> bool {
        self.generator.gen().get_low()
    }

    ///Generates a random ``u8`` based on Algorithm output
    #[inline(always)]
    pub fn gen_u8(&mut self) -> u8 {
        let val = self.generator.gen();
        let r_shift = (T::Output::SIZE as u8 - 1) * 8;
        (val >> r_shift).cast_to_u8()
    }

    make_gen! {gen_u16, u16, gen_u8, cast_to_u16}

    make_gen! {gen_u32, u32, gen_u16, cast_to_u32}

    make_gen! {gen_u64, u64, gen_u32, cast_to_u64}

    make_gen! {gen_u128, u128, gen_u64, cast_to_u128}

    ///Generates a random ``f64`` uniformly distributed on [0,1)
    #[inline(always)]
    pub fn gen_f64(&mut self) -> f64 {
        let val = 0x3FFu64 << 52 | self.gen_u64() >> 12;
        f64::from_bits(val) - 1.0f64
    }

    ///Generates a random ``f32`` uniformly distributed on [0,1)
    #[inline(always)]
    pub fn gen_f32(&mut self) -> f32 {
        let val = 0x1FFu32 | self.gen_u32() >> 9;
        f32::from_bits(val) - 1.0f32
    }

    /// Samples a normal distribution N(0,1) for 1 sample
    /// Algorithm from "A Note on the Generation of Random Normal Deviates" - G. E. P. Box, Mervin E. Muller The Annals of Mathematical Statistics 1958
    pub fn normal(&mut self) -> f64 {
        let (mut u, mut s) = (0f64, 0f64);

        while s >= 1f64 || s == 0f64 {
            u = self.gen_f64() * 2.0 - 1.0;
            let v = self.gen_f64() * 2.0 - 1.0;
            s = u * u + v * v;
        }

        s = (-2f64 * (s.ln()) / s).sqrt();
        u * s
    }

    ///Samples a bernoulli distribution with B(p)
    pub fn bernoulli(&mut self, p: f64) -> u64 {
        if p > self.gen_f64() {
            1
        } else {
            0
        }
    }

    ///Samples a binomial distribution with B(n,p)
    pub fn binomial(&mut self, n: u64, p: f64) -> u64 {
        let mut count = 0;
        for _i in 0..n {
            count += self.bernoulli(p);
        }
        count
    }

    ///Samples a Cauchy Distribution
    /// Algorithm based on the fact that X~N(0,1) & Y~N(0,1) have  X/Y~C(0,1)
    pub fn cauchy(&mut self) -> f64 {
        self.normal() / self.normal()
    }

    ///Samples the student t distribution
    /// Algorithm From "Polar generation of random variates with the t-Distibution" - Ralph W. Bailey Mathematics of Computation 1994
    /// DOI: https://doi.org/10.2307/2153537
    pub fn student_t(&mut self, nu:f64)-> f64{

        let (mut u, mut v, mut w) = (0f64, 0f64, 0f64);

        while w > 1f64{
            u = self.gen_f64()*2f64 - 1f64;
            v = self.gen_f64()*2f64 - 1f64;
            w = u*u+v*v;
        }

        let c = u*u / w;
        let r = nu*(w.powf(-2f64/v)-1f64);
        let p_res = (c*c*r*r).sqrt();

        match self.gen_bool() {
            true => p_res,
            false => -p_res
        }

    }

    /// Samples a Gamma Distribution with Γ(α,ß)
    /// Algorithm from "A simple method for generating gamma variables" - George Marsaglia, Wai Wan Tsang ACM Transactions on  Mathematical Software 2000
    /// DOI: https://doi.org/10.1145/358407.358414
    pub fn gamma(&mut self, alpha: f64, beta: f64) -> f64 {
        if alpha <= 1f64 {
            return self.gamma(alpha + 1.0, beta) * (self.gen_f64().powf(1f64 / alpha));
        }

        let d = alpha - 1f64 / 3f64;
        let c = 1f64 / ((9f64 * d).sqrt());

        loop {
            let u = self.gen_f64();
            let x = self.normal();
            let mut v = 1f64 + c * x;

            v = v * v * v;

            if v > 0f64 && u.ln() < 0.5 * x * x + d - d * v + d * v.ln() {
                return d * v * beta;
            }
        }
    }

    /// Samples a Chi square distribution with nu degrees of freedom
    /// Based on relating chi squared distirubtion to the gamma distribution
    /// Y ~ Y(nu) <==> Y ~ gamma(0.5 nu, 2)
    pub fn chi_squared(&mut self, nu: f64) -> f64 {
        self.gamma(0.5 * nu, 2.0)
    }

    /// Samples a beta distribution
    /// Given X ~ gamma(alpha, 1) and Y ~ gamma(beta, 1) then X/(X+Y) ~ beta(alpha, beta)
    pub fn beta(&mut self, alpha: f64, beta: f64) -> f64 {
        let x = self.gamma(alpha, 1f64);
        let y = self.gamma(beta, 1f64);
        x / (x + y)
    }

    /// Samples a exponential distribution
    /// Direct inversion of CDF
    pub fn exponential(&mut self, lambda: f64) -> f64 {
        -self.gen_f64().ln() / lambda
    }

    /// Samples a log normal distribution
    /// by definition
    pub fn lognormal(&mut self) -> f64 {
        self.normal().exp()
    }

    /// Samples a logistic distribution
    ///
    pub fn logistic(&mut self, mu: f64, beta: f64) -> f64 {
        let x = self.gen_f64();
        mu + beta * ((x / (1.0 - x)).ln())
    }

    /// Samples a fischer distribution
    pub fn fischer(&mut self, d1: f64, d2: f64) -> f64 {
        let x_1 = self.chi_squared(d1);
        let x_2 = self.chi_squared(d1);
        (x_1 / d1) / (x_2 / d2)
    }

    /// Samples a poisson distribution
    pub fn poisson(&mut self, l: f64) -> u64 {
        let mut n = 0;
        let mut m = 0;
        let cutoff_f = 10f64;
        let mut l_ = l;

        while l_ > cutoff_f {
            m += self.poisson(cutoff_f);
            l_ -= cutoff_f;
        }

        let cdf = self.gen_f64() * E;
        let mut prod = 1f64;
        let mut denom = 1f64;
        let mut sum = 1f64;

        while sum < cdf {
            n += 1;
            prod *= l;
            denom *= n as f64;
            sum += prod / denom;
        }

        n + m
    }

    /// Samples a negative binomial distribution
    /// Relation between negative binaomial and gamma, then gamma to poisson
    pub fn negative_binomial(&mut self, r: f64, p: f64) -> u64 {
        let lambda = self.gamma(r, p / (1.0 - p));
        self.poisson(lambda)
    }
}
