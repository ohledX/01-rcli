use std::fs;

use csv::Reader;
use serde::{Deserialize, Serialize};

use crate::opts::CsvOpts;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Position")]
    pub position: String,

    #[serde(rename = "DOB")]
    pub dob: String,

    #[serde(rename = "Nationality")]
    pub nationality: String,

    #[serde(rename = "Kit Number")]
    pub kit_number: u8,
}

pub fn process_csv(csv_opts: &CsvOpts) -> anyhow::Result<()> {
    let file_path = csv_opts.input.clone();
    let mut file = Reader::from_path(file_path)?;

    let records = file
        .deserialize()
        .map(|r| r.unwrap())
        .collect::<Vec<Player>>();

    let json = serde_json::to_string_pretty(&records)?;
    fs::write(csv_opts.output.clone(), json)?;
    Ok(())
}
