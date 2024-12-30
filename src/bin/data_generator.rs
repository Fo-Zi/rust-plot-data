use std::{fmt::write, fs::File};
use csv::{Writer};
use serde::{Serialize};
use chrono::{format, DateTime, TimeZone, Utc};
use rand::{Rng};

const TIMESTAMP_STEP_IN_HOURS: u32 = 1;
const NUMBER_OF_POINTS: u32 = 5000;
const MAX_DATA_VALUE: i32 = 50;
const MIN_DATA_VALUE: i32 = -30 ;
const FILE_PATH: &str =  "csv_test.txt";

#[macro_export]
macro_rules! format_datetime {
    ($dt:expr) => {
        format!("{}", $dt.format("%Y-%m-%d %H:%M:%S"))
    };
}

#[derive(Serialize)]
struct Record {
    timestamp: String,
    value: i32,
}

fn main() {
    let start_date_time = Utc.with_ymd_and_hms(2020, 1, 1, 0, 1, 1).unwrap();
    let formatted = format_datetime!(start_date_time);
    assert_eq!(formatted, "2020-01-01 00:01:01");

    let start_timestamp = start_date_time.timestamp();
    let mut rng = rand::thread_rng();

    let mut writer = Writer::from_path(FILE_PATH).unwrap();

    // Write the headers
    //writer.write_record(&["Timestamp", "Value"]).unwrap();

    for i in 0..NUMBER_OF_POINTS {
        let timestamp = start_timestamp + (i*TIMESTAMP_STEP_IN_HOURS*60*60) as i64 ;
        let record = Record{
            timestamp: format_datetime!(DateTime::from_timestamp(timestamp,0).unwrap()),
            value: rng.gen_range(MIN_DATA_VALUE..MAX_DATA_VALUE),
        };
        writer.serialize(record).unwrap();
    }
    writer.flush().unwrap();
    
}
