extern crate polars;
use data_processing::create_csv;
use polars::prelude::*;
use std::{error::Error, fs, collections::HashMap, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
        // Directory paths
        let csv_dir_path = "./data/";
    
        // Parse HTML files and generate a CSV
        create_csv();
        /*
        // Aggregate email counts from CSV files
        let email_counts = aggregate_email_counts(csv_dir_path)?;
    
        // Print the results
        println!("Total email counts per state:");
        for (state, count) in email_counts.iter() {
            println!("State: {}, Email Count: {}", state, count);
        }
        */
        Ok(())
}
