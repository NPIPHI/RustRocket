use std::process;

mod csvreader;

fn main() {
  let rocket_data = csvreader::example().unwrap();
  if let Err(err) = csvreader::example() {
      println!("error running example: {}", err);
      process::exit(1);
  }
}
