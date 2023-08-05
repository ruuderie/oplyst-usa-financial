extern crate polars;

use polars::prelude::*;
use std::{error::Error, collections::HashMap};

fn main() -> Result<(), Box<dyn Error>> {
    let files = vec![
        "USA-CARRIERS-Dry_Bulk.csv",
        "LOGISTICS_SHIPPERS - Owners.csv",
        "USA-CARRIERS-Farm_Supplies.csv",
        "USA-CARRIERS-Meat_Only.csv",
        "LOGISTICS_WAREHOUSES_DISTRIBUTORS - Owners.csv",
        "USA-CARRIERS-Fresh_Produce.csv",
        "USA-CARRIERS-Metal_Sheet.csv",
        "USA-CARRIERS-General_Freight.csv",
        "USA-CARRIERS-Mobile_Home.csv",
        "USA-CARRIERS-All_States.csv",
        "USA-CARRIERS-Grain_Feed_Hay.csv",
        "USA-CARRIERS-Motor_Vehicles.csv",
        "USA-CARRIERS-Beverages.csv",
        "USA-CARRIERS-Household_Goods.csv",
        "USA-CARRIERS-Paper_Products.csv",
        "USA-CARRIERS-Building_Materials.csv",
        "USA-CARRIERS-Intermodal_Only.csv",
        "USA-CARRIERS-Reefer.csv",
        "USA-CARRIERS-Chemicals_Only.csv",
        "USA-CARRIERS-Large_Machinery.csv",
        "USA-CARRIERS-US_Mail_Only.csv",
        "USA-CARRIERS-Coal_Coke.csv",
        "USA-CARRIERS-Liquid_Gases.csv",
        "USA-CARRIERS_Passenger.csv",
        "USA-CARRIERS-Log_Pole.csv",

    ];
   // let files_without_emails = vec!["Construction Companies - Loans.csv"];
    let mut total_email_counts_per_state: HashMap<String, usize> = HashMap::new();

    for file in files.iter() {
        // Adjust the file path according to your directory structure
        let path = format!("./data/{}", file);

        match CsvReader::from_path(&path)?.has_header(true).finish() {
            Ok(df) => {
                if let Ok(email_column) = df.column("EMAIL") {
                    // Create a mask where the email column is not null
                    let email_filter = !email_column.is_null();

                    // Filter the DataFrame using the mask
                    let email_df = df.filter(&email_filter)?;

                    // Group by "State" column and count the number of email addresses
                    let grouped = email_df.groupby(&["STATE"])?;
                    println!("State Count: {} {}", file, &grouped.count()?);
                    println!("Email: {} {}", file, &grouped.clone().select(&["EMAIL"]).count()?);    
                    let re_grouped = &grouped.clone().select(&["EMAIL"]).count()?;
                    println!("From File with ReGrouped: {} {}", file,re_grouped);
                    
                    // Iterate through the grouped DataFrame to add the counts to the total
                    let states = re_grouped.column("STATE")?.utf8().unwrap();
                    let counts = re_grouped.column("EMAIL_count")?.u32().unwrap();

                    for (state, count) in states.into_iter().zip(counts.into_iter()) {
                        if let (Some(state_str), Some(count_num)) = (state, count) {
                            *total_email_counts_per_state.entry(state_str.to_string()).or_insert(0) += count_num as usize;
                        }
                    }
                } else {
                    println!("File: {} doesn't have an 'email' column", file);
                }
            },
            Err(err) => println!("Error reading file {}: {:?}", file, err),
        }
    }

    println!("Total email counts per state:");
    for (state, count) in total_email_counts_per_state.iter() {
        println!("State: {}, Email Count: {}", state, count);
    }

    Ok(())
}
