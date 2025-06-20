use std::fs::File;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn read_csv(file_path: &str) -> Result<Vec<Player>> {
    let mut reader = csv::Reader::from_path(file_path)?;
    let result: Vec<Player> = reader
        .deserialize::<Player>()
        .collect::<Result<Vec<Player>, csv::Error>>()?;
    println!("{:?}", result);
    Ok(result)
}

pub fn write_json(data: &[Player], file_path: &str) -> Result<()> {
    let file = File::create(file_path)?;
    serde_json::to_writer_pretty(file, data)?;
    Ok(())
}
