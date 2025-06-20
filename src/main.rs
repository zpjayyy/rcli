// rcli csv -i input.csv -o output.json  --header -d ";"

use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    option: Options,
}

#[derive(Debug, Subcommand)]
enum Options {
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

#[derive(Serialize, Deserialize, Debug)]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

fn validate_file_exists(file_path: &str) -> Result<String, String> {
    if Path::new(&file_path).exists() {
        Ok(file_path.to_string())
    } else {
        Err(format!("File {} does not exist", file_path))
    }
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);

    match args.option {
        Options::Csv { input, .. } => {
            read_csv(&input).unwrap();
        }
    }
}

fn read_csv(file_path: &str) -> Result<()> {
    let mut reader = csv::Reader::from_path(file_path)?;
    let result: Vec<Player> = reader
        .deserialize::<Player>()
        .collect::<Result<Vec<Player>, csv::Error>>()?;
    println!("{:?}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
