use std::cmp::min;

use crate::TinyMT64;

const TINYMT64_MEXP: usize = 127;
const TINYMT64_SH0: u64 = 12;
const TINYMT64_SH1: u64 = 11;
const TINYMT64_SH8: u64 = 8;
const TINYMT64_MASK: u64 = 0x7fff_ffff_ffff_ffff_u64;
const TINYMT64_MUL: f64 = 1.0 / 9_007_199_254_740_992.0;
const MIN_LOOP: usize = 8;

impl TinyMT64 {
  pub fn new(status: [u64; 2], mat1: u32, mat2: u32, tmat: u64) -> TinyMT64 {
    TinyMT64 { status, mat1, mat2, tmat }
  }
}

/// This function represents a function used in the initialization by init_by_array.
#[inline]
fn ini_func1(x: u64) -> u64 {
  (x ^ (x >> 59)).wrapping_mul(2_173_292_883_993_u64)
}

/// This function represents a function used in the initialization by init_by_array
#[inline]
fn ini_func2(x: u64) -> u64 {
  (x ^ (x >> 59)).wrapping_mul(58_885_565_329_898_161_u64)
}

/// This function certificate the period of 2^127-1.
#[inline]
fn period_certification(random: &mut TinyMT64) {
  if random.status[0] & TINYMT64_MASK == 0 && random.status[1] == 0 {
    random.status[0] = 'T' as u64;
    random.status[1] = 'M' as u64;
  }
}

/// This function initializes the internal state array with a 64-bit unsigned integer seed.
/// @param seed a 64-bit unsigned integer used as a seed.
pub fn tinymt64_init(random: &mut TinyMT64, seed: u64) {
  random.status[0] = seed ^ ((random.mat1 as u64) << 32);
  random.status[1] = (random.mat2 as u64) ^ (random.tmat as u64);
  for i in 1..MIN_LOOP {
    random.status[i & 1] ^= (i as u64).wrapping_add(
      6_364_136_223_846_793_005_u64
        .wrapping_mul(random.status[(i - 1) & 1] ^ (random.status[(i - 1) & 1] >> 62)),
    );
  }
  period_certification(random);
}

/// This function initializes the internal state array, with an array of 64-bit unsigned integers used as seeds
/// @param init_key the array of 64-bit integers, used as a seed.
/// @param key_length the length of init_key.
pub fn tinymt64_init_by_array(random: &mut TinyMT64, init_key: &[u64]) {
  let lag: usize = 1;
  let mid: usize = 1;
  let size: usize = 4;
  let key_length: usize = init_key.len();

  let mut st: [u64; 4] = [0, random.mat1 as u64, random.mat2 as u64, random.tmat];
  let mut count: usize = if key_length + 1 > MIN_LOOP { key_length + 1 } else { MIN_LOOP };
  let mut r: u64 = ini_func1(st[0] ^ st[mid % size] ^ st[(size - 1) % size]);
  st[mid % size] += r;
  r += key_length as u64;
  st[(mid + lag) % size] += r;
  st[0] = r;
  count -= 1;
  let mut i = 1;
  let boundary = min(count, key_length);
  for key in init_key.iter().take(boundary) {
    r = ini_func1(st[i] ^ st[(i + mid) % size] ^ st[(i + size - 1) % size]);
    st[(i + mid) % size] = st[(i + mid) % size].wrapping_add(r);
    r += key + i as u64;
    st[(i + mid + lag) % size] = st[(i + mid + lag) % size].wrapping_add(r);
    st[i] = r;
    i = (i + 1) % size;
  }
  for _ in boundary..count {
    r = ini_func1(st[i] ^ st[(i + mid) % size] ^ st[(i + size - 1) % size]);
    st[(i + mid) % size] = st[(i + mid) % size].wrapping_add(r);
    r += i as u64;
    st[(i + mid + lag) % size] = st[(i + mid + lag) % size].wrapping_add(r);
    st[i] = r;
    i = (i + 1) % size;
  }
  for _ in 0..size {
    r = ini_func2(st[i].wrapping_add(st[(i + mid) % size]).wrapping_add(st[(i + size - 1) % size]));
    st[(i + mid) % size] ^= r;
    r -= i as u64;
    st[(i + mid + lag) % size] ^= r;
    st[i] = r;
    i = (i + 1) % size;
  }
  random.status[0] = st[0] ^ st[1];
  random.status[1] = st[2] ^ st[3];
  period_certification(random);
}

/// This function always returns 127.
#[inline]
pub fn tinymt64_get_mexp(_: &TinyMT64) -> usize {
  TINYMT64_MEXP
}

/**
 * This function changes internal state of tinymt64. Users should not call this function directly.
 * @param random tinymt internal status
 */
#[inline]
pub fn tinymt64_next_state(random: &mut TinyMT64) {
  random.status[0] &= TINYMT64_MASK;
  let mut x: u64 = random.status[0] ^ random.status[1];
  x ^= x << TINYMT64_SH0;
  x ^= x >> 32;
  x ^= x << 32;
  x ^= x << TINYMT64_SH1;
  random.status[0] = random.status[1];
  random.status[1] = x;
  random.status[0] ^= (-((x & 1) as i64) as u64) & (random.mat1 as u64);
  random.status[1] ^= (-((x & 1) as i64) as u64) & ((random.mat2 as u64) << 32);
}

/// This function outputs 64-bit unsigned integer from internal state. Users should not call this function directly.
/// @return 64-bit unsigned pseudorandom number
#[inline]
pub fn tinymt64_temper(random: &TinyMT64) -> u64 {
  // defined(LINEARITY_CHECK)
  // x = random->status[0] ^ random->status[1];
  let mut x = random.status[0].wrapping_add(random.status[1]);
  x ^= random.status[0] >> TINYMT64_SH8;
  x ^ (-((x & 1) as i64) as u64) & random.tmat
}

/// This function outputs floating point number from internal state. Users should not call this function directly.
/// @return floating point number r (1.0 <= r < 2.0)
#[inline]
pub fn tinymt64_temper_conv(random: &TinyMT64) -> f64 {
  // defined(LINEARITY_CHECK)
  // x = random->status[0] ^ random->status[1];
  let mut x = random.status[0].wrapping_add(random.status[1]);
  x ^= random.status[0] >> TINYMT64_SH8;
  x = ((x ^ ((-((x & 1) as i64) as u64) & random.tmat)) >> 12) | 0x3ff0_0000_0000_0000_u64;
  f64::from_le_bytes(x.to_le_bytes())
}

/// This function outputs floating point number from internal state. Users should not call this function directly.
/// @return floating point number r (1.0 < r < 2.0)
#[inline]
pub fn tinymt64_temper_conv_open(random: &TinyMT64) -> f64 {
  // defined(LINEARITY_CHECK)
  // x = random->status[0] ^ random->status[1];
  let mut x = random.status[0].wrapping_add(random.status[1]);
  x ^= random.status[0] >> TINYMT64_SH8;
  x = ((x ^ ((-((x & 1) as i64) as u64) & random.tmat)) >> 12) | 0x3ff0_0000_0000_0001_u64;
  f64::from_le_bytes(x.to_le_bytes())
}

/// This function outputs 64-bit unsigned integer from internal state.
/// @return 64-bit unsigned integer r (0 <= r < 2^64)
#[inline]
pub fn tinymt64_generate_uint64(random: &mut TinyMT64) -> u64 {
  tinymt64_next_state(random);
  tinymt64_temper(random)
}

/// This function outputs floating point number from internal state. This function is implemented
/// using multiplying by (1 / 2^53).
/// @return floating point number r (0.0 <= r < 1.0)
#[inline]
pub fn tinymt64_generate_double(random: &mut TinyMT64) -> f64 {
  tinymt64_next_state(random);
  ((tinymt64_temper(random) >> 11) as f64) * TINYMT64_MUL
}

/// This function outputs floating point number from internal state. This function is implemented
/// using union trick.
/// @return floating point number r (0.0 <= r < 1.0)
#[inline]
pub fn tinymt64_generate_double01(random: &mut TinyMT64) -> f64 {
  tinymt64_next_state(random);
  tinymt64_temper_conv(random) - 1.0
}

/// This function outputs floating point number from internal state. This function is implemented
/// using union trick.
/// @return floating point number r (1.0 <= r < 2.0)
#[inline]
pub fn tinymt64_generate_double12(random: &mut TinyMT64) -> f64 {
  tinymt64_next_state(random);
  tinymt64_temper_conv(random)
}

/// This function outputs floating point number from internal state. This function is implemented
/// using union trick.
/// @return floating point number r (0.0 < r <= 1.0)
#[inline]
pub fn tinymt64_generate_double_oc(random: &mut TinyMT64) -> f64 {
  tinymt64_next_state(random);
  2.0 - tinymt64_temper_conv(random)
}

/// This function outputs floating point number from internal state. This function is implemented
/// using union trick.
/// @return floating point number r (0.0 < r < 1.0)
pub fn tinymt64_generate_double_oo(random: &mut TinyMT64) -> f64 {
  tinymt64_next_state(random);
  tinymt64_temper_conv_open(random) - 1.0
}
