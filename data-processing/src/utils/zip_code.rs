// data-processing/src/utils/zip_data.rs

use std::path::Path;
use std::fs::File;
use csv::Reader;

#[derive(Debug, Clone)]
pub struct ZipData {
    pub zip: String,
    pub lat: String,
    pub lng: String,
    pub city: String,
    pub state_id: String,
    pub state_name: String,
    pub zcta: String,
    pub parent_zcta: String,
    pub population: String,
    pub density: String,
    pub county_fips: String,
    pub county_name: String,
    pub county_weights: String,
    pub county_names_all: String,
    pub county_fips_all: String,
    pub imprecise: String,
    pub military: String,
    pub timezone: String
}


impl ZipData {
    pub fn from_csv_record(record: &[String]) -> Option<Self> {
        if record.len() < 5 {  // Just ensuring the required fields are present
            return None;
        }

        Some(Self {
            zip: record[0].clone(),
            lat: record[1].clone(),
            lng: record[2].clone(),
            city: record[3].clone(),
            state_id: record[4].clone(),
            state_name: record[5].clone(),
            zcta: record[6].clone(),
            parent_zcta: record[7].clone(),
            population: record[8].clone(),
            density: record[9].clone(),
            county_fips: record[10].clone(),
            county_name: record[11].clone(),
            county_weights: record[12].clone(),
            county_names_all: record[13].clone(),
            county_fips_all: record[14].clone(),
            imprecise: record[15].clone(),
            military: record[16].clone(),
            timezone: record[17].clone()
        })
    }
    pub fn find_by_zip<'a>(zip: &'a str, zip_data: &'a [ZipData]) -> Option<&'a ZipData>{
                zip_data.iter().find(|&data| data.zip == zip)
    }
    
}
pub fn load_zip_data(path: &Path) -> std::result::Result<Vec<ZipData>, Box<dyn std::error::Error>> {

    let mut rdr = Reader::from_reader(File::open(path)?);
    let zip_data = rdr.records()
        .filter_map(|result| {
            ZipData::from_csv_record(&result.ok()?.iter().map(|s| s.to_string()).collect::<Vec<String>>())
        })
        .collect::<Vec<ZipData>>();
    Ok(zip_data)
}
