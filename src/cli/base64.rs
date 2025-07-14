use std::str::FromStr;

use crate::utils::input_util::validate_input_exists;
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
    #[arg(long, default_value = "-", value_parser = validate_input_exists)]
    pub input: String,

    #[arg(long, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct DecodeOpts {
    #[arg(long, default_value = "-", value_parser = validate_input_exists)]
    pub input: String,

    #[arg(long, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl FromStr for Base64Format {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}
