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
//Small Business Data
pub fn transform_csv_with_stacked_addresses(input_paths: Vec<&Path>, output_path: &Path) -> std::result::Result<(), Box<dyn std::error::Error>> {

    println!("Initializing regex for address parsing...");
    let re = Regex::new(r"^(.*?), ([^,]+),? ([A-Z]{2}) (\d+-?\d*)$").unwrap();
    println!("Initializing counter...");
    let mut counter = 0;
    println!("Setting up common email domains...");
    

    let re = Regex::new(r"^(.*?), ([^,]+),? ([A-Z]{2}) (\d+-?\d*)$").unwrap();
    let mut counter = 0;
    
    let mut wtr = if output_path.exists() {
        let w = WriterBuilder::new().quote_style(QuoteStyle::Never).from_writer(OpenOptions::new().append(true).open(output_path)?);
        w
    } else {
        let mut writer = WriterBuilder::new().quote_style(QuoteStyle::Necessary).from_writer(File::create(output_path)?);
        writer.write_record(&["#", "Name and Trade Name of Firm", "First Name", "Last Name", "Address", "City", "State", "Zip", "Capabilities Narrative", "Email"])?;
        writer
    };

    for input_path in input_paths.iter() {
        println!("Processing file: {:?}", input_path);
        let mut rdr = Reader::from_reader(File::open(input_path)?);
        let mut prev_record: Option<Vec<Vec<String>>> = None;
        let mut records = rdr.records().peekable();

        let mut process_stacked_records = |prev: &mut Vec<Vec<String>>, wtr: &mut Writer<_>, re: &Regex, counter: &mut usize| -> std::result::Result<(), Box<dyn std::error::Error>> {
            debug!("Processing stacked records");
        
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

fn has_personal_email_domain(email: &str, common_email_domains: &HashSet<&str>) -> bool {
    let email_domain = email.split('@').last().unwrap_or("");
    common_email_domains.contains(email_domain)
}
fn process_record(record: &[String], wtr: &mut Writer<File>, re: &Regex, counter: &mut usize) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let common_email_domains: HashSet<&str> = [
        "gmail.com",
        "yahoo.com",
        "hotmail.com",
        "aol.com",
        "outlook.com",
        "msn.com",
        "icloud.com",
        "live.com",
        "comcast.net",
        "ymail.com",
        "mail.com",
        "verizon.net",
        "sbcglobal.net",
        "cox.net",
        "me.com",
        "att.net",
        "bellsouth.net",
        "rocketmail.com",
        "earthlink.net",
        "optonline.net",
        "juno.com",
        "protonmail.com"].iter().cloned().collect();

    if record.len() < 6 {  // Check if record has at least 6 fields
        eprintln!("Incomplete record found with length {}: {:?}", record.len(), record);
        return Ok(());
    }

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
    let email = &record[5].to_lowercase();

    println!("Capabilities: {}", capabilities);
    println!("Email: {}", email);
    let binding = counter.to_string();

    let record_to_write: Vec<String> = vec![
    binding,    
    sanitize(&record[1]),
    sanitize(&first_name),
    sanitize(&last_name),
    sanitize(&address),
    sanitize(&city),
    sanitize(&state),
    sanitize(&zip),
    sanitize(&capabilities),
    sanitize(&email)];
    if !has_personal_email_domain(email, &common_email_domains) {
        *counter += 1;
        println!("Record to write: {:?}", record_to_write);
        wtr.write_record(&record_to_write.iter().map(AsRef::as_ref).collect::<Vec<&str>>())?;
        wtr.flush()?;
    } else {
        wtr.flush()?;
        println!("Skipping record with personal email domain: {:?}", record_to_write);  
    }
    Ok(())
}

fn sanitize(s: &str) -> String {
    println!("Sanitizing: {}", s);
    let res = s.replace("\"", "");
    println!("Sanitized: {}", res);
    res
}


//functions to convert html file to a usable csv file

//parsing html to create csv
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