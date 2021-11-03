// Source: https://docs.rs/csv/1.1.6/csv/

use std::error::Error;
use std::io;
use std::process;
use std::fs::File;
use csv;

const csv_file: &str = "csv/trimmed.csv";

pub fn example() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let file = File::open(csv_file)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
