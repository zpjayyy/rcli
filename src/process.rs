use std::fs::{self};

use anyhow::Result;

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = csv::Reader::from_path(input)?;
    let headers = reader.headers()?.clone();

    let mut ret = Vec::new();

    for record in reader.records() {
        let record = record?;
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(json_value);
    }

    let content = serde_json::to_string(&ret)?;
    fs::write(output, content)?;
    Ok(())
}
