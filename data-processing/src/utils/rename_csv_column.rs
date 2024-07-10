use std::fs;
use std::path::Path;
use csv::Writer;
use sea_orm::ColIdx;

fn rename_column_in_file(file_path: &Path) -> Result<(), csv::Error> {
    // Read the CSV file into memory
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut records = rdr.records().collect::<Result<Vec<_>, _>>()?;
    println!("{:?}",&file_path );

    // Get headers and find the column to rename
    let headers = rdr.headers()?;
    let mut new_headers: Vec<String> = headers.iter().map(|header| {
        if header == "Reveneu" {
            "Revenue".to_string()
        } else {
            header.to_string()
        }
    }).collect();



    // Write the modified records back to the file
    let mut wtr = Writer::from_path(file_path)?;
    wtr.write_record(&new_headers)?;
    for record in records.iter() {
        wtr.write_record(record)?;
    }

    Ok(())
}

pub fn run_clean_up_of_columns() {
    let folder_path = "./data/zoom_info_leads";  // Replace with your folder path

    // Iterate over each file in the folder
    for entry in fs::read_dir(folder_path).unwrap() {
        println!("{:?}",&entry );
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension() == Some("csv".as_ref()) {
            if let Err(err) = rename_column_in_file(&path) {
                eprintln!("Failed to process file {}: {:?}", path.display(), err);
            }
        }
    }
}
