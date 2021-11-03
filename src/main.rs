use std::process;

mod csvreader;

fn main() {
  if let Err(err) = csvreader::example() {
      println!("error running example: {}", err);
      process::exit(1);
  }
}
