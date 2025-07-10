use clap::{Parser, Subcommand};

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
    #[arg(long, default_value = "-")]
    pub input: String,

    #[arg(long)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct VerifyOpts {
    #[arg(long, default_value = "-")]
    pub input: String,

    #[arg(long)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct GenerateKeyOpts {
    #[arg(short, long)]
    pub output: String,
}
