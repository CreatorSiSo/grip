use std::io::BufReader;
use std::{fs::File, io::BufRead};

use clap::Parser;

#[derive(Debug, Parser)]
struct Arguments {
  // #[clap(short = 'f', long = "filter")]
  filter: String,
  #[clap(parse(from_os_str))]
  path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
  let arguments = Arguments::parse();

  let file = File::open(&arguments.path)?;
  let lines = BufReader::new(file).lines();

  for line in lines {
    match line {
      Ok(line) => println!("{line}"),
      Err(error) => eprintln!("{error}"),
    }
  }

  Ok(())
}
