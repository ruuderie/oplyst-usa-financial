use polars::prelude::*;
use crate::{db};
use std::{
    fs::{self, File, OpenOptions},
};
use std::collections::HashMap;
use sea_orm::{entity::prelude::*, Set};

use crate::model::us_business_credit_leads::Entity;
use crate::model::us_business_credit_leads;
use db::postgres;
use sea_orm::entity::prelude::*;
use uuid::Uuid;
use csv::Writer;
use std::collections::HashSet;
use crate::utils::constants::common_email_domains;
use crate::utils::functions::{remove_quotes, remove_quotes_from_str,remove_quotes_from_string};

pub async fn load_business_leads_insights() -> Result<(), Box<dyn std::error::Error>> {
    // Read the data from the file at runtime
    println!("Reading data from file");
    let data_path = "./data/BusinessLeadsUSA.csv";
    let output_file = "./data/business_lead_usa.csv";

    // Process the data and insert into database
    println!("Processing data");
    read_and_process(&data_path, output_file).await
}

async fn read_and_process(
    data_path: &str,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    print!("Reading data");
    let mut df = CsvReader::from_path(data_path)?
    .infer_schema(None)
    .has_header(true)
    .finish()?;
    println!("Finished reading data");
    
    df.align_chunks(); // Rechunking the DataFrame to ensure all chunks are aligned.
    let chunk_size = 1000;
    let num_rows = df.height();
    println!("Number of rows: {}", num_rows);
    println!("connecting to db");
    let db = postgres::establish_connection().await?;

   // let mut wtr = Writer::from_path(output_file)?;
   // wtr.flush()?;
    println!("Processing {} rows", num_rows);
    for i in (0..num_rows).step_by(chunk_size) {
        let end = std::cmp::min(i + chunk_size, num_rows);

        let chunk = df.slice(i.try_into().unwrap(), end - i);
        send_chunk_to_business_credit_directory_db(&chunk, &db).await?;  // Pass the DataFrame chunk and the SeaORM connection
    }

    
    Ok(())
}

async fn send_chunk_to_business_credit_directory_db(
    chunk: &DataFrame,
    db: &DatabaseConnection
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing chunk with {} rows", chunk.height());
    let mut new_business_credit_leads_list: Vec<us_business_credit_leads::ActiveModel> = Vec::new();

    let email_series = chunk.column("EMAIL").unwrap();

    for i in 0..chunk.height() {
        let email_value = email_series.get(i).unwrap().to_string();
        let domain = remove_quotes_from_str(email_value.split('@').last().unwrap_or(""));
        let business_name = remove_quotes(&chunk.column("BUSINESS NAME").unwrap().get(i).unwrap());
        let mailing_address = remove_quotes(&chunk.column("MAILING ADDRESS").unwrap().get(i).unwrap());
        let mailing_city = remove_quotes(&chunk.column("MAILING CITY").unwrap().get(i).unwrap());
        let mailing_state = remove_quotes(&chunk.column("MAILING STATE").unwrap().get(i).unwrap());
        let mailing_zip = remove_quotes(&chunk.column("MAILING ZIP").unwrap().get(i).unwrap());
        let phone = remove_quotes(&chunk.column("AREA CODE AND PHONE").unwrap().get(i).unwrap());
        let fax = remove_quotes(&chunk.column("FAX").unwrap().get(i).unwrap());
        let sales_volume = remove_quotes(&chunk.column("SALES VOLUME").unwrap().get(i).unwrap());
        let number_of_employees = remove_quotes(&chunk.column("NUMBER OF EMPLOYEES").unwrap().get(i).unwrap());
        let public_private_company = remove_quotes(&chunk.column("PUBLIC PRIVATE COMPANY").unwrap().get(i).unwrap());
        let location_type = remove_quotes(&chunk.column("LOCATION TYPE").unwrap().get(i).unwrap());
        let firm_or_individual = remove_quotes(&chunk.column("FIRM INDIVIDUAL").unwrap().get(i).unwrap());
        let credit_score = remove_quotes(&chunk.column("CREDIT SCORE CODE").unwrap().get(i).unwrap());
        let stock_exchange = remove_quotes(&chunk.column("STOCK EXCHANGE").unwrap().get(i).unwrap());
        let stock_ticker_symbol = remove_quotes(&chunk.column("STOCK TICKER SYMBOL").unwrap().get(i).unwrap());
        let website = remove_quotes(&chunk.column("WEB ADDRESS").unwrap().get(i).unwrap());
        let first_name = remove_quotes(&chunk.column("FIRSTNAME").unwrap().get(i).unwrap());
        let last_name = remove_quotes(&chunk.column("LASTNAME").unwrap().get(i).unwrap());
        let title = remove_quotes(&chunk.column("TITLE").unwrap().get(i).unwrap());
        let industry = remove_quotes(&chunk.column("SIC NAME1").unwrap().get(i).unwrap());
        let sic = remove_quotes(&chunk.column("SIC").unwrap().get(i).unwrap());
        let naics = remove_quotes(&chunk.column("NAICS").unwrap().get(i).unwrap());
        let ncci = remove_quotes(&chunk.column("NCCI").unwrap().get(i).unwrap());
        
        println!("---- Business Info ----");
        println!("Email domain: {}", domain );
        println!("Title: {}", title);
        println!("Business Name: {}", business_name);
        println!("Mailing City: {}", mailing_city);
        println!("Mailing State: {}", mailing_state);
        println!("Website: {}", website);
        println!("Industry: {}", industry);
        
        println!("\n---- Financial Info ----");
        println!("Sales Volume: {}", sales_volume);
        println!("Number of Employees: {}", number_of_employees);
        println!("Credit Score: {}", credit_score);
        println!("Stock Exchange: {}", stock_exchange);
        println!("Stock Ticker Symbol: {}", stock_ticker_symbol);
        
        // Check if email domain is common
        if !common_email_domains().contains(&domain.as_str()) {

            let new_lead = us_business_credit_leads::ActiveModel {
                id: sea_orm::ActiveValue::Set(Uuid::new_v4()),
                business_name: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("BUSINESS NAME").unwrap().get(i).unwrap())),
                mailing_address: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("MAILING ADDRESS").unwrap().get(i).unwrap())),
                mailing_city: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("MAILING CITY").unwrap().get(i).unwrap())),
                mailing_state: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("MAILING STATE").unwrap().get(i).unwrap())),
                mailing_zip: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("MAILING ZIP").unwrap().get(i).unwrap())),
                phone: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("AREA CODE AND PHONE").unwrap().get(i).unwrap())),
                fax: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("FAX").unwrap().get(i).unwrap())),
                sales_volume: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("SALES VOLUME").unwrap().get(i).unwrap())),
                number_of_employees: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("NUMBER OF EMPLOYEES").unwrap().get(i).unwrap())),
                public_private_company: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("PUBLIC PRIVATE COMPANY").unwrap().get(i).unwrap())),
                location_type: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("LOCATION TYPE").unwrap().get(i).unwrap())),
                firm_or_individual: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("FIRM INDIVIDUAL").unwrap().get(i).unwrap())),
                credit_score: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("CREDIT SCORE CODE").unwrap().get(i).unwrap())),
                stock_exchange: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("STOCK EXCHANGE").unwrap().get(i).unwrap())),
                stock_ticker_symbol: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("STOCK TICKER SYMBOL").unwrap().get(i).unwrap())),
                website: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("WEB ADDRESS").unwrap().get(i).unwrap())),
                first_name: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("FIRSTNAME").unwrap().get(i).unwrap())),
                last_name: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("LASTNAME").unwrap().get(i).unwrap())),
                title: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("TITLE").unwrap().get(i).unwrap())),
                email: sea_orm::ActiveValue::Set(remove_quotes_from_string(&email_value)),
                industry: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("SIC NAME1").unwrap().get(i).unwrap())),
                sic: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("SIC").unwrap().get(i).unwrap())),
                naics: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("NAICS").unwrap().get(i).unwrap())),
                ncci: sea_orm::ActiveValue::Set(remove_quotes(&chunk.column("NCCI").unwrap().get(i).unwrap())),
            };
            
            new_business_credit_leads_list.push(new_lead);
        } else {
            println!("Skipping email: {}", email_value);
        }
    }
    println!("Inserting {} rows into us_business_credit_leads table", new_business_credit_leads_list.len());

    us_business_credit_leads::Entity::insert_many(new_business_credit_leads_list).exec(db).await?;

    Ok(())
}


