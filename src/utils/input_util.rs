use anyhow::{Ok, Result};
use std::path::Path;

pub fn validate_input_exists(input: &str) -> Result<String> {
    if input == "-" || Path::new(input).exists() {
        Ok(input.to_string())
    } else {
        Err(anyhow::anyhow!("Input file {} does not exist", input))
    }
}

pub fn validate_output_exists(output: &str) -> Result<String> {
    if Path::new(output).exists() {
        Ok(output.to_string())
    } else {
        Err(anyhow::anyhow!("Output path {} does not exist", output))
    }
}

pub fn validate_path(path: &str) -> Result<String> {
    if path == "." || Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err(anyhow::anyhow!(
            "Invalid path: {} (must be a valid directory or relative path)",
            path.to_string()
        ))
    }
}
