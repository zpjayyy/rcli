use std::str::FromStr;

use anyhow::Error;
use clap::{Parser, Subcommand};

use crate::utils::input_util::validate_input_exists;

#[derive(Debug, Subcommand)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "Sign a text")]
    Sign(SignOpts),
    #[command(name = "verify", about = "Verify a text")]
    Verify(VerifyOpts),
    #[command(name = "generate-key", about = "Generate a key")]
    GenerateKey(GenerateKeyOpts),
}

#[derive(Debug, Parser)]
pub struct SignOpts {
    #[arg(long, default_value = "-", value_parser = validate_input_exists)]
    pub input: String,

    #[arg(long)]
    pub private_key: Option<String>,

    #[arg(long)]
    pub format: SignFormat,
}

#[derive(Debug, Parser)]
pub struct VerifyOpts {
    #[arg(long, default_value = "-", value_parser = validate_input_exists)]
    pub input: String,

    #[arg(long)]
    pub signature: Option<String>,

    #[arg(long)]
    pub public_key: Option<String>,

    #[arg(long)]
    pub format: SignFormat,
}

#[derive(Debug, Parser)]
pub struct GenerateKeyOpts {
    #[arg(short, long)]
    pub output: String,
}

#[derive(Debug, Clone, Copy)]
pub enum SignFormat {
    ED25519,
    BLAKE3,
}

impl FromStr for SignFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ed25519" => Ok(SignFormat::ED25519),
            "blake3" => Ok(SignFormat::BLAKE3),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}
