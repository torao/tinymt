extern crate tinymt;

use rand::{Rng, RngCore, SeedableRng};

use tinymt::{TinyMT32, TinyMT32Seed, TinyMT64, TinyMT64Seed};

pub mod tinymt32;
pub mod tinymt64;

#[test]
fn test_tinymt64_seed() {
  assert_eq!(0u64, u64::from_le_bytes(TinyMT64Seed::default().0));
  assert_eq!(
    [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01],
    TinyMT64Seed::from(0x0102030405060708u64).as_mut()
  );
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
  const THRESHOLD: f64 = 16.92; // 5% for 9 degrees of freedom
  const SAMPLING_COUNT: usize = 1000000;
  let mut histogram = [0; DEGREE_OF_FREEDOM + 1];
  let length = histogram.len();
  for _ in 0..SAMPLING_COUNT {
    histogram[random.gen_range(0..length)] += 1;
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
    for _ in 0..size {
      buffer.push(0)
    }
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
  for (i, h) in histogram.iter().enumerate() {
    println!("{:2}: {}", i, *h);
    let actual = *h as f64;
    chi2 += (expected - actual) * (expected - actual) / expected;
  }
  println!("χ² := {}, expected := {}", chi2, expected);
  assert!(chi2 < threshold);
}
