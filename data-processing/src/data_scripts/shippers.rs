use std::{
    collections::{HashMap, HashSet},
    //error::Error,
    fs::{self, File, OpenOptions},
    path::Path,
   // str::FromStr,
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
pub fn aggregate_email_counts<E: std::convert::From<std::io::Error>>(dir_path: &str) -> Result<HashMap<String, usize>, E> {
    let mut total_email_counts_per_state: HashMap<String, usize> = HashMap::new();

    // Iterate over all entries in the directory
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let file_path = entry.path();

        // Check if the entry is a file and has .csv extension
        if file_path.is_file() && file_path.extension() == Some(std::ffi::OsStr::new("csv")) {
            let file_name = file_path.file_name().unwrap().to_str().unwrap();

            match CsvReader::from_path(&file_path) {
                Ok(reader) => {
                    let df = reader.has_header(true).finish().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

                    if let Ok(email_column) = df.column("EMAIL") {
                        // Create a mask where the email column is not null
                        let email_filter = !email_column.is_null();

                        // Filter the DataFrame using the mask
                        let email_df = df.filter(&email_filter).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

                        // Group by "State" column and count the number of email addresses
                        let grouped = email_df.groupby(&["STATE"]).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
                        let re_grouped = grouped.select(&["EMAIL"]).count().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

                        // Iterate through the grouped DataFrame to add the counts to the total
                        let states = re_grouped.column("STATE").map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?.utf8().unwrap();
                        let counts = re_grouped.column("EMAIL_count").map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?.u32().unwrap();

                        for (state, count) in states.into_iter().zip(counts.into_iter()) {
                            if let (Some(state_str), Some(count_num)) = (state, count) {
                                *total_email_counts_per_state.entry(state_str.to_string()).or_insert(0) += count_num as usize;
                            }
                        }
                    }
                },
                Err(e) => return Err(E::from(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))),            }
        }
    }

    Ok(total_email_counts_per_state)
}