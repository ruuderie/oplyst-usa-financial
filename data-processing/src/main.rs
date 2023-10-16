use data_processing::data_scripts::credit::load_business_insights;
use data_processing::data_scripts::shippers::load_shipper_insights;
use data_processing::data_scripts::sba::{create_csv, transform_csv_with_stacked_addresses};
use data_processing::data_scripts::business_credit_usa::load_business_leads_insights;
use crate::model::shippers;
use data_processing::data_scripts::zoom_info::load_zoom_info_insights;
use std::{ path::Path};
use dotenv::dotenv;
use futures::TryFutureExt;

mod model;
mod data_scripts;
mod db;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //utils::rename_csv_column::run_clean_up_of_columns();
    load_zoom_info_insights().await.expect_err("Error getting zoominfo leads");
    Ok(())
}

/*
Business Leads Insights:
    //let paths = ["./data/LOGISTICS_SHIPPERS - Owners.csv","./data/LOGISTICS_WAREHOUSES_DISTRIBUTORS - Owners.csv"];
    //let counts = aggregate_email_counts(&paths)?;
    //println!("{:?}", counts);
    //load_business_leads_insights().await.expect_err("Error getting business leads insights");
    //Ok(())
*/

/* 
    //sba file analysis 
    dotenv().ok();
    let path1 = Path::new("./data/SBA_Construction_Contractors.csv");



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
    //laoding credit insights and handle errors

    load_business_insights().await?;
*/
/*
    //create_csv().await?;
    let path1 = Path::new("./data/output_OK_ONLY_MILLIONVERIFIER.COM.csv");
    transform_csv_with_stacked_addresses(vec![path1], &Path::new("./data/manufacturers_1_09_09_2023.csv")).unwrap();

    Ok(())
*/
