use clap::{Parser, Subcommand};
use std::path::Path;

use super::base64::Base64SubCommand;
use super::csv::CsvSubCommand;
use super::gen_pass::GenPassSubCommand;
use super::http::HttpSubCommand;
use super::jwt::JwtSubCommand;
use super::text::TextSubCommand;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub option: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[command(name = "csv", about = "CSV operations")]
    Csv(CsvSubCommand),
    #[command(name = "gen-pass", about = "Generate a random password")]
    GenPass(GenPassSubCommand),
    #[command[subcommand]]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
    #[command(subcommand)]
    Jwt(JwtSubCommand),
    #[command(subcommand)]
    Http(HttpSubCommand),
}

pub fn validate_file_exists(file_path: &str) -> Result<String, String> {
    if Path::new(&file_path).exists() {
        Ok(file_path.to_string())
    } else {
        Err(format!("File {} does not exist", file_path))
    }
}
