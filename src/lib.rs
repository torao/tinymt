use std::cmp::min;

use rand::{Error, RngCore, SeedableRng};

pub mod tinymt32;
pub mod tinymt64;

pub struct TinyMT64Seed(pub [u8; 8]);

impl From<u64> for TinyMT64Seed {
  fn from(seed: u64) -> Self {
    TinyMT64Seed(seed.to_le_bytes())
  }
}

impl From<TinyMT64Seed> for u64 {
  fn from(seed: TinyMT64Seed) -> Self {
    u64::from_le_bytes(seed.0)
  }
}

impl Default for TinyMT64Seed {
  fn default() -> TinyMT64Seed {
    TinyMT64Seed([0; 8])
  }
}

impl AsMut<[u8]> for TinyMT64Seed {
  fn as_mut(&mut self) -> &mut [u8] {
    &mut self.0
  }
}

/// random TinyMT state vector
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct TinyMT64 {
  status: [u64; 2],
  mat1: u32,
  mat2: u32,
  tmat: u64,
}

impl TinyMT64 {
  pub fn from_seed_u64(seed: u64) -> Self {
    Self::from_seed(TinyMT64Seed::from(seed))
  }
}

impl SeedableRng for TinyMT64 {
  type Seed = TinyMT64Seed;

  fn from_seed(seed: Self::Seed) -> Self {
    let mut random = TinyMT64 { status: [0, 0], mat1: 0, mat2: 0, tmat: 0 };
    tinymt64::tinymt64_init(&mut random, u64::from(seed));
    random
  }
}

impl RngCore for TinyMT64 {
  fn next_u32(&mut self) -> u32 {
    self.next_u64() as u32
  }

  fn next_u64(&mut self) -> u64 {
    tinymt64::tinymt64_generate_uint64(self)
  }

  fn fill_bytes(&mut self, dest: &mut [u8]) {
    let mut position = 0;
    let mut remaining = dest.len();
    while remaining > 0 {
      let bytes = self.next_u64().to_le_bytes();
      for b in bytes.iter().take(min(remaining, bytes.len())) {
        dest[position] = *b;
        position += 1;
        remaining -= 1;
      }
    }
  }

  fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
    self.fill_bytes(dest);
    Ok(())
  }
}

pub struct TinyMT32Seed(pub [u8; 4]);

impl From<u32> for TinyMT32Seed {
  fn from(seed: u32) -> Self {
    TinyMT32Seed(seed.to_le_bytes())
  }
}

impl From<TinyMT32Seed> for u32 {
  fn from(seed: TinyMT32Seed) -> Self {
    u32::from_le_bytes(seed.0)
  }
}

impl Default for TinyMT32Seed {
  fn default() -> TinyMT32Seed {
    TinyMT32Seed([0; 4])
  }
}

impl AsMut<[u8]> for TinyMT32Seed {
  fn as_mut(&mut self) -> &mut [u8] {
    &mut self.0
  }
}

/// tinymt32 internal state vector and parameters
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct TinyMT32 {
  status: [u32; 4],
  mat1: u32,
  mat2: u32,
  tmat: u32,
}

impl TinyMT32 {
  pub fn from_seed_u32(seed: u32) -> Self {
    Self::from_seed(TinyMT32Seed::from(seed))
  }
}

impl SeedableRng for TinyMT32 {
  type Seed = TinyMT32Seed;

  fn from_seed(seed: Self::Seed) -> Self {
    let mut random = TinyMT32 { status: [0, 0, 0, 0], mat1: 0, mat2: 0, tmat: 0 };
    tinymt32::tinymt32_init(&mut random, u32::from(seed));
    random
  }
}

impl RngCore for TinyMT32 {
  fn next_u32(&mut self) -> u32 {
    tinymt32::tinymt32_generate_uint32(self)
  }

  fn next_u64(&mut self) -> u64 {
    ((self.next_u32() as u64) << 32) | (self.next_u32() as u64)
  }

  fn fill_bytes(&mut self, dest: &mut [u8]) {
    let mut position = 0;
    let mut remaining = dest.len();
    while remaining > 0 {
      let bytes = self.next_u32().to_le_bytes();
      for b in bytes.iter().take(min(remaining, bytes.len())) {
        dest[position] = *b;
        position += 1;
        remaining -= 1;
      }
    }
  }

  fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
    self.fill_bytes(dest);
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use rand::Rng;

  use super::*;

  #[test]
  fn tinymt_usage() {
    // from nondeterministic seed
    let mut random = TinyMT64::from_entropy();
    let rn = random.gen_range(0.0, 1.0);
    assert!(rn >= 0.0 && rn < 1.0);

    // from deterministic seed (reproduction of random number sequence is possible)
    let mut random = TinyMT64::from_seed(TinyMT64Seed::from(0u64));
    let rn = random.gen_range(0.0, 1.0);
    assert!(rn >= 0.0 && rn < 1.0);
  }
}
