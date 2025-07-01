use clap::{Parser, arg};

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value = "16")]
    pub length: u8,

    #[arg(long, default_value = "true")]
    pub uppercase: bool,

    #[arg(long, default_value = "true")]
    pub lowercase: bool,

    #[arg(long, default_value = "true")]
    pub number: bool,

    #[arg(long, default_value = "true")]
    pub symbol: bool,
}
