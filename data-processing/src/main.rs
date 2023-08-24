extern crate polars;
use data_processing::{analyze_b2b_data, create_csv, transform_csv_with_stacked_addresses};
use polars::prelude::*;
use std::{error::Error, fs, collections::HashMap, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {


    Ok(())
}
/*
    //let usa_prefix = "usa_";
    //analyze_b2b_data("./data/USA.csv", &usa_prefix)?;
*/

        /*
        // Aggregate email counts from CSV files
        let email_counts = aggregate_email_counts(csv_dir_path)?;
    
        // Print the results
        println!("Total email counts per state:");
        for (state, count) in email_counts.iter() {
            println!("State: {}, Email Count: {}", state, count);
        }
        */