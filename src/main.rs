use std::error::Error;
use serde::{Serialize};
use plotly::common::Mode;
use plotly::{ImageFormat, Plot, Scatter};
use chrono::{};

// The record we will use to write onto the csv
#[derive(Serialize,Clone)]
struct Record {
    timestamp: String,
    value: i32,
}

struct FirFilter{
    last_value: f32,
    alpha: f32,
}

impl FirFilter {
    pub fn filter_new_value(&mut self,new_value: f32) -> f32 {
        self.last_value = self.alpha * (self.last_value) + (self.alpha - 1_f32)* (new_value);
        return self.last_value;
    }
}

fn main() -> Result<(), Box<dyn Error>>  {
    
    // Initialize the plot
    let mut plot = Plot::new();
    let mut x_values: Vec<String> = vec![];
    let mut y_values: Vec<i32> = vec![];

    // Read the data (For now the data is read at once and then processed, this will be done concurrently later on)

    // Process the data
    let mut filter = FirFilter{
        last_value: 0_f32,
        alpha: 0.9
    };

    // Create an empty Scatter trace
    let trace = Scatter::new(x_values, y_values)
        .name("Dynamic Data")
        .mode(Mode::LinesMarkers);
    plot.add_trace(trace);

    // Launch the plot viewer
    plot.show();
    
    Ok(())
}
