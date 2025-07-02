use std::{path::Path, str::FromStr};

use anyhow::Error;
use clap::{Parser, Subcommand, arg};

#[derive(Debug, Subcommand)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct EncodeOpts {
    #[arg(long, value_parser = validate_input_exists)]
    pub input: String,

    #[arg(long, default_value = "standard")]
    pub format: Format,
}

#[derive(Debug, Parser)]
pub struct DecodeOpts {
    #[arg(long, value_parser = validate_input_exists)]
    pub input: String,

    #[arg(long, default_value = "standard")]
    pub format: Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Standard,
    UrlSafe,
}

impl FromStr for Format {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Format::Standard),
            "urlsafe" => Ok(Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

fn validate_input_exists(input: &str) -> Result<String, String> {
    if input == "-" || Path::new(input).exists() {
        Ok(input.to_string())
    } else {
        Err(format!("Input file {} does not exist", input))
    }
}
