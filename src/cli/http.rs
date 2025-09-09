use clap::{Parser, Subcommand};

use crate::utils::input_util::validate_path;

#[derive(Debug, Subcommand)]
pub enum HttpSubCommand {
    #[command(name = "serve", about = "Serve a HTTP server")]
    Serve(ServeOpts),
}

#[derive(Debug, Parser)]
pub struct ServeOpts {
    #[arg(long, value_parser = validate_path, default_value = ".")]
    pub path: String,

    #[arg(long, default_value = "8080")]
    pub port: u16,
}
