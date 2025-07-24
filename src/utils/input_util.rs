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
