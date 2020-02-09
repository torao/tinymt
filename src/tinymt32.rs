use std::cmp::min;

use crate::TinyMT32;

const TINYMT32_MEXP: usize = 127;
const TINYMT32_SH0: u32 = 1;
const TINYMT32_SH1: u32 = 10;
const TINYMT32_SH8: u32 = 8;
const TINYMT32_MASK: u32 = 0x7fff_ffff_u32;
const TINYMT32_MUL: f64 = (1.0f64 / 16_777_216.0_f64);
const MIN_LOOP: usize = 8;
const PRE_LOOP: usize = 8;

/// This function represents a function used in the initialization by init_by_array
fn ini_func1(x: u32) -> u32 {
  (x ^ (x >> 27)).wrapping_mul(1_664_525_u32)
}

/// This function represents a function used in the initialization by init_by_array
fn ini_func2(x: u32) -> u32 {
  (x ^ (x >> 27)).wrapping_mul(1_566_083_941_u32)
}

/// This function certificate the period of 2^127-1.
/// @param random tinymt state vector.
fn period_certification(random: &mut TinyMT32) {
  if random.status[0] & TINYMT32_MASK == 0 && random.status[1] == 0 && random.status[2] == 0 && random.status[3] == 0 {
    random.status[0] = 'T' as u32;
    random.status[1] = 'I' as u32;
    random.status[2] = 'N' as u32;
    random.status[3] = 'Y' as u32;
  }
}

/// This function initializes the internal state array with a 32-bit unsigned integer seed.
/// @param random tinymt state vector.
/// @param seed a 32-bit unsigned integer used as a seed.
pub fn tinymt32_init(random: &mut TinyMT32, seed: u32) {
  random.status[0] = seed;
  random.status[1] = random.mat1;
  random.status[2] = random.mat2;
  random.status[3] = random.tmat;
  for i in 1..MIN_LOOP {
    random.status[i & 3] ^= (i as u32).wrapping_add(
      1_812_433_253_u32.wrapping_mul(random.status[(i - 1) & 3] ^ (random.status[(i - 1) & 3] >> 30))
    );
  }
  period_certification(random);
  for _ in 0..PRE_LOOP {
    tinymt32_next_state(random);
  }
}

/// This function initializes the internal state array, with an array of 32-bit unsigned integers used as seeds
/// @param init_key the array of 32-bit integers, used as a seed.
/// @param key_length the length of init_key.
pub fn tinymt32_init_by_array(random: &mut TinyMT32, init_key: &[u32]) {
  let key_length: usize = init_key.len();
  let lag: usize = 1;
  let mid: usize = 1;
  let size: usize = 4;

  let st: &mut [u32; 4] = &mut random.status;
  st[0] = 0;
  st[1] = random.mat1;
  st[2] = random.mat2;
  st[3] = random.tmat;
  let mut count: usize = if key_length + 1 > MIN_LOOP {
    key_length + 1
  } else {
    MIN_LOOP
  };
  let mut r: u32 = ini_func1(st[0] ^ st[mid % size] ^ st[(size - 1) % size]);
  st[mid % size] = st[mid % size].wrapping_add(r);
  r += key_length as u32;
  st[(mid + lag) % size] = st[(mid + lag) % size].wrapping_add(r);
  st[0] = r;
  count -= 1;
  let mut i: usize = 1;
  for j in 0..min(count, key_length) {
    r = ini_func1(st[i % size] ^ st[(i + mid) % size] ^ st[(i + size - 1) % size]);
    st[(i + mid) % size] = st[(i + mid) % size].wrapping_add(r);
    r += init_key[j] + i as u32;
    st[(i + mid + lag) % size] = st[(i + mid + lag) % size].wrapping_add(r);
    st[i % size] = r;
    i = (i + 1) % size;
  }
  for _ in min(count, key_length)..count {
    r = ini_func1(st[i % size] ^ st[(i + mid) % size] ^ st[(i + size - 1) % size]);
    st[(i + mid) % size] = st[(i + mid) % size].wrapping_add(r);
    r += i as u32;
    st[(i + mid + lag) % size] = st[(i + mid + lag) % size].wrapping_add(r);
    st[i % size] = r;
    i = (i + 1) % size;
  }
  for _ in 0..size {
    r = ini_func2(st[i % size].wrapping_add(st[(i + mid) % size]).wrapping_add(st[(i + size - 1) % size]));
    st[(i + mid) % size] ^= r;
    r -= i as u32;
    st[(i + mid + lag) % size] ^= r;
    st[i % size] = r;
    i = (i + 1) % size;
  }
  period_certification(random);
  for _ in 0..PRE_LOOP {
    tinymt32_next_state(random);
  }
}

/// This function always returns 127
/// @return always 127
#[inline]
pub fn tinymt32_get_mexp(_: &TinyMT32) -> usize {
  TINYMT32_MEXP
}

/// This function changes internal state of tinymt32. Users should not call this function directly.
/// @param random tinymt internal status
#[inline]
pub fn tinymt32_next_state(random: &mut TinyMT32) {
  let mut y: u32 = random.status[3];
  let mut x: u32 = (random.status[0] & TINYMT32_MASK) ^ random.status[1] ^ random.status[2];
  x ^= x << TINYMT32_SH0;
  y ^= (y >> TINYMT32_SH0) ^ x;
  random.status[0] = random.status[1];
  random.status[1] = random.status[2];
  random.status[2] = x ^ (y << TINYMT32_SH1);
  random.status[3] = y;
  random.status[1] ^= (-((y & 1) as i32) as u32) & random.mat1;
  random.status[2] ^= (-((y & 1) as i32) as u32) & random.mat2;
}

/// This function outputs 32-bit unsigned integer from internal state. Users should not call this function directly.
/// @param random tinymt internal status
/// @return 32-bit unsigned pseudorandom number
#[inline]
pub fn tinymt32_temper(random: &mut TinyMT32) -> u32 {
  let mut t0: u32 = random.status[3];
  // defined(LINEARITY_CHECK)
  // t1 = random->status[0]^ (random->status[2] >> TINYMT32_SH8);
  let t1: u32 = random.status[0].wrapping_add(random.status[2] >> TINYMT32_SH8);
  t0 ^= t1;
  t0 ^ (-((t1 & 1) as i32) as u32) & random.tmat
}

/// This function outputs floating point number from internal state. Users should not call this function directly.
/// @param random tinymt internal status
/// @return floating point number r (1.0 <= r < 2.0)
#[inline]
pub fn tinymt32_temper_conv(random: &mut TinyMT32) -> f32 {
  let mut t0: u32 = random.status[3];
  // defined(LINEARITY_CHECK)
  // t1 = random->status[0]^ (random->status[2] >> TINYMT32_SH8);
  let t1: u32 = random.status[0].wrapping_add(random.status[2] >> TINYMT32_SH8);
  t0 ^= t1;
  let u: u32 = ((t0 ^ ((-((t1 & 1) as i32) as u32) & random.tmat)) >> 9) | 0x3f80_0000_u32;
  f32::from_le_bytes(u.to_le_bytes())
}

/// This function outputs floating point number from internal state. Users should not call this function directly.
/// @return floating point number r (1.0 < r < 2.0)
#[inline]
pub fn tinymt32_temper_conv_open(random: &mut TinyMT32) -> f32 {
  let mut t0: u32 = random.status[3];
  // defined(LINEARITY_CHECK)
  // t1 = random->status[0] ^ (random->status[2] >> TINYMT32_SH8);
  let t1: u32 = random.status[0].wrapping_add(random.status[2] >> TINYMT32_SH8);
  t0 ^= t1;
  let u: u32 = ((t0 ^ ((-((t1 & 1) as i32) as u32) & random.tmat)) >> 9) | 0x3f80_0001_u32;
  f32::from_le_bytes(u.to_le_bytes())
}

/// This function outputs 32-bit unsigned integer from internal state.
/// @return 32-bit unsigned integer r (0 <= r < 2^32)
#[inline]
pub fn tinymt32_generate_uint32(random: &mut TinyMT32) -> u32 {
  tinymt32_next_state(random);
  tinymt32_temper(random)
}

/// This function outputs floating point number from internal state. This function is implemented using multiplying by (1 / 2^24). floating point multiplication is faster than using union trick in my Intel CPU.
/// @return floating point number r (0.0 <= r < 1.0)
#[inline]
pub fn tinymt32_generate_float(random: &mut TinyMT32) -> f32 {
  tinymt32_next_state(random);
  ((tinymt32_temper(random) >> 8) as f64 * TINYMT32_MUL) as f32
}

/// This function outputs floating point number from internal state. This function is implemented using union trick.
/// @return floating point number r (1.0 <= r < 2.0)
#[inline]
pub fn tinymt32_generate_float12(random: &mut TinyMT32) -> f32 {
  tinymt32_next_state(random);
  tinymt32_temper_conv(random)
}

/// This function outputs floating point number from internal state.
/// This function is implemented using union trick.
/// @return floating point number r (0.0 <= r < 1.0)
#[inline]
pub fn tinymt32_generate_float01(random: &mut TinyMT32) -> f32 {
  tinymt32_next_state(random);
  tinymt32_temper_conv(random) - 1.0f32
}

/// This function outputs floating point number from internal state. This function may return 1.0 and never returns 0.0.
/// @return floating point number r (0.0 < r <= 1.0)
#[inline]
pub fn tinymt32_generate_float_oc(random: &mut TinyMT32) -> f32 {
  tinymt32_next_state(random);
  1.0f32 - tinymt32_generate_float(random)
}

/// This function outputs floating point number from internal state. This function returns neither 0.0 nor 1.0.
/// @return floating point number r (0.0 < r < 1.0)
#[inline]
pub fn tinymt32_generate_float_oo(random: &mut TinyMT32) -> f32 {
  tinymt32_next_state(random);
  tinymt32_temper_conv_open(random) - 1.0f32
}

/// This function outputs double precision floating point number from internal state. The returned value has 32-bit precision.  In other words, this function makes one double precision floating point number from one 32-bit unsigned integer.
/// @return floating point number r (0.0 <= r < 1.0)
#[inline]
pub fn tinymt32_generate_32double(random: &mut TinyMT32) -> f64 {
  tinymt32_next_state(random);
  tinymt32_temper(random) as f64 * (1.0f64 / 4_294_967_296.0_f64)
}

#[cfg(test)]
mod test {
  use super::*;

  /// https://github.com/MersenneTwister-Lab/TinyMT/blob/master/tinymt/check32.c
  #[test]
  fn test_cases_by_author() {
    // tinymt32 0x8f7011ee 0xfc78ff1f 0x3793fdff seed = 1
    let mut tinymt = TinyMT32 {
      status: [0, 0, 0, 0],
      mat1: 0x8f7011ee,
      mat2: 0xfc78ff1f,
      tmat: 0x3793fdff,
    };
    let seed = 1;
    tinymt32_init(&mut tinymt, seed);

    test_generate_uint32(&mut tinymt);

    // init_by_array {1}
    let seed_array: [u32; 1] = [1];
    tinymt32_init_by_array(&mut tinymt, &seed_array);

    test_generate_float(&mut tinymt);
    test_generate_float12(&mut tinymt);
    test_generate_float_oc(&mut tinymt);
    test_generate_float_oo(&mut tinymt);
    test_generate_32double(&mut tinymt);
  }

  /// 32-bit unsigned integers r, where 0 <= r < 2^32
  fn test_generate_uint32(tinymt: &mut TinyMT32) {
    let expected = [
      [2545341989, 981918433, 3715302833, 2387538352, 3591001365],
      [3820442102, 2114400566, 2196103051, 2783359912, 764534509],
      [643179475, 1822416315, 881558334, 4207026366, 3690273640],
      [3240535687, 2921447122, 3984931427, 4092394160, 44209675],
      [2188315343, 2908663843, 1834519336, 3774670961, 3019990707],
      [4065554902, 1239765502, 4035716197, 3412127188, 552822483],
      [161364450, 353727785, 140085994, 149132008, 2547770827],
      [4064042525, 4078297538, 2057335507, 622384752, 2041665899],
      [2193913817, 1080849512, 33160901, 662956935, 642999063],
      [3384709977, 1723175122, 3866752252, 521822317, 2292524454],
    ];
    for i in 0..10 {
      for j in 0..5 {
        assert_eq!(expected[i][j], tinymt32_generate_uint32(tinymt));
      }
    }
  }

  // float numbers r, where 0.0 <= r < 1.0
  fn test_generate_float(tinymt: &mut TinyMT32) {
    let expected = [
      [0.0132459, 0.2083899, 0.1457998, 0.1144078, 0.6173239],
      [0.0522397, 0.9873815, 0.1503184, 0.4039059, 0.6909348],
      [0.0908061, 0.0637298, 0.5002118, 0.1056944, 0.0936889],
      [0.0609041, 0.0725737, 0.7802556, 0.8761556, 0.5714422],
      [0.1706455, 0.4046335, 0.4131218, 0.2825145, 0.8249400],
      [0.4180385, 0.2152816, 0.4346161, 0.4916836, 0.5997444],
      [0.9118822, 0.1928336, 0.7523277, 0.9890286, 0.7421532],
      [0.9053972, 0.3542482, 0.9161059, 0.1209783, 0.8205475],
      [0.8592415, 0.8379903, 0.6638085, 0.8796422, 0.8608698],
      [0.9255103, 0.6475281, 0.7260162, 0.8757523, 0.0845953],
    ];
    for i in 0..10 {
      for j in 0..5 {
        let actual = tinymt32_generate_float(tinymt);
        assert_eq!(format!("{:.7}", expected[i][j]), format!("{:.7}", actual));
      }
    }
  }

  // float numbers r, where 1.0 <= r < 2.0
  fn test_generate_float12(tinymt: &mut TinyMT32) {
    let expected = [
      [1.6180767, 1.8378111, 1.7666160, 1.2583882, 1.6962934],
      [1.6468527, 1.8065972, 1.5554585, 1.4074975, 1.0875973],
      [1.9197918, 1.4574956, 1.6669209, 1.8137155, 1.3395888],
      [1.7431080, 1.0419986, 1.7254776, 1.8457749, 1.7100438],
      [1.9055752, 1.1819330, 1.8549275, 1.9305544, 1.1244931],
      [1.2847148, 1.8663290, 1.4107596, 1.1664802, 1.1365448],
      [1.4102769, 1.9013107, 1.9665589, 1.2195582, 1.7036947],
      [1.3244984, 1.3074670, 1.4314530, 1.3307399, 1.4553448],
      [1.2322005, 1.3248408, 1.6282554, 1.6237093, 1.9553823],
      [1.2515985, 1.2902025, 1.8261194, 1.7116343, 1.0828516],
    ];
    for i in 0..10 {
      for j in 0..5 {
        let actual = tinymt32_generate_float12(tinymt);
        assert_eq!(format!("{:.7}", expected[i][j]), format!("{:.7}", actual));
      }
    }
  }

  // float numbers r, where 0.0 < r <= 1.0
  fn test_generate_float_oc(tinymt: &mut TinyMT32) {
    let expected = [
      [0.4334422, 0.1254190, 0.9491148, 0.7561387, 0.5671672],
      [0.8243424, 0.0393196, 0.3985791, 0.4224766, 0.6121919],
      [0.5195524, 0.0341858, 0.3006201, 0.9415598, 0.1908746],
      [0.6455914, 0.9965364, 0.3110815, 0.4033393, 0.9034473],
      [0.0202459, 0.6251086, 0.2076811, 0.1991719, 0.0160369],
      [0.5703404, 0.8151199, 0.9348064, 0.9298607, 0.6834633],
      [0.2357914, 0.6382589, 0.0393693, 0.4783188, 0.4688579],
      [0.2675911, 0.4227387, 0.2752262, 0.7581965, 0.3906184],
      [0.6015150, 0.4173800, 0.2261215, 0.5006371, 0.2059622],
      [0.1784128, 0.4403929, 0.5902822, 0.2307619, 0.4184512],
    ];
    for i in 0..10 {
      for j in 0..5 {
        let actual = tinymt32_generate_float_oc(tinymt);
        assert_eq!(format!("{:.7}", expected[i][j]), format!("{:.7}", actual));
      }
    }
  }

  // float numbers r, where 0.0 < r < 1.0
  fn test_generate_float_oo(tinymt: &mut TinyMT32) {
    let expected = [
      [0.7539235, 0.5481223, 0.0172182, 0.3837644, 0.5756599],
      [0.1929101, 0.6351088, 0.1388987, 0.2030107, 0.5359520],
      [0.7981051, 0.8822426, 0.5865937, 0.9584194, 0.9073082],
      [0.6073984, 0.8127722, 0.7480494, 0.9829172, 0.6296896],
      [0.2040328, 0.0169488, 0.5349101, 0.7498616, 0.4206887],
      [0.4468912, 0.6781071, 0.5027536, 0.4000009, 0.2352458],
      [0.3784646, 0.8087858, 0.3579344, 0.6030601, 0.2197810],
      [0.9718446, 0.5287687, 0.7941138, 0.9504710, 0.3413824],
      [0.1003662, 0.8295220, 0.6224557, 0.9157780, 0.4195939],
      [0.2126821, 0.8094529, 0.1176151, 0.1643153, 0.2755433],
    ];
    for i in 0..10 {
      for j in 0..5 {
        let actual = tinymt32_generate_float_oo(tinymt);
        assert_eq!(format!("{:.7}", expected[i][j]), format!("{:.7}", actual));
      }
    }
  }

  // 32-bit precision double numbers r, where 0.0 <= r < 1.0
  fn test_generate_32double(tinymt: &mut TinyMT32) {
    let expected = [
      [0.4094066, 0.4827545, 0.5979867, 0.2170185, 0.8970369],
      [0.6829838, 0.3973019, 0.2750306, 0.1092794, 0.8370101],
      [0.6354089, 0.5781288, 0.7005250, 0.2332346, 0.8395586],
      [0.3070853, 0.3678428, 0.6112665, 0.9327284, 0.7354169],
      [0.5996112, 0.7402635, 0.1835579, 0.5796655, 0.9021798],
      [0.7528325, 0.0600313, 0.4967926, 0.1992569, 0.6572806],
      [0.9061203, 0.4460495, 0.3509606, 0.9238296, 0.5796654],
      [0.6424482, 0.9554131, 0.5053623, 0.9762550, 0.5047233],
      [0.6132142, 0.2425692, 0.9836005, 0.5532928, 0.4105124],
      [0.5009801, 0.8399252, 0.7654016, 0.6735107, 0.8542220],
    ];
    for i in 0..10 {
      for j in 0..5 {
        let actual = tinymt32_generate_32double(tinymt);
        assert_eq!(format!("{:.7}", expected[i][j]), format!("{:.7}", actual));
      }
    }
  }
}