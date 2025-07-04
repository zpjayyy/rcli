mod cli;
mod process;

pub use process::base64::decode;
pub use process::base64::encode;
pub use process::csv_convert::process_csv;
pub use process::gen_pass::gen_password;

pub use cli::base64::Base64SubCommand;
pub use cli::csv::CsvSubCommand;
pub use cli::csv::OutputFormat;
pub use cli::gen_pass::GenPassSubCommand;
pub use cli::opts::Opts;
pub use cli::opts::SubCommand;

pub use cli::base64::DecodeOpts;
pub use cli::base64::EncodeOpts;
