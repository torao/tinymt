# TinyMT [![CircleCI](https://circleci.com/gh/torao/tinymt.svg?style=svg)](https://circleci.com/gh/torao/tinymt)

Rust implementation for TinyMT 64/32 -- Mersenne Twister PRNGs with Lightweight Footprint. This repository is based on
the original [TinyMT 1.1.1](https://github.com/MersenneTwister-Lab/TinyMT) @ 9d7ca3c161 implemented in C.

## Getting Started

You would use TinyMT random numbers by simply adding the URL of this repository to your project's `Cargo.toml`.

```toml
[dependencies]
rand = "0.7"
tinymt = { git = "https://github.com/torao/tinymt", tag = "1.0.0" }
```

In the head of your source, please declare the `tinymt` crate, and then  retrieve random numbers using `TinyMT64` or
its 32-bit edition, `TinyMT32`.

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


## How to Build

The followings are typical `cargo` commands used to test, verify the quality of TinyMT.

```
$ cargo +nightly test
$ cargo +nightly clippy
$ cargo +nightly fmt       # or fmt -- --check
```

## Licenses

```
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
```

## See Also

* [Mersenne Twister Home Page](http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/mt.html)
* [Tiny Mersenne Twister (TinyMT): A small-sized variant of Mersenne Twister](http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/TINYMT/index.html)
