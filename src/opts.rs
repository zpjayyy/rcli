use anyhow::Error;
use clap::{Parser, Subcommand};
use std::{fmt::Display, path::Path, str::FromStr};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub option: Options,
}

#[derive(Debug, Subcommand)]
pub enum Options {
    Csv {
        #[arg(short, long, value_parser = validate_file_exists)]
        input: String,

        #[arg(short, long, default_value = "output")]
        output: String,

        #[arg(long, default_value = "json")]
        output_format: OutputFormat,

        #[arg(long, default_value = "false")]
        header: bool,

        #[arg(short, long, default_value = ",")]
        delimiter: String,
    },
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

fn validate_file_exists(file_path: &str) -> Result<String, String> {
    if Path::new(&file_path).exists() {
        Ok(file_path.to_string())
    } else {
        Err(format!("File {} does not exist", file_path))
    }
}
