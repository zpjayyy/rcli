use clap::{Parser, Subcommand};
use std::path::Path;

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

        #[arg(short, long, default_value = "output.json")]
        output: String,

        #[arg(long, default_value = "false")]
        header: bool,

        #[arg(short, long, default_value = ",")]
        delimiter: String,
    },
}

fn validate_file_exists(file_path: &str) -> Result<String, String> {
    if Path::new(&file_path).exists() {
        Ok(file_path.to_string())
    } else {
        Err(format!("File {} does not exist", file_path))
    }
}
