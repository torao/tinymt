extern crate clap;
extern crate rand;
extern crate tinymt;

use clap::{App, Arg};
use rand::{Rng, RngCore, SeedableRng};
use std::process;
use std::str::FromStr;
use tinymt::{TinyMT32, TinyMT64};

/// Parse the specified string to integer.
fn num<F: FromStr>(s: &str) -> Result<F, String> where <F as std::str::FromStr>::Err: std::fmt::Display {
  FromStr::from_str(s).map_err(|e: F::Err| { format!("ERROR: {}: {}", e, s) })
}

/// Generate TinyMT32 with specified seed.
fn rand32(seed: Option<&str>) -> Result<TinyMT32, String> {
  Ok(if let Some(seed) = seed {
    let seed: u32 = num(seed)?;
    TinyMT32::from_seed_u32(seed)
  } else {
    TinyMT32::from_entropy()
  })
}

/// Generate TinyMT64 with specified seed.
fn rand64(seed: Option<&str>) -> Result<TinyMT64, String> {
  Ok(if let Some(seed) = seed {
    let seed: u64 = num(seed)?;
    TinyMT64::from_seed_u64(seed)
  } else {
    TinyMT64::from_entropy()
  })
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
  let n: usize = num(args.value_of("count").unwrap())?;
  match args.value_of("type").unwrap() {
    "int" | "int32" | "i32" | "u32" | "32" =>
      print(&mut rand32(seed)?, n, |r| { format!("{}", r.next_u32()) }),
    "long" | "int64" | "i64" | "u64" | "64" =>
      print(&mut rand64(seed)?, n, |r| { format!("{}", r.next_u64()) }),
    "float" | "float32" | "f32" =>
      print(&mut rand32(seed)?, n, |r| { format!("{}", r.gen_range(0f32, 1f32)) }),
    "double" | "float64" | "f64" =>
      print(&mut rand64(seed)?, n, |r| { format!("{}", r.gen_range(0f64, 1f64)) }),
    "string" | "str" | "s" => {
      let mut r = rand64(seed)?;
      let length: usize = num(args.value_of("length").unwrap())?;
      let radix: Vec<char> = args.value_of("radix").unwrap().chars().collect();
      for _ in 0..n {
        println!("{}", (0..length).map(|_| { radix[r.gen_range(0, radix.len())] }).collect::<String>());
      }
    }
    unknown =>
      return Err(format!("ERROR: the specified type is not defined: {}", unknown))
  };
  Ok(())
}

fn print<T>(rng: &mut T, n: usize, next_random: fn(&mut T) -> String) {
  for _ in 0..n {
    println!("{}", next_random(rng));
  }
}

fn error(message: String) -> ! {
  eprintln!("{}", message);
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
      .help("generate random number of int, long, float, double or string type")
    )
    .arg(Arg::with_name("length")
      .short("l")
      .long("length")
      .takes_value(true)
      .default_value("32")
      .help("the number of characters when generating random strings")
    )
    .arg(Arg::with_name("radix")
      .short("r")
      .long("radix")
      .takes_value(true)
      .default_value("0123456789abcdefghijklmnopqrstuvwxyz")
      .help("characters to be used when generating random string")
    )
    .arg(Arg::with_name("seed")
      .help("seed of pseudo-random number generator")
    )
}