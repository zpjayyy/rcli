use std::fs::{self};

use anyhow::{Ok, Result};

use crate::{CsvSubCommand, OutputFormat};

pub fn process_csv(opts: CsvSubCommand) -> Result<()> {
    let mut reader = csv::Reader::from_path(opts.input)?;
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

    let file_path = format!("{}.{}", opts.output, opts.output_format);

    let content = match opts.output_format {
        OutputFormat::Json => serde_json::to_string(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        _ => return Err(anyhow::anyhow!("Unsupported output format")),
    };
    fs::write(file_path, content)?;
    Ok(())
}
