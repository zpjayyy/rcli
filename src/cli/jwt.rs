use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum JwtSubCommand {
    #[command(name = "sign", about = "Sign a JWT")]
    Sign(SignOpts),
    #[command(name = "verify", about = "Verify a JWT")]
    Verify(VerifyOpts),
}

#[derive(Debug, Parser)]
pub struct SignOpts {
    #[arg(long)]
    pub sub: String,

    #[arg(long)]
    pub aud: String,

    #[arg(long)]
    pub exp: String,
}

#[derive(Debug, Parser)]
pub struct VerifyOpts {}
