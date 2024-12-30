use std::error::Error;
use plotly::common::Mode;
use plotly::{ImageFormat, Plot, Scatter};
use chrono::{NaiveDate, Duration};

fn main() -> Result<(), Box<dyn Error>>  {
    // Generate example time series data
    let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let dates: Vec<String> = (0..10)
        .map(|i| (start_date + Duration::days(i)).to_string())
        .collect();
    let values: Vec<f64> = (0..10).map(|i| (i as f64).sin()).collect();

    // Create a scatter plot for the time series data
    let trace = Scatter::new(dates, values)
        .name("Time Series")
        .mode(Mode::LinesMarkers);

    // Build the plot
    let mut plot = Plot::new();
    plot.add_trace(trace);
    //plot.set_title("Time Series Plot");
    plot.show();
    Ok(())
}
