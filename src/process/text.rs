use std::fs;

use anyhow::Result;
use ed25519_dalek::pkcs8::EncodePrivateKey;
use ed25519_dalek::{SigningKey, pkcs8::spki::der::pem::LineEnding};
use rand::{TryRngCore, rngs::OsRng};

use crate::GenerateKeyOpts;

pub fn generate_key(opts: GenerateKeyOpts) -> Result<SigningKey> {
    let mut csprng = OsRng.unwrap_err();
    let signing_key = SigningKey::generate(&mut csprng);
    let pem = signing_key
        .to_pkcs8_pem(LineEnding::default())
        .expect("Failed to convert to PEM");
    fs::write(opts.output, pem)?;
    Ok(signing_key)
}

// trait Sign {}

// trait Verify {}
