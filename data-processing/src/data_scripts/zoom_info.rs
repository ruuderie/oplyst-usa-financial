use polars::prelude::*;
use crate::{db};
use std::fs::{self, File, OpenOptions};
use std::collections::HashMap;
use sea_orm::{entity::prelude::*, Set};
use crate::model::us_zoom_info::{Entity as ZoomInfoEntity, self};
use db::postgres;
use uuid::Uuid;
use anyhow::{Result, Error};
use polars::export::chrono;
use chrono::Utc;
use crate::utils::constants::common_email_domains;
use crate::utils::functions::remove_quotes_from_anyvalue;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
pub async fn load_zoom_info_insights() -> anyhow::Result<()> {
    println!("Reading data from files");

    // Specified folder path
    let folder_path = "./data/zoom_info_leads";
    let output_file = "./data/test_zoom_info_usa.csv";

    // Collect all CSV files from the specified folder
    let files: Vec<_> = fs::read_dir(folder_path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() && path.extension()? == "csv" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    println!("Found {} files to process.", files.len());

    // Process files one by one
    for file in files {
        let file_str = file.to_str().ok_or_else(|| anyhow::anyhow!("Invalid file path"))?;
        println!("Processing file: {}", file_str);

        // Directly call the processing function for each file
        read_and_process(file_str, output_file).await?;
    }

    println!("All files have been processed.");
    Ok(())
}




async fn read_and_process(
    data_path: &str,
    output_file: &str,
) -> Result<()> {
    println!("Reading data");
    let mut df = CsvReader::from_path(data_path)?
        .infer_schema(None)
        .has_header(true)
        .finish()?;
    println!("Finished reading data");

    df.align_chunks(); // Rechunking the DataFrame to ensure all chunks are aligned.

    let chunk_size = 2750;
    let num_rows = df.height();
    println!("Number of rows: {}", num_rows);

    // Create a channel for sending DataFrame chunks to worker tasks
    let (tx, rx) = mpsc::channel::<DataFrame>(32);
    let rx = Arc::new(Mutex::new(rx)); // Shared receiver

    // Spawn worker tasks
    let num_workers = 12; // determine based on hardware concurrency
    let mut handles = Vec::with_capacity(num_workers);
    for _ in 0..num_workers {
        let rx = Arc::clone(&rx); // Clone for each worker
        let db = postgres::establish_connection().await?; // Each worker establishes its own DB connection
        let data_path = data_path.to_string(); // Clone data_path for each worker

        let handle = tokio::spawn(async move {
            while let Some(chunk) = {
                let mut locked_rx = rx.lock().await;
                locked_rx.recv().await
            } {
                send_chunk_to_zoom_info_db(&chunk, &db, &data_path).await?; // Process each chunk
            }
            Ok::<(), anyhow::Error>(())
        });
        handles.push(handle);
    }

    // Distribute DataFrame chunks to workers
    for i in (0..num_rows).step_by(chunk_size) {
        let end = std::cmp::min(i + chunk_size, num_rows);
        let chunk = df.slice(i.try_into().unwrap(), end - i);
        tx.send(chunk).await?; // Send each chunk to the channel for processing by workers
    }

    // Drop the sender to close the channel, signaling the workers to exit after processing all chunks
    drop(tx);

    // Wait for all worker tasks to complete
    for handle in handles {
        handle.await??; // Handle errors appropriately
    }

    println!("All chunks have been processed.");
    Ok(())
}

async fn send_chunk_to_zoom_info_db(
    chunk: &DataFrame,
    db: &DatabaseConnection,
    file_name: &str
) -> anyhow::Result<()> {
    println!("Processing chunk with {:?} rows in {:?}", chunk.height(), file_name);
    let mut new_zoom_info_list = Vec::new();
    for i in 0..chunk.height() {
        let email = chunk.column("Email")
            .and_then(|col| col.get(i))
            .and_then(|val| Ok(remove_quotes_from_anyvalue(&val)))
            .unwrap_or_else(|_| "".to_string());

        let domain = chunk.column("Domain")
            .and_then(|col| col.get(i))
            .and_then(|val| Ok(remove_quotes_from_anyvalue(&val)))
            .unwrap_or_else(|_| {
                // Extract domain from email if email is not blank
                if !email.is_empty() {
                    email.split('@').last().unwrap_or("").to_string()
                } else {
                    "".to_string()
                }
            });
        if !common_email_domains().contains(&domain.to_string().as_str()) {

            let pattern = chunk.column("Pattern")
                .and_then(|col| col.get(i))
                .and_then(|val| Ok(remove_quotes_from_anyvalue(&val)))
                .unwrap_or_else(|_| "".to_string());
            let first_name = remove_quotes_from_anyvalue(&chunk.column("First Name").unwrap().get(i).unwrap());
            let middle_name =   remove_quotes_from_anyvalue(&chunk.column("Middle Name").unwrap().get(i).unwrap());
            let last_name =remove_quotes_from_anyvalue(&chunk.column("Last Name").unwrap().get(i).unwrap());
            let title = remove_quotes_from_anyvalue(&chunk.column("Title").unwrap().get(i).unwrap());
            let company_name = remove_quotes_from_anyvalue(&chunk.column("Company Name").unwrap().get(i).unwrap());
            let mailing_address = remove_quotes_from_anyvalue(&chunk.column("Mailing Address").unwrap().get(i).unwrap());
            let primary_city = remove_quotes_from_anyvalue(&chunk.column("Primary City").unwrap().get(i).unwrap());
            let primary_state = remove_quotes_from_anyvalue(&chunk.column("Primary State").unwrap().get(i).unwrap());
            let zip_code = remove_quotes_from_anyvalue(&chunk.column("ZIP Code").unwrap().get(i).unwrap());
            let country = remove_quotes_from_anyvalue(&chunk.column("Country").unwrap().get(i).unwrap());
            let phone = remove_quotes_from_anyvalue(&chunk.column("Phone").unwrap().get(i).unwrap());
            let website = remove_quotes_from_anyvalue(&chunk.column("Web Address").unwrap().get(i).unwrap());
            let revenue = remove_quotes_from_anyvalue(&chunk.column("Revenue").unwrap().get(i).unwrap());
            let employee = remove_quotes_from_anyvalue(&chunk.column("Employee").unwrap().get(i).unwrap());
            let industry =remove_quotes_from_anyvalue(&chunk.column("Industry").unwrap().get(i).unwrap());
            let sub_industry = remove_quotes_from_anyvalue(&chunk.column("Sub Industry").unwrap().get(i).unwrap());

            let new_lead = us_zoom_info::ActiveModel {
                id: sea_orm::ActiveValue::Set(Uuid::new_v4()),
                domain: sea_orm::ActiveValue::Set(domain.to_string()),
                pattern: sea_orm::ActiveValue::Set(pattern.to_string()),
                first_name: sea_orm::ActiveValue::Set(first_name.to_string()),
                middle_name: sea_orm::ActiveValue::Set(Some(middle_name.to_string())),
                last_name: sea_orm::ActiveValue::Set(last_name.to_string()),
                title: sea_orm::ActiveValue::Set(title.to_string()),
                company_name: sea_orm::ActiveValue::Set(company_name.to_string()),
                mailing_address: sea_orm::ActiveValue::Set(mailing_address.to_string()),
                primary_city: sea_orm::ActiveValue::Set(primary_city.to_string()),
                primary_state: sea_orm::ActiveValue::Set(primary_state.to_string()),
                zip_code: sea_orm::ActiveValue::Set(zip_code.to_string()),
                phone: sea_orm::ActiveValue::Set(phone.to_string()),
                web_address: sea_orm::ActiveValue::Set(Some(website.to_string())),
                email: sea_orm::ActiveValue::Set(email.to_string()),
                revenue: sea_orm::ActiveValue::Set(Some(revenue.to_string())),
                employee: sea_orm::ActiveValue::Set(Some(employee.to_string())),
                country: sea_orm::ActiveValue::Set(country.to_string()),
                industry: sea_orm::ActiveValue::Set(industry.to_string()),
                sub_industry: sea_orm::ActiveValue::Set(Some(sub_industry.to_string())),
                created_date: sea_orm::ActiveValue::Set(Some(Utc::now().naive_utc())),
                file_of_origin: sea_orm::ActiveValue::Set(Some(file_name.to_string()))
            };

            new_zoom_info_list.push(new_lead);
        } else {
            println!("Skipping email: {}", &email.to_string());
        }
    }

    println!("Inserting {} rows into us_zoom_info table", new_zoom_info_list.len());
    ZoomInfoEntity::insert_many(new_zoom_info_list).exec(db).await?;
    Ok(())
}
