use std::cmp::min;

use rand::{Error, RngCore, SeedableRng};

pub mod tinymt64;
pub mod tinymt32;

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
      for i in 0..min(remaining, bytes.len()) {
        dest[position] = bytes[i];
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
      for i in 0..min(remaining, bytes.len()) {
        dest[position] = bytes[i];
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
  fn test_tinymt64_seed() {
    assert_eq!(0u64, u64::from_le_bytes(TinyMT64Seed::default().0));
    assert_eq!([0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01], TinyMT64Seed::from(0x0102030405060708u64).as_mut());
  }

  #[test]
  fn test_tinymt32_seed() {
    assert_eq!(0u32, u32::from_le_bytes(TinyMT32Seed::default().0));
    assert_eq!([0x04, 0x03, 0x02, 0x01], TinyMT32Seed::from(0x01020304u32).as_mut());
  }

  #[test]
  fn test_chi_squared_tinymt64() {
    let mut random = TinyMT64::from_seed(TinyMT64Seed::from(12345678901234u64));
    test_chi_squared(&mut random);
  }

  #[test]
  fn test_chi_squared_tinymt32() {
    let mut random = TinyMT32::from_seed(TinyMT32Seed::from(1234567890u32));
    test_chi_squared(&mut random);
  }

  #[test]
  fn test_try_fill_bytes_tinymt64() {
    let mut random = TinyMT64::from_seed(TinyMT64Seed::from(12345678901234u64));
    test_try_fill_bytes(&mut random);
  }

  #[test]
  fn test_try_fill_bytes_tinymt32() {
    let mut random = TinyMT32::from_seed(TinyMT32Seed::from(1234567890u32));
    test_try_fill_bytes(&mut random);
  }

  /// Test that the significance level of the chi-square test for random number sequence generated
  /// by the specified PRING is 95% or higher.
  fn test_chi_squared(random: &mut dyn RngCore) {
    const DEGREE_OF_FREEDOM: usize = 9;
    const THRESHOLD: f64 = 16.92;  // 5% for 9 degrees of freedom
    const SAMPLING_COUNT: usize = 1000000;
    let mut histogram = [0; DEGREE_OF_FREEDOM + 1];
    let length = histogram.len();
    for _ in 0..SAMPLING_COUNT {
      histogram[random.gen_range(0, length)] += 1;
    }
    let expected = SAMPLING_COUNT as f64 / length as f64;
    verify_chi_squared(&histogram[..], expected, THRESHOLD);
  }

  /// Acquire random bytes using various buffer lengths and perform a 0.5% chi-square test.
  fn test_try_fill_bytes(random: &mut dyn RngCore) {
    const SAMPLING_COUNT: usize = 200000;
    const THRESHOLD: f64 = 20.3; // 0.5% for 7 degree of freedom
    for size in 1..256 {
      let mut histogram = [0u32; 8];
      let mut buffer = Vec::<u8>::with_capacity(size);
      let mut total = 0;
      for _ in 0..size { buffer.push(0) }
      for _ in 0..SAMPLING_COUNT / size {
        random.try_fill_bytes(&mut buffer).unwrap();
        for i in 0..buffer.len() {
          histogram[(buffer[i] >> 5) as usize] += 1;
        }
        total += buffer.len();
      }
      let expected = total as f64 / histogram.len() as f64;
      verify_chi_squared(&histogram[..], expected, THRESHOLD);
    }
  }

  /// Performs a chi-square test using the specified histogram of uniform random numbers.
  fn verify_chi_squared(histogram: &[u32], expected: f64, threshold: f64) {
    let mut chi2: f64 = 0f64;
    for i in 0..histogram.len() {
      println!("{:2}: {}", i, histogram[i]);
      let actual = histogram[i] as f64;
      chi2 += (expected - actual) * (expected - actual) / expected;
    }
    println!("χ² := {}, expected := {}", chi2, expected);
    assert!(chi2 < threshold);
  }
}