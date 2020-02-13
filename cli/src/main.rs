extern crate clap;
extern crate rand;
extern crate tinymt;

use clap::{App, Arg};
use rand::{Rng, RngCore, SeedableRng};
use tinymt::{TinyMT32, TinyMT32Seed, TinyMT64, TinyMT64Seed};

fn main() {
  let app = App::new("tinymt")
    .version("1.0.0")
    .author("Torao Takami <koiroha@mail.com>")
    .about("TinyMT 64/32-bit Random Number Generator CLI")
    .arg(Arg::with_name("count")
      .short("n")
      .takes_value(true)
      .default_value("1")
      .help("number to generate")
    )
    .arg(Arg::with_name("type")
      .short("t")
      .long("type")
      .takes_value(true)
      .default_value("double")
      .help("generate random number of int, long, float or double type")
    )
    .arg(Arg::with_name("seed")
      .help("seed of pseudo-random number generator")
    );

  let args = app.get_matches();
  let style = args.value_of("type").unwrap();
  let bit32 = style == "int" || style == "float";
  let n = args.value_of("count").unwrap().parse().unwrap();
  let mut rng: Box<dyn RngCore> = if let Some(seed) = args.value_of("seed") {
    if bit32 {
      let seed: u32 = seed.parse().unwrap();
      Box::new(TinyMT32::from_seed(TinyMT32Seed::from(seed)))
    } else {
      let seed: u64 = seed.parse().unwrap();
      Box::new(TinyMT64::from_seed(TinyMT64Seed::from(seed)))
    }
  } else if bit32 {
    Box::new(TinyMT32::from_entropy())
  } else {
    Box::new(TinyMT64::from_entropy())
  };
  for _ in 0..n {
    match style {
      "int" => println!("{}", rng.next_u32()),
      "long" => println!("{}", rng.next_u64()),
      "float" => println!("{}", rng.gen_range(0f32, 1f32)),
      "double" => println!("{}", rng.gen_range(0f64, 1f64)),
      _ => panic!(format!("undefined type: {}", style)),
    }
  }
}
