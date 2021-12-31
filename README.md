# SmolPRNG

This is a small PRNG library/framwork written in pure Rust, that is a translation of another project of mine [SmallPRNG](https://github.com/DKenefake/SmallPRNG). The main goal of this project is to not implement every feature possible but to provide a general framework for implmenting PRNG algorithms to test monte carlo codes.


## Generate Numbers

Generating random numbers is straight forward after initilizing a ``PRNG`` object

```rust
let prng = PRNG{generator: JsfGenerator::default()};

let rand_bool = prng.gen_bool(); // Generates a random bool

let rand_u8 = prng.gen_u8();      //Generates a random u8
let rand_u16 = prng.gen_u16();    //Generates a random u16
let rand_u32 = prng.gen_u32();    //Generates a random u32
let rand_u64 = prng.gen_u64();    //Generates a random u64
let rand_u128 = prng.gen_u128();  //Generates a random u128

let rand_f32 = prng.gen_f32();    //Generates a random f32
let rand_f32 = prng.gen_f64();    //Generates a random f64
```


## Implement Your own algorithm

Here is an example of injecting a new algorithm to generate pseudo-random nunmbers by ``impl`` the ``Algorithm`` trait on a struct. Availible ``Outputs`` are ``u8``,``u16``,``u32``,``u64``,``u128``.

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
```

Using this, we can then create a ``PRNG`` struct from 

```rust
let gen_state = 12765u32; 
let prng = PRNG{generator: StepGenerator{data: gen_state}}
```
