use clap::{Arg, Command};
use rand::{Rng, RngCore, SeedableRng};
use std::process;
use std::str::FromStr;
use tinymt::{TinyMT32, TinyMT64};

/// Parse the specified string to integer.
fn num<F: FromStr>(s: &str) -> Result<F, String>
where
  <F as std::str::FromStr>::Err: std::fmt::Display,
{
  FromStr::from_str(s).map_err(|e: F::Err| format!("ERROR: {}: {}", e, s))
}

/// Generate TinyMT32 with specified seed.
fn rand32(seed: Option<&String>) -> Result<TinyMT32, String> {
  Ok(if let Some(seed) = seed {
    let seed: u32 = num(seed)?;
    TinyMT32::from_seed_u32(seed)
  } else {
    TinyMT32::from_entropy()
  })
}

/// Generate TinyMT64 with specified seed.
fn rand64(seed: Option<&String>) -> Result<TinyMT64, String> {
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
    Err(msg) => error(msg),
  }
}

fn exec() -> Result<(), String> {
  let args = arguments().get_matches();
  let seed = args.get_one::<String>("seed");
  let n = num(args.get_one::<String>("count").unwrap())?;
  match args.get_one::<String>("type").unwrap().as_str() {
    "int" | "int32" | "i32" | "u32" | "32" => {
      print(&mut rand32(seed)?, n, |r| format!("{}", r.next_u32()))
    }
    "long" | "int64" | "i64" | "u64" | "64" => {
      print(&mut rand64(seed)?, n, |r| format!("{}", r.next_u64()))
    }
    "float" | "float32" | "f32" => {
      print(&mut rand32(seed)?, n, |r| format!("{}", r.gen_range(0f32..1f32)))
    }
    "double" | "float64" | "f64" => {
      print(&mut rand64(seed)?, n, |r| format!("{}", r.gen_range(0f64..1f64)))
    }
    "string" | "str" | "s" => {
      let mut r = rand64(seed)?;
      let length: usize = num(args.get_one::<String>("length").unwrap())?;
      let radix: Vec<char> = args.get_one::<String>("radix").unwrap().chars().collect();
      for _ in 0..n {
        println!(
          "{}",
          (0..length).map(|_| { radix[r.gen_range(0..radix.len())] }).collect::<String>()
        );
      }
    }
    unknown => return Err(format!("ERROR: the specified type is not defined: {}", unknown)),
  };
  Ok(())
}

fn print<T: Rng>(rng: &mut T, n: usize, next_random: fn(&mut T) -> String) {
  for _ in 0..n {
    println!("{}", next_random(rng));
  }
}

fn error(message: String) -> ! {
  eprintln!("{}", message);
  process::exit(1)
}

fn arguments() -> Command {
  Command::new("tinymt")
    .version("1.0")
    .author("Torao Takami <koiroha@mail.com>")
    .about("TinyMT 64/32-bit Random Number Generator CLI")
    .arg(
      Arg::new("count")
        .short('n')
        .number_of_values(1)
        .default_value("1")
        .help("number to generate"),
    )
    .arg(
      Arg::new("type")
        .short('t')
        .long("type")
        .number_of_values(1)
        .default_value("double")
        .help("generate random number of int, long, float, double or string type"),
    )
    .arg(
      Arg::new("length")
        .short('l')
        .long("length")
        .number_of_values(1)
        .default_value("32")
        .help("the number of characters when generating random strings"),
    )
    .arg(
      Arg::new("radix")
        .short('r')
        .long("radix")
        .number_of_values(1)
        .default_value("0123456789abcdefghijklmnopqrstuvwxyz")
        .help("characters to be used when generating random string"),
    )
    .arg(Arg::new("seed").help("seed of pseudo-random number generator"))
}
