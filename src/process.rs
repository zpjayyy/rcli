use std::fs::{self};

use anyhow::Result;

use crate::opts::OutputFormat;

pub fn process_csv(input: &str, output: &str, output_format: OutputFormat) -> Result<()> {
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

    let file_path = format!("{}.{}", output, output_format);

    let content = match output_format {
        OutputFormat::Json => serde_json::to_string(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        _ => return Err(anyhow::anyhow!("Unsupported output format")),
    };
    fs::write(file_path, content)?;
    Ok(())
}
