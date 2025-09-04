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
    #[arg(long, help = "Subject (user ID)")]
    pub sub: String,

    #[arg(long, help = "Audience")]
    pub aud: Option<String>,

    #[arg(long, help = "Expiration time in seconds from now")]
    pub exp: u64,

    #[arg(long, help = "Issuer")]
    pub iss: Option<String>,

    #[arg(long, help = "Issued at time (default: now)")]
    pub iat: Option<u64>,

    #[arg(long, help = "Secret key for signing")]
    pub secret: String,
}

#[derive(Debug, Parser)]
pub struct VerifyOpts {
    #[arg(long, help = "JWT token to verify")]
    pub token: String,

    #[arg(long, help = "Secret key for verification")]
    pub secret: String,
}
