use std::{
    collections::{HashMap, HashSet},
    fs::{self, File, OpenOptions},
    path::Path,
};
use tokio_postgres::{NoTls};
use csv::{Reader,QuoteStyle,Writer,WriterBuilder};
use polars::prelude::*;
use regex::Regex;
use scraper::{Html, Selector};
use std::future::Future;
use std::pin::Pin;
extern crate log;
extern crate env_logger;
use log::debug;
//for parsing usa dataset
pub fn analyze_b2b_data(file_path: &str, prefix: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    // 1. Load the data
    let df = CsvReader::new(File::open(file_path)?)
        .infer_schema(Some(1000))
        .has_header(true)
        .finish()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // Print the first 5 rows
    println!("{:?}", df.head(Some(5)));

    // Convert the DataFrame to a LazyFrame
    let ldf = df.lazy();

    // 2. Analyses
    let analyses = vec![
        ("Industry", "industry_distribution"),
        ("Job title", "job_title_distribution"),
        ("Company Name", "company_distribution"),
        ("Company Size", "company_size_distribution"),
        ("Location", "location_distribution")
    ];

    for (column, file_suffix) in analyses.iter() {
        let output = ldf.clone()
            .groupby(vec![col(column)])
            .agg(vec![col(column).count()])
            .sort(column, SortOptions { descending: true, nulls_last: true })
            .collect()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        // Placeholder for writing the DataFrame to CSV. Adjust this to the correct method.
        println!("{:?}", output);
    }

    println!("All distributions saved to CSV files.");
    Ok(())
}
