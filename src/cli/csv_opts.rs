use anyhow::Error;
use std::{fmt::Display, str::FromStr};

use clap::{Parser, arg};

use crate::cli::opts::validate_file_exists;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = validate_file_exists)]
    pub input: String,

    #[arg(short, long, default_value = "output")]
    pub output: String,

    #[arg(long, default_value = "json")]
    pub output_format: OutputFormat,

    #[arg(long, default_value = "false")]
    pub header: bool,

    #[arg(short, long, default_value = ",")]
    pub delimiter: String,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Toml,
    Yaml,
}

impl FromStr for OutputFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "toml" => Ok(OutputFormat::Toml),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid output format")),
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            OutputFormat::Json => "json",
            OutputFormat::Toml => "toml",
            OutputFormat::Yaml => "yaml",
        };
        write!(f, "{}", s)
    }
}
