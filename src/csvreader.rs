// Source: https://docs.rs/csv/1.1.6/csv/

use std::error::Error;
use std::fs::File;
use std::io;
use csv;
use serde::de::DeserializeOwned;
use crate::rocket_data::RocketData;

const CSV_FILE: &str = "csv/trimmed.csv";

pub fn get_rocket_data() -> Result<Vec<RocketData>, Box<dyn Error>> {
    let file = File::open(CSV_FILE)?;
    return Ok(get_csv_vec(file).unwrap());
}

// Source: https://users.rust-lang.org/t/deserialising-multiple-types-from-csv-with-serde/38338
pub fn get_csv_vec<D: DeserializeOwned, R: io::Read>(rdr: R) -> csv::Result<Vec<D>> {
    return csv::Reader::from_reader(rdr).into_deserialize().collect();
}
