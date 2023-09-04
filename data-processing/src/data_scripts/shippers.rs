use polars::prelude::*;
use crate::db;
use std::{
    fs::{self, File, OpenOptions},
};
use std::collections::HashMap;
use sea_orm::{entity::prelude::*, Set};

use crate::model::shippers::Entity;
use crate::model::shippers;
use db::postgres;
use sea_orm::entity::prelude::*;
use uuid::Uuid;
use csv::Writer;
use std::collections::HashSet;
use crate::utils::constants::common_email_domains;
use crate::utils::functions::{remove_quotes, remove_quotes_from_string,remove_quotes_from_str};

pub async fn load_shipper_insights() -> Result<(), Box<dyn std::error::Error>> {
    // Read the data from the file at runtime
    let data = fs::read_to_string("./data/LOGISTICS_WAREHOUSES_DISTRIBUTORS - Owners.csv")?;
    let output_file = "./data/shippers_insights_2.csv";

    // Process the data and insert into database
    read_and_process(&data, output_file).await
}

async fn read_and_process(
    data: &str,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut df = CsvReader::new(std::io::Cursor::new(data.as_bytes()))
        .infer_schema(Some(50))
        .finish()?;
    
    df.align_chunks(); // Rechunking the DataFrame to ensure all chunks are aligned.
    let chunk_size = 100;
    let num_rows = df.height();

    let db = postgres::establish_connection().await?;
    let common_domains = common_email_domains();

    let mut wtr = Writer::from_path(output_file)?;
    wtr.flush()?;

    for i in (0..num_rows).step_by(chunk_size) {
        let end = std::cmp::min(i + chunk_size, num_rows);

        let chunk = df.slice(i.try_into().unwrap(), end - i);
        send_chunk_to_shippers_db(&chunk, &db).await?;  // Pass the DataFrame chunk and the SeaORM connection
    }

    
    Ok(())
}


async fn send_chunk_to_shippers_db(
    chunk: &DataFrame,
    db: &DatabaseConnection
) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut new_shippers_list: Vec<shippers::ActiveModel> = Vec::new();

    let num_rows = chunk.height();
    let shipper_type= chunk.column("SHIPPER TYPE").unwrap();
    let company_name= chunk.column("COMPANY")?;
    let street_address= chunk.column("ADDRESS")?;
    let city= chunk.column("CITY")?;
    let state= chunk.column("STATE")?;
    let zip_code= chunk.column("ZIPCODE")?;
    let phone= chunk.column("PHONE")?;
    let fax= chunk.column("FAX")?;
    let sales= chunk.column("SALES")?;
    let industry= chunk.column("PROFILE")?;
    let website= chunk.column("WEBSITE")?;
    let first_name= chunk.column("FIRSTNAME")?;
    let last_name= chunk.column("LASTNAME")?;
    let title= chunk.column("TITLE")?;
    let email_series = chunk.column("EMAIL").unwrap();

    let num_rows = chunk.height();
    let mut new_shippers_list = Vec::new();

    for i in 0..num_rows {
        let email_value = email_series.get(i).unwrap().to_string();
        let domain = remove_quotes_from_str(email_value.split('@').last().unwrap_or(""));
        println!("Email domain: {}", domain );
        // Check if email domain is common
        if !common_email_domains().contains(&domain.as_str()) {

            let new_shipper = shippers::ActiveModel {
                id: sea_orm::ActiveValue::Set(Uuid::new_v4()),
                shipper_type: sea_orm::ActiveValue::Set(remove_quotes(&shipper_type.get(i).unwrap())),
                company_name: sea_orm::ActiveValue::Set(remove_quotes(&company_name.get(i).unwrap())),
                street_address: sea_orm::ActiveValue::Set(remove_quotes(&street_address.get(i).unwrap())),
                city: sea_orm::ActiveValue::Set(remove_quotes(&city.get(i).unwrap())),
                state: sea_orm::ActiveValue::Set(remove_quotes(&state.get(i).unwrap())),
                zip_code: sea_orm::ActiveValue::Set(remove_quotes(&zip_code.get(i).unwrap())),
                phone: sea_orm::ActiveValue::Set(remove_quotes(&phone.get(i).unwrap())),
                fax: sea_orm::ActiveValue::Set(remove_quotes(&fax.get(i).unwrap())),
                sales: sea_orm::ActiveValue::Set(remove_quotes(&sales.get(i).unwrap())),
                industry: sea_orm::ActiveValue::Set(remove_quotes(&industry.get(i).unwrap())),
                website: sea_orm::ActiveValue::Set(remove_quotes(&website.get(i).unwrap())),
                first_name: sea_orm::ActiveValue::Set(remove_quotes(&first_name.get(i).unwrap())),
                last_name: sea_orm::ActiveValue::Set(remove_quotes(&last_name.get(i).unwrap())),
                title: sea_orm::ActiveValue::Set(remove_quotes(&title.get(i).unwrap())),
                email: sea_orm::ActiveValue::Set(remove_quotes_from_string(&email_value)),
            };
            
            new_shippers_list.push(new_shipper);
        } else {
            println!("Skipping email: {}", email_value);
        }
    }
    println!("Inserting {} rows into shippers table", new_shippers_list.len());

    Entity::insert_many(new_shippers_list).exec(db).await?;

    Ok(())
}


pub fn aggregate_email_counts(dir_paths: &[&str]) -> Result<HashMap<String, usize>, Box<dyn std::error::Error>> {
    let mut total_email_counts_per_state: HashMap<String, usize> = HashMap::new();

    for dir_path in dir_paths {
        // Read the CSV into a DataFrame
        let df = CsvReader::from_path(dir_path)?
            .has_header(true)
            .finish()?;

        // Filter rows where EMAIL is not null
        let email_df = df.lazy().filter(col("EMAIL").is_not_null());

        // Group by "STATE" and count the number of email addresses
        let grouped = email_df
            .groupby(vec![col("STATE")])
            .agg(vec![col("EMAIL").count().alias("EMAIL_count")])
            .collect()?;

        // Extract counts per state
        let states = grouped.column("STATE")?.utf8()?;
        let counts = grouped.column("EMAIL_count")?.u32()?;

        for (state_option, count_option) in states.into_iter().zip(counts) {
            if let (Some(state), Some(count)) = (state_option, count_option) {
                *total_email_counts_per_state.entry(state.to_string()).or_insert(0) += count as usize;
            }
        }
        
    }

    Ok(total_email_counts_per_state)
}


