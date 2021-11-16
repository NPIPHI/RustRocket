// Source: https://docs.rs/csv/1.1.6/csv/

use std::error::Error;
use std::fs::File;
use std::io;
use csv;
use serde::de::DeserializeOwned;
use crate::rocket_data::RocketData;

const CSV_FILE: &str = "csv/trimmed.csv";
// pub static ROCKET_DATA: Vec<RocketData> = get_rocket_data().unwrap();

pub fn get_rocket_data() -> Result<Vec<RocketData>, Box<dyn Error>> {
    /*
    // Build the CSV reader and iterate over each record.
    let file = File::open(CSV_FILE)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    let x: Vec<RocketData> = parse_csv(file)?;
    return Ok(());
    */
    let file = File::open(CSV_FILE)?;
    return Ok(get_csv_vec(file).unwrap());
}

// Source: https://users.rust-lang.org/t/deserialising-multiple-types-from-csv-with-serde/38338
pub fn get_csv_vec<D: DeserializeOwned, R: io::Read>(rdr: R) -> csv::Result<Vec<D>> {
    return csv::Reader::from_reader(rdr).into_deserialize().collect();
}