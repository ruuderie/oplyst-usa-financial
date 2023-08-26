
extern crate polars;
extern crate scraper;
extern crate csv;
extern crate regex;
use std::fs::{File, OpenOptions};  
use std::{fs, collections::HashMap, collections::HashSet};
use std::error::Error;
use csv::{ Writer, Reader};
use std::path::Path;
use regex::Regex;
use std::io::Result;
use polars::prelude::*;


use scraper::{Html, Selector};

fn parse_html_to_dataframe(html_content: &str) -> DataFrame {
    let fragment = Html::parse_fragment(html_content);

    let row_selector = Selector::parse("tr.AlternatingRowBGC4Form1, tr.AlternatingRowBGC4Form0").unwrap();
    let df = DataFrame::new_no_checks(vec![
        Series::new("Name and Trade Name of Firm", Vec::<String>::new()),
        Series::new("Contact", Vec::<String>::new()),
        Series::new("Address and City, State Zip", Vec::<String>::new()),
        Series::new("Capabilities Narrative", Vec::<String>::new()),
        Series::new("E-mail Address", Vec::<String>::new()),
    ]);

    for row in fragment.select(&row_selector) {
        let values: Vec<String> = row
            .text()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
    
        if values.len() == 6 {
            for (idx, value) in values.iter().skip(1).enumerate() {
                // The problem here is that idx starts from 0 after the skip.
                // We might need to adjust the column index accordingly.
                let adjusted_idx = idx + 1;
    
               println!("{}: {}", adjusted_idx, value);
        }
    }
    }
    df

}

pub fn create_csv() {
    let folder_path = "./data/";  // replace with the path to your folder

        // Initialize an empty DataFrame to append each file's data
        let mut aggregated_df = DataFrame::new_no_checks(vec![
            Series::new("Name and Trade Name of Firm", Vec::<String>::new()),
            Series::new("Contact", Vec::<String>::new()),
            Series::new("Address and City, State Zip", Vec::<String>::new()),
            Series::new("Capabilities Narrative", Vec::<String>::new()),
            Series::new("Email", Vec::<String>::new()),
        ]);
    
        for entry in fs::read_dir(folder_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "html") {
                let content = fs::read_to_string(&path).unwrap();
                let df = parse_html_to_dataframe(&content);
                aggregated_df = aggregated_df.vstack(&df).unwrap(); // Reassign the result to aggregated_df
            }
        }
    
        let mut file = File::create("output.csv").expect("could not create file");
    
        CsvWriter::new(&mut file)
            .has_header(true)
            .with_delimiter(b',')
            .finish(&mut aggregated_df)
            .expect("could not write to file");

}

pub fn aggregate_email_counts(dir_path: &str) -> Result<HashMap<String, usize>> {
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
                Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())),
            }
        }
    }

    Ok(total_email_counts_per_state)
}
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
pub fn transform_csv_with_stacked_addresses(input_paths: Vec<&Path>, output_path: &Path) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new(r"^(.*?), ([^,]+),? ([A-Z]{2}) (\d+-?\d*)$").unwrap();
    let mut counter = 0;
    
    let mut wtr = if output_path.exists() {
        Writer::from_writer(OpenOptions::new().append(true).open(output_path)?)
    } else {
        let mut writer = Writer::from_writer(File::create(output_path)?);
        writer.write_record(&["#", "Name and Trade Name of Firm", "First Name", "Last Name", "Address", "City", "State", "Zip", "Capabilities Narrative", "EMAIL"])?;
        writer
    };

    for input_path in input_paths.iter() {
        let mut rdr = Reader::from_reader(File::open(input_path)?);
        let mut prev_record: Option<Vec<Vec<String>>> = None;
        let mut records = rdr.records().peekable();
        
        // Continue from here
        let mut process_stacked_records = |prev: &mut Vec<Vec<String>>, wtr: &mut Writer<_>, re: &Regex, counter: &mut usize| -> std::result::Result<(), Box<dyn std::error::Error>> {
            let full_address = prev.iter().map(|vec| &vec[3]).cloned().collect::<Vec<String>>().join(", ");

            let mut consolidated_record = vec!["".to_string(); 9];
        
            for record in prev.iter() {
                for (i, field) in record.iter().enumerate() {
                    if !field.is_empty() && consolidated_record[i].is_empty() {
                        consolidated_record[i] = field.clone();
                    }
                }
            }

            consolidated_record[3] = full_address;
        
            process_record(&consolidated_record, wtr, re, counter)
        };

        while let Some(result) = records.next() {
            let current_record = result?.iter().map(|s| s.to_string()).collect::<Vec<String>>();
            if !current_record[0].is_empty() {
                if let Some(mut prev) = prev_record.take() {
                    process_stacked_records(&mut prev, &mut wtr, &re, &mut counter)?;
                }
                prev_record.get_or_insert_with(Vec::new).push(current_record);
            } else {
                prev_record.get_or_insert_with(Vec::new).push(current_record);
            }
        }

        if let Some(mut prev) = prev_record.take() {
            process_stacked_records(&mut prev, &mut wtr, &re, &mut counter)?;
        }
    }
    

    Ok(())
}
fn process_record(record: &[String], wtr: &mut Writer<File>, re: &Regex, counter: &mut usize) -> std::result::Result<(), Box<dyn std::error::Error>> {
    if record.len() < 6 {  // Check if record has at least 6 fields
        eprintln!("Incomplete record found with length {}: {:?}", record.len(), record);
        return Ok(());
    }
    println!("Processing record: {:?}", record);
    
    // Parsing names
    let names: Vec<&str> = record[2].split_whitespace().collect();
    let first_name = names.get(0).unwrap_or(&"").to_string();
    let last_name = names.get(1..).map_or("".to_string(), |slice| slice.join(" "));
    
    println!("First Name: {}", first_name);
    println!("Last Name: {}", last_name);

    // Parsing address
    let re_address = Regex::new(r"(?i)^([\w\s]+),\s+([A-Za-z\s]+)[,\s]+([A-Z]{2})\s+(\d{5}(?:-\d{4})?)$").unwrap();

    let mut address = "".to_string();
    let mut city = "".to_string();
    let mut state = "".to_string();
    let mut zip = "".to_string();

    if let Some(caps) = re_address.captures(&record[3]) {
        address = caps.get(1).map_or("", |m| m.as_str()).trim().to_string();
        city = caps.get(2).map_or("", |m| m.as_str()).trim().to_string();
        state = caps.get(3).map_or("", |m| m.as_str()).to_string();
        zip = caps.get(4).map_or("", |m| m.as_str()).to_string();
    } else {
        eprintln!("Failed to parse address from record: {:?}", record);
        return Ok(());
    }
    
    println!("Parsed state: {:?}", state);

    // Validate state abbreviation
    let valid_states: HashSet<&str> = ["AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA", "HI", "ID", "IL", "IN", "IA", "KS", "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV", "NH", "NJ", "NM", "NY", "NC", "ND", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN", "TX", "UT", "VT", "VA", "WA", "WV", "WI", "WY"].iter().cloned().collect();
    
    if !valid_states.contains(state.as_str()) {
        eprintln!("Invalid state abbreviation in record: {:?}", record);
        return Ok(());
    }
    
    println!("Address: {}", address);
    println!("City: {}", city);
    println!("State: {}", state);
    println!("ZIP: {}", zip);

    // Email and capabilities narrative
    let capabilities = &record[4];
    let email = &record[5];
    
    println!("Capabilities: {}", capabilities);
    println!("Email: {}", email);

    let record_to_write = vec![&record[0], &record[1], &first_name, &last_name, &address, &city, &state, &zip, &capabilities, &email];
    println!("Record to write: {:?}", record_to_write);
    wtr.write_record(&record_to_write)?;
    wtr.flush()?;
    
    Ok(())
}