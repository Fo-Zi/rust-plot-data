use std::{fmt::write, fs::File};
use csv::{Writer};
use serde::{Serialize};
use chrono::{format, DateTime, TimeZone, Utc};
use rand::{Rng};
use std::f64::consts::PI;

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
    0.0, // 0 degrees
    0.17364817766693033, // 10 degrees
    0.3420201433256687, // 20 degrees
    0.49999999999999994, // 30 degrees
    0.6427876096865393, // 40 degrees
    0.766044443118978, // 50 degrees
    0.8660254037844387, // 60 degrees
    0.9396926207859084, // 70 degrees
    0.984807753012208, // 80 degrees
    1.0, // 90 degrees
    0.984807753012208, // 100 degrees
    0.9396926207859084, // 110 degrees
    0.8660254037844387, // 120 degrees
    0.766044443118978, // 130 degrees
    0.6427876096865393, // 140 degrees
    0.49999999999999994, // 150 degrees
    0.3420201433256687, // 160 degrees
    0.17364817766693033, // 170 degrees
    0.0, // 180 degrees
    -0.17364817766693033, // 190 degrees
    -0.3420201433256687, // 200 degrees
    -0.49999999999999994, // 210 degrees
    -0.6427876096865393, // 220 degrees
    -0.766044443118978, // 230 degrees
    -0.8660254037844387, // 240 degrees
    -0.9396926207859084, // 250 degrees
    -0.984807753012208, // 260 degrees
    -1.0, // 270 degrees
    -0.984807753012208, // 280 degrees
    -0.9396926207859084, // 290 degrees
    -0.8660254037844387, // 300 degrees
    -0.766044443118978, // 310 degrees
    -0.6427876096865393, // 320 degrees
    -0.49999999999999994, // 330 degrees
    -0.3420201433256687, // 340 degrees
    -0.17364817766693033, // 350 degrees
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
            value: value ,
        };
        writer.serialize(record).unwrap();
    }
    writer.flush().unwrap();
    
}
