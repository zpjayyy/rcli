use std::fs;

use anyhow::Result;
use ed25519_dalek::SigningKey;
use rand::{TryRngCore, rngs::OsRng};

use crate::GenerateKeyOpts;

pub fn generate_key(opts: GenerateKeyOpts) -> Result<SigningKey> {
    let mut csprng = OsRng.unwrap_err();
    let signing_key = SigningKey::generate(&mut csprng);
    println!("{:?}", signing_key.to_keypair_bytes());
    fs::write(opts.output, signing_key.to_keypair_bytes())?;
    Ok(signing_key)
}
