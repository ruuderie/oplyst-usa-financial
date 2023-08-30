extern crate polars;
//use data_processing::{create_csv, transform_csv_with_stacked_addresses};
// src/main.rs
use data_processing::credit_insights::credit::load_business_insights;
use std::{ path::Path};
use dotenv::dotenv;
mod db;
mod credit_insights;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Run your business logic function and handle errors
    load_business_insights().await?;

    Ok(())
}
/* 
    //sba file analysis 
    dotenv().ok();
    let path1 = Path::new("./data/SBA_Construction_Contractors.csv");
    let path2 = Path::new("./data/SBA_Construction_Contractors_2.csv");
    transform_csv_with_stacked_addresses(vec![path1, path2], &Path::new("./data/SBA_Construction_08_28.csv")).unwrap();


async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();

    // Connect to the database.
    let (client, connection) =
    tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    get_business_leads_insights().await.expect_err("Error getting business leads insights");

    Ok(())
}

*/
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