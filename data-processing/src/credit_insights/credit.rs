use crate::db;
use crate::db::us_business_credit_analysis;
use crate::db::postgres;
use std::{
    fs::{self, File, OpenOptions},
};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use serde::ser::StdError;
use polars::prelude::*;
use std::future::Future;

type TaskResult = Result<(), Box<dyn std::error::Error>>;
type TaskFuture = dyn Future<Output = TaskResult> + Send;



// Path: data-processing/src/model/mod.rs Credit Data

pub async fn load_business_insights() -> TaskResult {
    // Read the data from the file at runtime
    let data = fs::read_to_string("./data/BusinessLeadsUSA.csv")?;
    let output_file = "./data/business_leads_insights.csv";

    // Process the data and insert into database
    read_and_process(&data, output_file).await
}
async fn read_and_process(
    data: &str, 
    output_file: &str,
) -> std::result::Result<(), Box<dyn std::error::Error>>{


    // Parse the CSV data into a DataFrame
    let null_values = NullValues::AllColumns(vec![
        "CA WC".to_string(),
        "DE WC".to_string(),
        "PA WC".to_string(),
        "MI WC".to_string(),
        "NJ WC".to_string(),
        "NY WC".to_string(),
        "TX WC".to_string(),
        "ISO CGL".to_string(),
        "NCCI".to_string(),
        "NAICS".to_string(),
        "SIC".to_string(),
        "".to_string(), 
        "NULL".to_string(),
        "N/A".to_string(),
        "AREA CODE AND PHONE".to_string()
    ]);
    let df = CsvReader::new(std::io::Cursor::new(data.as_bytes()))
    .infer_schema(Some(50))
    .with_null_values(Some(null_values))
    .finish()?;

    // Group by 'SIC NAME1', 'SALES VOLUME', and 'CREDIT SCORE CODE' and then aggregate by counting the number of companies
    let grouped = df.groupby(&["SIC NAME1", "SALES VOLUME", "CREDIT SCORE CODE"])?
        .select(&["BUSINESS NAME"])
        .count()?;

        let file = std::fs::File::create(output_file)?;
        let mut csv_writer = CsvWriter::new(file).has_header(true);
        csv_writer.finish(&mut grouped.clone())?;
        


    let business_type_series = grouped.column("SIC NAME1").unwrap();
    let sales_volume_series = grouped.column("SALES VOLUME").unwrap();
    let credit_score_series = grouped.column("CREDIT SCORE CODE").unwrap();
    let companies_series = grouped.column("BUSINESS NAME_count").unwrap();

    let num_rows = business_type_series.len();

    // Define chunk size
    let chunk_size = 100; // You can adjust this as per your need
        // Establish a connection to the database
        let mut db = postgres::establish_connection().await?;
        // Process each chunk concurrently
    let mut all_tasks: Vec<Box<dyn Future<Output = Result<(), Box<dyn StdError>>> + Send>> = Vec::new();
    for i in (0..num_rows).step_by(chunk_size) {
        let end = std::cmp::min(i + chunk_size, num_rows);
        let chunk = (
            business_type_series.slice(i.try_into().unwrap(), end - i),
            sales_volume_series.slice(i.try_into().unwrap(), end - i),
            credit_score_series.slice(i.try_into().unwrap(), end - i),
            companies_series.slice(i.try_into().unwrap(), end - i)
        );
        send_chunk_to_enterprise_db(chunk, &mut db).await?;  // Pass the SeaORM connection and make the call async
    }
    
    Ok(())
    

}
async fn send_chunk_to_enterprise_db(chunk: (Series, Series, Series, Series), db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_analysis_list = Vec::new();
    for i in 0..chunk.0.len() {
        let business_type = chunk.0.get(i).unwrap().to_string().replace("\"", "");
        let sales_volume = chunk.1.get(i).unwrap().to_string().replace("\"", "");
        let credit_score = chunk.2.get(i).unwrap().to_string().replace("\"", "");
        let number_of_companies = match chunk.3.get(i).unwrap() {
            AnyValue::UInt32(val) => val.clone() as i32,
            _ => 0,
        };
        
        let new_analysis = us_business_credit_analysis::ActiveModel {
            business_type: Set(business_type),
            sales_volume: Set(sales_volume),
            credit_score: Set(credit_score),
            number_of_companies: Set(number_of_companies),
            dataset: Set("us business credit analysis".to_string()),
            ..Default::default()
        };
    
      new_analysis_list.append(&mut vec![new_analysis]);
    }
    let res = us_business_credit_analysis::Entity::insert_many(new_analysis_list)
    .exec(db)
    .await?;

    Ok(())
}
