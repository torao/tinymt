extern crate clap;
extern crate rand;
extern crate tinymt;

use clap::{App, Arg};
use rand::{Rng, RngCore, SeedableRng};
use std::num::ParseIntError;
use std::process;
use tinymt::{TinyMT32, TinyMT64};

fn rand32(seed: Option<&str>) -> TinyMT32 {
  if let Some(seed) = seed {
    let seed: u32 = seed.parse().unwrap();
    TinyMT32::from_seed_u32(seed)
  } else {
    TinyMT32::from_entropy()
  }
}

fn rand64(seed: Option<&str>) -> TinyMT64 {
  if let Some(seed) = seed {
    let seed: u64 = seed.parse().unwrap();
    TinyMT64::from_seed_u64(seed)
  } else {
    TinyMT64::from_entropy()
  }
}

fn main() {
  match exec() {
    Ok(_) => (),
    Err(msg) => error(msg)
  }
}

fn exec() -> Result<(), String> {
  let args = arguments().get_matches();
  let seed = args.value_of("seed");
  let n = args.value_of("count").unwrap().parse()
    .map_err(|e: ParseIntError| { format!("ERROR: {}", e.to_string()) })?;
  match args.value_of("type").unwrap() {
    "int" | "int32" | "i32" | "u32" | "32" =>
      print(&mut rand32(seed), n, |r| { format!("{}", r.next_u32()) }),
    "long" | "int64" | "i64" | "u64" | "64" =>
      print(&mut rand64(seed), n, |r| { format!("{}", r.next_u64()) }),
    "float" | "float32" | "f32" =>
      print(&mut rand32(seed), n, |r| { format!("{}", r.gen_range(0f32, 1f32)) }),
    "double" | "float64" | "f64" =>
      print(&mut rand64(seed), n, |r| { format!("{}", r.gen_range(0f64, 1f64)) }),
    unknown =>
      return Err(format!("ERROR: the specified type is not defined: {}", unknown))
  };
  Ok(())
}

fn print<T>(rng: &mut T, n: usize, rand: fn(&mut T) -> String) {
  for _ in 0..n {
    println!("{}", rand(rng));
  }
}

fn error(message: String) -> ! {
  eprintln!("{}", message);
  arguments().print_help().unwrap();
  process::exit(1)
}

fn arguments() -> App<'static, 'static> {
  App::new("tinymt")
    .version("1.0")
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
    )
}