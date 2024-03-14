[![codecov](https://codecov.io/gh/DKenefake/smolprng/branch/master/graph/badge.svg?token=5ZUYXYH6AD)](https://codecov.io/gh/DKenefake/smolprng)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
![crates.io](https://img.shields.io/crates/v/smolprng.svg)
# SmolPRNG

This is a small PRNG library/framwork written in pure Rust, that is a translation of another project of mine, [SmallPRNG](https://github.com/DKenefake/SmallPRNG). The main goal of this project is to not implement every feature possible but to provide a general framework for implementing PRNG algorithms to test monte carlo codes. This was made primarily as a educational project of learning Rust, and it's features, but I hope that this can be used for productive projects like SmallPRNG was. 

To live up to the name of ``SmolPRNG`` there are less than 1000 lines of code but implements over 22 different algorithms out of the box, can sample from 15 statistical distributions this includes all code + tests + docs + benchs.

SmolPRNG performance is competitive to the Rand Rust crate and is much more straightforward to extend. The ``no_std`` feature can be used to remove the standard library dependency, but this is not the default. The compile time is on the order of .2 seconds on modern hardware. This package will also never allocate memory on the heap with or without the ``std`` dependency.
To include this crate in your project, you can use the following in your ``Cargo.toml`` file. With the most recent version being 0.1.6.
```toml
[dependencies]
smolprng = {version = "0.1.6", features = ["std"]}
```

To use this package without a dependency to ``std``, you can use the following in your ``Cargo.toml`` file. Note, as of now, turning of the std dependency will remove most of the advanced distribution sampling, but leaves uniform distribution sampling, bernoulli, binomial, and poisson distributions. The main limitation to including the other distributions is the lack of the certain special functions in the ``core`` library (``ln``, ``sqrt``, ``sin`` ect). 
```toml
[dependencies]
smolprng = {version = "0.1.6", features = ["no_std"]}
```

### Features

- [X] Interface
- [x] PRNG Algorithms
- [x] Generate unsigned ints
- [x] Generate uniform ``f32``,``f64``
- [x] Distributions (Normal, Beta, Cauchy, Bernoulli, ect)
- [x] Easy seeding of algorithm states
- [x] Benchmarking
- [ ] TestU01 Validation


## Generate Numbers

Generating random numbers is straight forward after initializing a ``PRNG`` object

```rust
let prng = PRNG{generator: JsfGenerator::default()};

let rand_bool = prng.gen_bool(); // Generates a random bool

let rand_u8 = prng.gen_u8();      //Generates a random u8
let rand_u16 = prng.gen_u16();    //Generates a random u16
let rand_u32 = prng.gen_u32();    //Generates a random u32
let rand_u64 = prng.gen_u64();    //Generates a random u64
let rand_u128 = prng.gen_u128();  //Generates a random u128

let rand_f32 = prng.gen_f32();    //Generates a random f32
let rand_f64 = prng.gen_f64();    //Generates a random f64
```


## Implement Your own algorithm

Here is an example of injecting a new algorithm to generate pseudo-random numbers by ``impl`` the ``Algorithm`` trait on a struct. Availible ``Outputs`` are ``u8``,``u16``,``u32``,``u64``,``u128``.

```rust
struct StepGenerator{
  state: u32,
}

impl Algorithm for StepGenerator {
  type Output = u32;

  fn gen(&mut self) -> Self::Output {
    self.data = self.data.overflowing_add(1).0;
    self.data
  }
}

// somewhat gross macro, that adds the traits Iterator, Default, and From<U> where U in {u8, u16, u32, u64, u128}
prng_setup! {StepGenerator, StepGenerator, data, make_1_u32}
```

Using this, we can then create a ``PRNG`` struct from 

```rust
// create step generator state from output of SplitMix64 algorithm of a u32 seed
let step_generator = StepGenerator::from(12765u32); 
let prng = PRNG{generator: step_generator}
```

## Performance 

This is an overview of the performance of the package, running on an intel 12700k CPU. This is categorized as three different
benchmarks, generating a 1024 byte buffer filled with random numbers, summing 1 million randomly generated ``f64`` and then 1 million
generated ``f32``. The fastest Step generator result is used as a baseline, as this is the algorithm defined by adding 1 and returning state.
It is the simplest possible algorithm other than the constant generator.


This is not exhaustive but to cover the broad strokes of the performance characters.

### Generating 1024 byte Buffer

|                |  Time | Bandwidth |
|----------------|:-----:|:---------:|
| Step Generator |  49ns | 20.4 GB/s |
| Xoshiro256+    |  87ns | 11.5 GB/s |
| SplitMix64     |  92ns | 10.9 GB/s |
| JSFLarge       | 105ns |  9.5 GB/s |


### Summing 1 million randomly generated ``f32``

|                |  Time   | Time per f32 |
|----------------|:-------:|:------------:|
| Step Generator | 0.28 ms |   0.28 ns    |
| SCFSmall       | 0.51 ms |   0.51 ns    |
| Xoshiro256+    | 0.61 ms |   0.61 ns    |
| SplitMix64     | 0.65 ms |   0.65 ns    |
| JSFLarge       | 0.83 ms |   0.83 ns    |

### Summing 1 million randomly generated ``f64``

|                |  Time   | Time per f64 |
|----------------|:-------:|:------------:|
| Step Generator | 0.32 ms |   0.32 ns    |
| Xoshiro256+    | 0.62 ms |   0.62 ns    |
| SplitMix64     | 0.63 ms |   0.63 ns    |
| JSFLarge       | 0.79 ms |   0.79 ns    |
