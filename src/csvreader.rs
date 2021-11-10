// Source: https://docs.rs/csv/1.1.6/csv/

use std::error::Error;
use std::io;
use std::process;
use std::fs::File;
use csv;
use cgmath;
use serde_derive;
use serde::de::DeserializeOwned;
use serde::Deserialize;

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

#[derive(PartialEq, Debug, PartialOrd, Deserialize)]
pub struct RocketData {
    timestamp: u32,
    ax: f64,
    ay: f64,
    az: f64,
    gx: f64,
    gy: f64,
    gz: f64,
    mx: f64,
    my: f64,
    mz: f64,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    satellite_count: u32,
    position_lock: u32,
    temperature: f64,
    pressure: f64,
    barometer_altitude: f64,
    rocket_state: u32,
    l1_extension: f64,
    l2_extension: f64,
}

impl Default for RocketData {
    fn default() -> Self {
        return RocketData {
            timestamp: 0,
            ax: 0.0,
            ay: 0.0,
            az: 0.0,
            gx: 0.0,
            gy: 0.0,
            gz: 0.0,
            mx: 0.0,
            my: 0.0,
            mz: 0.0,
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
            satellite_count: 0,
            position_lock: 0,
            temperature: 0.0,
            pressure: 0.0,
            barometer_altitude: 0.0,
            rocket_state: 0,
            l1_extension: 0.0,
            l2_extension: 0.0,
        };
    }
}
