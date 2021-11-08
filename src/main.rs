use std::process;

mod csvreader;
mod mvpmatrix;

fn main() {
  let rocket_data = csvreader::get_rocket_data().unwrap();
  if let Err(err) = csvreader::get_rocket_data() {
      println!("error running example: {}", err);
      process::exit(1);
  }
}
