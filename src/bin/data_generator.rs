use csv::{Writer};
use serde::{Serialize};
use chrono::{DateTime, TimeZone, Utc};
use rand::{Rng};

// Global const parameters
const TIMESTAMP_STEP_IN_HOURS: u32 = 1;
const NUMBER_OF_POINTS: u32 = 5000;
const MAX_DATA_VALUE: i32 = 50;
const MIN_DATA_VALUE: i32 = -30 ;
const FILE_PATH: &str =  "csv_test.csv";

// For readability
#[macro_export]
macro_rules! format_datetime {
    ($dt:expr) => {
        format!("{}", $dt.format("%Y-%m-%d %H:%M:%S"))
    };
}

// The record we will use to write onto the csv
#[derive(Serialize)]
struct Record {
    timestamp: String,
    value: f32,
}

// Static sine lookup table for common angles in degrees.
const SINE_LOOKUP_TABLE: [f32; 36] = [
    0.0,            // 0 degrees
    0.173_648_1,    // 10 degrees
    0.342_020_1,    // 20 degrees
    0.5,            // 30 degrees
    0.642_787_6,    // 40 degrees
    0.766_044_4,    // 50 degrees
    0.866_025_4,    // 60 degrees
    0.939_692_6,    // 70 degrees
    0.984_807_7,    // 80 degrees
    1.0,            // 90 degrees
    0.984_807_7,    // 100 degrees
    0.939_692_6,    // 110 degrees
    0.866_025_4,    // 120 degrees
    0.766_044_4,    // 130 degrees
    0.642_787_6,    // 140 degrees
    0.5,            // 150 degrees
    0.342_020_1,    // 160 degrees
    0.173_648_1,    // 170 degrees
    0.0,            // 180 degrees
   -0.173_648_1,    // 190 degrees
   -0.342_020_1,    // 200 degrees
   -0.5,            // 210 degrees
   -0.642_787_6,    // 220 degrees
   -0.766_044_4,    // 230 degrees
   -0.866_025_4,    // 240 degrees
   -0.939_692_6,    // 250 degrees
   -0.984_807_7,    // 260 degrees
   -1.0,            // 270 degrees
   -0.984_807_7,    // 280 degrees
   -0.939_692_6,    // 290 degrees
   -0.866_025_4,    // 300 degrees
   -0.766_044_4,    // 310 degrees
   -0.642_787_6,    // 320 degrees
   -0.5,            // 330 degrees
   -0.342_020_1,    // 340 degrees
   -0.173_648_1,    // 350 degrees
];

fn truncate_to_two_decimal_places(value: f32) -> f32 {
    (value * 100.0).trunc() / 100.0
}

fn main() {
    
    // Arbitrary date as a "seed" to generate the time series
    let start_date_time = Utc.with_ymd_and_hms(2020, 1, 1, 0, 1, 1).unwrap();
    let start_timestamp = start_date_time.timestamp();
    
    // Instantiating random number generator 
    let mut rng = rand::thread_rng();

    // Instantiating csv writer from file
    let mut writer = Writer::from_path(FILE_PATH).unwrap();

    // Data generation 
    for i in 0..NUMBER_OF_POINTS {
        let timestamp = start_timestamp + (i*TIMESTAMP_STEP_IN_HOURS*60*60) as i64 ;
        let value = truncate_to_two_decimal_places(rng.gen_range(MIN_DATA_VALUE..MAX_DATA_VALUE) as f32 + 30_f32*SINE_LOOKUP_TABLE[i as usize % 36]);
        let record = Record{
            timestamp: format_datetime!(DateTime::from_timestamp(timestamp,0).unwrap()),
            value ,
        };
        writer.serialize(record).unwrap();
    }
    writer.flush().unwrap();
    
}
