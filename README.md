# TinyMT
[![Release Build Status for Linux](https://github.com/torao/tinymt/actions/workflows/build.yml/badge.svg)](https://github.com/torao/tinymt/actions)
[![Test Status](https://github.com/torao/tinymt/actions/workflows/test.yml/badge.svg)](https://github.com/torao/tinymt/actions)
[![docs](https://docs.rs/tinymt/badge.svg)](https://docs.rs/tinymt)
[![crates.io](https://img.shields.io/crates/v/tinymt.svg)](https://crates.io/crates/tinymt)

Rust implementation of TinyMT 64/32 - a lightweight variant of Mersenne Twister PRNG.

This crate is based on the original [TinyMT 1.1.1](https://github.com/MersenneTwister-Lab/TinyMT) @ 9d7ca3c161
implemented in C.

## Features

**TinyMT** is a lightweight variant of **Mersenne Twister** MT19937, a widely used PRNG (pseudo-random number generator). This is useful in the case where you don't need so a long period as MT19937, but need sufficient randomness and speed with less memory.

This algorithm works with only 16B internal state space, which is much smaller than the 2,500B of MT19937. The period is
2<sup>127</sup>-1, it's shorter than MT19937 but sufficient for practical use. The cost to generate one random number
was as fast as 4ns on Intel Core i7-8550U CPU.

This crate provides the following two TinyMT implementations for both `std` and `no_std` environments.

* TinyMT32 (32-bit version) for `u32` and `f32`. This is also defined as [RFC 8682](https://tools.ietf.org/html/rfc8682) by IEFT.
* TinyMT64 (64-bit version) for `u64` and `f64`.

TinyMT32 has also been used for random numbers to control which monsters will hatch in Pokémon.

Note that neither TinyMT nor MT 19937 are cryptographically secure pseudo-random number generators. You shouldn't use them in applications where high security is required, such as the generation of private keys.

## Getting Started

You'll be able to use TinyMT by simply adding the dependency to `Cargo.toml` of your project.

```toml
[dependencies]
rand = "0.8"
tinymt = "1.0"
```

By declaring `tinymt` crate at the beginning of the source code, you can use `TinyMT64` or `TinyMT32` to obtain random numbers.

```rust
extern crate tinymt;

use rand::{Rng, SeedableRng};
use tinymt::{TinyMT64, TinyMT64Seed};

fn main() {
  // from nondeterministic seed
  let mut random = TinyMT64::from_entropy();
  let rn = random.gen_range(0.0..1.0);
  println!("{}", rn);   // => nondeterministic but such that 0.3487526381670172

  // from deterministic seed (reproduction of random number sequence is possible)
  let mut random = TinyMT64::from_seed(TinyMT64Seed::from(0u64));
  let rn = random.gen_range(0.0..1.0);
  println!("{}", rn);   // => 0.5531250908497853
}
```

The `TinyMT64` and `TinyMT32` respectively implement the `rand::RngCore` features that are widely-used PRNG interface in Rust. Note that 64-bit operations for `TinyMT32` will generate 32-bit random numbers two times at once for the compatibility of `RngCore`. You should use `u32` or `f32` random number to achieve the best performance in `TinyMT32`.

### Lower-level API

This crate contains two modules `tinymt::tinymt64` and `tinymt::tinymt32` that have been migrated from the original C implementation. They might be useful if you are familiar with the original C implementation. Also, since they are independent of external libraries, it's able to avoid some conflict problems.

See the [API Reference](https://docs.rs/tinymt) for all functions.

## How to Build

The followings are typical `cargo` commands used to test, verify the quality of TinyMT.

```
$ cargo test
$ cargo clippy
$ cargo fmt       # or fmt -- --check
```

## Histories

* 2023-03-15 (v1.0.8) `no-std` available.
* 2022-05-19 (v1.0.7) Migrate the project to 2021 edition.
* 2021-06-12 (v1.0.6) Upgrade `rand` to 0.8.

## Licenses

MIT License

Copyright (c) 2020 Torao Takami

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## See Also

* [Mersenne Twister Home Page](http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/mt.html)
* [Tiny Mersenne Twister (TinyMT): A small-sized variant of Mersenne Twister](http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/TINYMT/index.html)
