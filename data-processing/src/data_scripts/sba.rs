use std::{
    collections::{HashMap, HashSet},
    fs::{self, File, OpenOptions},
    path::Path,
};
use tokio_postgres::{NoTls};
use csv::{Reader,QuoteStyle,Writer,WriterBuilder};
use polars::prelude::*;
use regex::Regex;
use inflector::Inflector;
use scraper::{Html, Selector};
use std::future::Future;
use std::pin::Pin;
use crate::utils::zip_code::{ZipData, load_zip_data};
extern crate log;
extern crate env_logger;
use log::debug;
//Small Business Data
pub fn transform_csv_with_stacked_addresses(input_paths: Vec<&Path>, output_path: &Path) -> std::result::Result<(), Box<dyn std::error::Error>> {

    let re = Regex::new(r"^(.*?), ([^,]+),? ([A-Z]{2}) (\d+-?\d*)$").unwrap();
    let mut counter = 0;

    let mut wtr = if output_path.exists() {
        WriterBuilder::new()
            .quote_style(QuoteStyle::Never)
            .from_writer(OpenOptions::new().append(true).open(output_path)?)
    } else {
        let mut writer = WriterBuilder::new()
            .quote_style(QuoteStyle::Necessary)
            .from_writer(File::create(output_path)?);
        writer.write_record(&[
            "#",
            "Name and Trade Name of Firm",
            "First Name",
            "Last Name",
            "Address",
            "City",
            "State",
            "Zip",
            "Capabilities Narrative",
            "Email",
        ])?;
        writer
    };
    let zip_data_path = Path::new("./data/zip code/us_zip_code.csv");
    let zip_data = load_zip_data(zip_data_path)?;

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
        
            process_record(&consolidated_record, wtr, &re, &zip_data, counter)?;
            Ok(())
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
fn process_record(record: &[String], wtr: &mut Writer<File>, re: &Regex, zip_data: &[ZipData], counter: &mut usize) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
    let names: Vec<&str> = record[1].split_whitespace().collect();
    let first_name = names.get(0).unwrap_or(&"").to_string().to_title_case();
    let last_name = names.get(1..).map_or("".to_string(), |slice| slice.join(" ")).to_title_case();

    // Parsing address
 
    let mut address = "".to_string();
    let mut city = "".to_string();
    let mut state = "".to_string();
    let mut zip = "".to_string();


    if let Some((addr, cty, st, zp)) = parse_address_from_patterns(&record[2], zip_data) {
        address = addr;
        city = cty;
        state = st;
        zip = zp;
    } else {
        eprintln!("Failed to parse address from record: {:?}", &record[2]);
        return Ok(());
    }


    // Email and capabilities narrative
    let capabilities = &record[3];
    let email = &record[4].to_lowercase();
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
        //println!("Record to write: {:?}", record_to_write);
        wtr.write_record(&record_to_write.iter().map(AsRef::as_ref).collect::<Vec<&str>>())?;
        wtr.flush()?;
    } else {
        wtr.flush()?;
        println!("Skipping record with personal email domain: {:?}", record_to_write);  
    }
    Ok(())
}
fn parse_address_from_patterns(record: &str, zip_data: &[ZipData]) -> Option<(String, String, String, String)> {
    println!("Processing record: {}", record);

    // Extract only the first 5 digits of the ZIP code
    let zip_pattern = r"\b(\d{5})\b";

    if let Ok(re) = Regex::new(zip_pattern) {
        if let Some(caps) = re.captures(record) {
            let zip = caps.get(1).unwrap().as_str();
            println!("Extracted ZIP: {}", zip);

            // Using the ZIP code, obtain the city and state from zip_data
            if let Some(data) = zip_data.iter().find(|&data| data.zip == zip) {
                let city = &data.city;
                let state_id = &data.state_id;
                println!("Matched city: {} and state: {}", city, state_id);

                // Procedural approach to extract the address
                let trimmed_record = record.splitn(2, zip).next().unwrap().trim(); // remove zip and following characters
                let without_state = trimmed_record.trim_end_matches(state_id).trim(); // remove state
                let mut address = Regex::new(&format!(r"(?i){}", city))
                .unwrap()
                .replace_all(without_state, "")
                .to_string()
                .trim()
                .to_string();
                if address.ends_with(',') {
                    address.pop();
                }
            
                println!("Procedurally extracted address: {}", address);
                
                return Some((address.to_string(), city.to_string(), state_id.to_string(), zip.to_string()));
            }
        }
    }
    println!("Failed to parse.");
    None
}

fn sanitize(s: &str) -> String {
    let res = s.replace("\"", "");
    res
}


//functions to convert html file to a usable csv file

//parsing html to create csv
fn parse_html_to_dataframe(html_content: &str) -> DataFrame {
    let fragment = Html::parse_fragment(html_content);

    let row_selector = Selector::parse("tr.AlternatingRowBGC4Form1, tr.AlternatingRowBGC4Form0").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    let df = DataFrame::new_no_checks(vec![
        Series::new("Name and Trade Name of Firm", Vec::<String>::new()),
        Series::new("Contact", Vec::<String>::new()),
        Series::new("Address and City, State Zip", Vec::<String>::new()),
        Series::new("Capabilities Narrative", Vec::<String>::new()),
        Series::new("Email", Vec::<String>::new()),
    ]);

    let mut name_and_trade = Vec::new();
    let mut contact = Vec::new();
    let mut address = Vec::new();
    let mut capabilities = Vec::new();
    let mut email = Vec::new();

    for row in fragment.select(&row_selector) {
        let mut values = Vec::new();
        for cell in row.select(&td_selector) {
            let value = cell.text().collect::<Vec<_>>().join("").trim().to_string();
            if !value.is_empty() {
                values.push(value);
            }
        }

        if values.len() == 6 {
            name_and_trade.push(values[1].clone());
            contact.push(values[2].clone());
            address.push(values[3].clone());
            capabilities.push(values[4].clone());
            email.push(values[5].clone());
        }
    }

    let df = DataFrame::new(vec![
        Series::new("Name and Trade Name of Firm", name_and_trade),
        Series::new("Contact", contact),
        Series::new("Address and City, State Zip", address),
        Series::new("Capabilities Narrative", capabilities),
        Series::new("Email", email),
    ]).unwrap();

    
    println!("df: {:?}", df);
    df

}

pub async fn create_csv() -> Result<(), Box<dyn std::error::Error>>{
    let folder_path = "./data/sba/";  // replace with the path to your folder

        // Initialize an empty DataFrame to append each file's data
        let mut aggregated_df = DataFrame::new_no_checks(vec![
            Series::new("Name and Trade Name of Firm", Vec::<String>::new()),
            Series::new("Contact", Vec::<String>::new()),
            Series::new("Address and City, State Zip", Vec::<String>::new()),
            Series::new("Capabilities Narrative", Vec::<String>::new()),
            Series::new("Email", Vec::<String>::new()),
        ]);
    
        for entry in fs::read_dir(folder_path).unwrap() {
            println!("Processing file: {:?}", entry);
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
    
    Ok(())

}