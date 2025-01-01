use std::error::Error;
use serde::{Serialize,Deserialize};
use plotly::common::Mode;
use plotly::{ Plot, Scatter};

const FILE_PATH: &str =  "csv_test.csv";

// The record we will use to write onto the csv
#[derive(Serialize,Deserialize,Clone)]
struct Record {
    timestamp: String,
    value: f32,
}

struct FirFilter{
    last_value: f32,
    alpha: f32,
}

impl FirFilter {
    pub fn filter_new_value(&mut self,new_value: f32) -> f32 {
        self.last_value = self.alpha * (self.last_value) + (self.alpha - 1_f32)* (new_value);
        self.last_value
    }
}

fn main() -> Result<(), Box<dyn Error>>  {
    
    // Initialize the plot
    let mut plot = Plot::new();
    let mut x_values: Vec<String> = vec![];
    let mut y_values: Vec<f32> = vec![];

    // Read the data (For now the data is read at once and then processed, this will be done concurrently later on)
    let mut reader = csv::Reader::from_path(FILE_PATH).unwrap();

    // Process the data
    let mut filter = FirFilter{
        last_value: 0_f32,
        alpha: 0.9
    };
    for result in reader.deserialize() {
        let record: Record = result?;
        let filtered_value = filter.filter_new_value(record.value);   
        //let filtered_value = record.value;   
        x_values.push(record.timestamp);
        y_values.push(filtered_value);
    }

    // Create an empty Scatter trace
    let trace = Scatter::new(x_values, y_values)
        .name("Dynamic Data")
        .mode(Mode::LinesMarkers);
    plot.add_trace(trace);

    // Launch the plot viewer
    plot.show();
    
    Ok(())

}
