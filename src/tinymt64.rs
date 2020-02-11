use std::cmp::min;

use crate::TinyMT64;

const TINYMT64_MEXP: usize = 127;
const TINYMT64_SH0: u64 = 12;
const TINYMT64_SH1: u64 = 11;
const TINYMT64_SH8: u64 = 8;
const TINYMT64_MASK: u64 = 0x7fff_ffff_ffff_ffff_u64;
const TINYMT64_MUL: f64 = (1.0 / 9_007_199_254_740_992.0);
const MIN_LOOP: usize = 8;

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
      6_364_136_223_846_793_005_u64.wrapping_mul(
        random.status[(i - 1) & 1] ^ (random.status[(i - 1) & 1] >> 62)
      )
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
  let mut count: usize = if key_length + 1 > MIN_LOOP {
    key_length + 1
  } else {
    MIN_LOOP
  };
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

#[cfg(test)]
mod test {
  use super::*;

  /// https://github.com/MersenneTwister-Lab/TinyMT/blob/master/tinymt/check64.c
  #[test]
  fn test_cases_by_author() {
    // tinymt64 0xfa051f40 0xffd0fff4 0x58d02ffeffbfffbc seed = 1
    let mut tinymt = TinyMT64 {
      status: [0, 0],
      mat1: 0xfa051f40,
      mat2: 0xffd0fff4,
      tmat: 0x58d02ffeffbfffbc,
    };
    let seed = 1;
    tinymt64_init(&mut tinymt, seed);

    test_generate_uint64(&mut tinymt);

    let mut tinymt = TinyMT64 {
      status: [0, 0],
      mat1: 0xfa051f40,
      mat2: 0xffd0fff4,
      tmat: 0x58d02ffeffbfffbc,
    };

    // init_by_array {1}
    let seed_array: [u64; 1] = [1];
    tinymt64_init_by_array(&mut tinymt, &seed_array);

    test_generate_double(&mut tinymt);
    test_generate_double12(&mut tinymt);
    test_generate_double_oc(&mut tinymt);
    test_generate_double_oo(&mut tinymt);
  }

  /// 64-bit unsigned integers r, where 0 <= r < 2^64
  fn test_generate_uint64(tinymt: &mut TinyMT64) {
    let expected = [
      [15503804787016557143, 17280942441431881838, 2177846447079362065],
      [10087979609567186558, 8925138365609588954, 13030236470185662861],
      [4821755207395923002, 11414418928600017220, 18168456707151075513],
      [1749899882787913913, 2383809859898491614, 4819668342796295952],
      [11996915412652201592, 11312565842793520524, 995000466268691999],
      [6363016470553061398, 7460106683467501926, 981478760989475592],
      [11852898451934348777, 5976355772385089998, 16662491692959689977],
      [4997134580858653476, 11142084553658001518, 12405136656253403414],
      [10700258834832712655, 13440132573874649640, 15190104899818839732],
      [14179849157427519166, 10328306841423370385, 9266343271776906817],
    ];
    for i in 0..10 {
      for j in 0..3 {
        assert_eq!(expected[i][j], tinymt64_generate_uint64(tinymt));
      }
    }
  }

  /// double numbers r, where 0.0 <= r < 1.0
  fn test_generate_double(tinymt: &mut TinyMT64) {
    let expected = [
      [0.1255671232295209, 0.8182624006077499, 0.30822110203281683, 0.8255918229908551],
      [0.2555517877036223, 0.8826415608914364, 0.21152361685256493, 0.31910695814713397],
      [0.8731938455315581, 0.7563442179617009, 0.9626867074958626, 0.12749130989590807],
      [0.6701740931329137, 0.09321519841996262, 0.4752574502959318, 0.4260656296146129],
      [0.8342582498085203, 0.2790339713179786, 0.5149478695059739, 0.036174029320189205],
      [0.8897244223874985, 0.2517822059311857, 0.38744954297723777, 0.08612504349103156],
      [0.7653487864751842, 0.5710556579917725, 0.3222522710904576, 0.02403074050158116],
      [0.08852883324737215, 0.9634180500801006, 0.45325188731530386, 0.9794639730992276],
      [0.5959355146678814, 0.5271370962042583, 0.4095847647067309, 0.004006194821549625],
      [0.20031375905173576, 0.4542823501864448, 0.713777124755221, 0.8079152811762041],
      [0.12756425652308678, 0.9863577232725115, 0.5371878217659848, 0.23544098948491765],
      [0.7000063536628773, 0.8580450245315596, 0.13056501891409378, 0.17396590319565852],
    ];
    for i in 0..12 {
      for j in 0..4 {
        assert_eq!(expected[i][j], tinymt64_generate_double(tinymt));
      }
    }
  }

  /// double numbers r, where 1.0 <= r < 2.0
  fn test_generate_double12(tinymt: &mut TinyMT64) {
    let expected = [
      [1.437679237017648, 1.239536785901373, 1.140298949383057, 1.776408301859232],
      [1.152013609994736, 1.791233026870471, 1.212111221146196, 1.829985488180836],
      [1.081512125943717, 1.363201836673650, 1.417933283495315, 1.814826826183523],
      [1.969922345279833, 1.053208264502199, 1.741205427976973, 1.837349090361589],
      [1.406622310957582, 1.510698317360325, 1.829965206684917, 1.859153888163104],
      [1.759271635641173, 1.824888617384633, 1.237637472413003, 1.367109059723164],
      [1.976389381199251, 1.989991431835970, 1.044503045383735, 1.769751873156083],
      [1.859046544898330, 1.218170930629463, 1.308291384260259, 1.694324347868131],
      [1.458264916022492, 1.128833025697983, 1.205547655611532, 1.909188848740936],
      [1.562083063485982, 1.333329220907858, 1.665680038183793, 1.001161742007127],
      [1.667546697634258, 1.296057871298311, 1.461095987795535, 1.459580681054313],
      [1.556093077958318, 1.916051394545249, 1.267046730316243, 1.147033584842960],
    ];
    for i in 0..12 {
      for j in 0..4 {
        let actual = tinymt64_generate_double12(tinymt);
        assert_eq!(format!("{:.15}", expected[i][j]), format!("{:.15}", actual));
      }
    }
  }

  /// double numbers r, where 0.0 < r <= 1.0
  fn test_generate_double_oc(tinymt: &mut TinyMT64) {
    let expected = [
      [0.231189305675805, 0.800078680337062, 0.839012626265816, 0.439830027924101],
      [0.287094637016178, 0.588065859945908, 0.979935435454641, 0.153150392249384],
      [0.730008781559804, 0.811897304025850, 0.213940001686070, 0.803418052576349],
      [0.872859727831960, 0.620437548528132, 0.978150212926246, 0.101173021131322],
      [0.910550586203282, 0.225948191636215, 0.374316722183833, 0.305990832583114],
      [0.349868211955804, 0.617675089001072, 0.977990275060935, 0.453879799720974],
      [0.894692817941832, 0.247166853705171, 0.639918430646982, 0.187528433375713],
      [0.098449225468909, 0.630303237374302, 0.080830809996716, 0.578706622599148],
      [0.734493648961771, 0.082578413837076, 0.287252902600609, 0.148892860351310],
      [0.032585155152626, 0.974545363240716, 0.115296495734384, 0.779122282518212],
      [0.217374280466345, 0.109080092606533, 0.926570354675966, 0.740322917071261],
      [0.751484666825263, 0.287404891102534, 0.652825666825707, 0.715421981731271],
    ];
    for i in 0..12 {
      for j in 0..4 {
        let actual = tinymt64_generate_double_oc(tinymt);
        assert_eq!(format!("{:.15}", expected[i][j]), format!("{:.15}", actual));
      }
    }
  }

  /// double numbers r, where 0.0 <= r < 1.0
  fn test_generate_double_oo(tinymt: &mut TinyMT64) {
    let expected = [
      [0.777528512172794, 0.427164705471249, 0.646272649811224, 0.544192399276788],
      [0.055887665337890, 0.341700406526459, 0.711593276934271, 0.834779506059920],
      [0.300433789431423, 0.551665825454914, 0.408923581859956, 0.087661367648074],
      [0.778332952183721, 0.987791992333503, 0.372489630499866, 0.814160794049290],
      [0.729786510846933, 0.759103094898038, 0.126259258963777, 0.126254138650957],
      [0.959804243067140, 0.297513369134440, 0.080559577448319, 0.759676389010768],
      [0.299289034712620, 0.930880432389953, 0.453686608702271, 0.051097960521366],
      [0.190779343357538, 0.668131291883840, 0.568395939590751, 0.051087427533098],
      [0.531001654494900, 0.863620810621520, 0.713312386789292, 0.079647214591833],
      [0.503865458699875, 0.649652561747019, 0.651303365948799, 0.423447062299514],
      [0.383204776309473, 0.325776002558352, 0.917474566663556, 0.612145094607316],
      [0.785176513330525, 0.046217567933767, 0.445326162565375, 0.426594677789800],
    ];
    for i in 0..12 {
      for j in 0..4 {
        let actual = tinymt64_generate_double_oo(tinymt);
        assert_eq!(format!("{:.15}", expected[i][j]), format!("{:.15}", actual));
      }
    }
  }
}