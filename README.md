# TinyMT
[![CircleCI](https://circleci.com/gh/torao/tinymt/tree/master.svg?style=shield)](https://circleci.com/gh/torao/tinymt)
[![docs](https://docs.rs/tinymt/badge.svg?version=0.6.21)](https://docs.rs/tinymt)

Rust implementation of TinyMT 64/32 -- Mersenne Twister PRNGs with Lightweight Footprint.

This create is based on the original [TinyMT 1.1.1](https://github.com/MersenneTwister-Lab/TinyMT) @ 9d7ca3c161
implemented in C.

## Algorithm

**TinyMT** is a lightweight version of **Mersenne Twister** MT19937, a widely used PRNG (pseudorandom number generator).

This algorithm only needs a much smaller 127-bit internal state space than MT19927 20,000-bit. The period is
2<sup>127</sup>-1, it's shorter than MT19937 but sufficient for practical use. The cost to generate one random number
was as fast as 4ns on Intel Core i7-8550U CPU for laptop PC.

This crate provides the following two TinyMT implementations.

* TinyMT32 (32-bit version) for `u32` or `f32`
* TinyMT64 (64-bit version) for `u64` or `f64`

## Getting Started

You'll be able to use TinyMT by simply adding the dependency to `Cargo.toml` of your project.

```toml
[dependencies]
rand = "0.7"
tinymt = "1.0"
```

You can declare the `tinymt` crate at the beginning of the source and obtain a random number using `TinyMT64` or its
32-bit version, `TinyMT32`.

```rust
extern crate tinymt;

use rand::{Rng, SeedableRng};
use tinymt::{TinyMT64, TinyMT64Seed};

fn main() {
  // from nondeterministic seed
  let mut random = TinyMT64::from_entropy();
  let rn = random.gen_range(0.0, 1.0);
  println!("{}", rn);   // => nondeterministic but such that 0.3487526381670172

  // from deterministic seed (reproduction of random number sequence is possible)
  let mut random = TinyMT64::from_seed(TinyMT64Seed::from(0u64));
  let rn = random.gen_range(0.0, 1.0);
  println!("{}", rn);   // => 0.5531250908497853
}
```

The `TinyMT64` and `TinyMT32` respectively implement the `rand::RngCore` features that is widely-used PRNG interface in
Rust.

The modules `tinymt::tinymt64` and `tinymt::tinymt32` that have been migrated from the original C implementation are
independent of external libraries. This allows you to use them if you expect exactly the same behavior as the original
C implementation, of if there are some conflicts between the versions of `rand` that TinyMT depends on.

See the [API Reference](https://docs.rs/tinymt) for all functions.

## How to Build

The followings are typical `cargo` commands used to test, verify the quality of TinyMT.

```
$ cargo +nightly test
$ cargo +nightly clippy
$ cargo +nightly fmt       # or fmt -- --check
```

## Licenses

<pre>
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
</pre>

## See Also

* [Mersenne Twister Home Page](http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/mt.html)
* [Tiny Mersenne Twister (TinyMT): A small-sized variant of Mersenne Twister](http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/TINYMT/index.html)
