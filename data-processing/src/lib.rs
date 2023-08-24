
extern crate polars;
extern crate scraper;
extern crate csv;
extern crate regex;
use std::path::Path;
use std::{fs, collections::HashMap};
use std::error::Error as StdError;
use regex::Regex;
use std::io::Result;
use std::fs::File;
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
pub fn transform_csv_with_stacked_addresses(input_path: &Path, output_path: &Path) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_reader(File::open(input_path)?);
    let mut wtr = csv::Writer::from_writer(File::create(output_path)?);

    // Write headers to the new CSV
    wtr.write_record(&["#", "Name and Trade Name of Firm", "Contact", "Address", "City", "State", "Zip", "Capabilities Narrative", "EMAIL"])?;

    for result in rdr.records() {
        let record = result?;
        let address = &record[3];

        // Use regex to split address into "Address", "City", "State" and "Zip"
        let re = Regex::new(r"^(.*), ([^,]+), ([A-Z]{2}) (\d+)$").unwrap();
        let caps = re.captures(address).unwrap_or_else(|| {
            panic!("Failed to parse address: {}", address);
        });

        // Write to new CSV
        wtr.write_record(&[
            &record[0], 
            &record[1], 
            &record[2], 
            caps.get(1).map_or("", |m| m.as_str()),
            caps.get(2).map_or("", |m| m.as_str()),
            caps.get(3).map_or("", |m| m.as_str()),
            caps.get(4).map_or("", |m| m.as_str()),
            &record[4], 
            &record[5]
        ])?;
    }

    wtr.flush()?;

    Ok(())
}


