use clap::{Parser, Subcommand};
use std::path::Path;

use super::csv_opts::CsvOpts;
use super::gen_pass_opts::GenPassOpts;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub option: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Csv(CsvOpts),
    GenPass(GenPassOpts),
}

pub fn validate_file_exists(file_path: &str) -> Result<String, String> {
    if Path::new(&file_path).exists() {
        Ok(file_path.to_string())
    } else {
        Err(format!("File {} does not exist", file_path))
    }
}
