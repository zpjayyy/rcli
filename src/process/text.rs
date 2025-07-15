use anyhow::{Ok, Result};
use ed25519_dalek::ed25519::signature::SignerMut;
use ed25519_dalek::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey};
use ed25519_dalek::{Signature, VerifyingKey};
use ed25519_dalek::{SigningKey, pkcs8::spki::der::pem::LineEnding};
use rand::{TryRngCore, rngs::OsRng};
use std::fs;
use std::path::Path;

use crate::GenerateKeyOpts;
use crate::cli::text::{SignFormat, SignOpts, VerifyOpts};

pub fn generate_key(opts: GenerateKeyOpts) -> Result<SigningKey> {
    let base = Path::new(&opts.output);
    let private_path = base.with_file_name("private_pem");
    let public_path = base.with_file_name("public_pem");

    let mut csprng = OsRng.unwrap_err();
    let signing_key = SigningKey::generate(&mut csprng);
    let pem = signing_key
        .to_pkcs8_pem(LineEnding::LF)
        .expect("Failed to convert to PEM");
    fs::write(private_path, pem)?;

    let verify_key = signing_key.verifying_key();
    let public_pem = verify_key
        .to_public_key_pem(LineEnding::LF)
        .expect("Failed to convert to PEM");
    fs::write(public_path, public_pem)?;
    Ok(signing_key)
}

pub fn sign(sign_opts: SignOpts) -> Result<String> {
    let signature = sign_opts.format.sign(&sign_opts.input, &sign_opts.key)?;
    Ok(signature)
}

pub fn verify(verify_opts: VerifyOpts) -> Result<bool> {
    let result =
        verify_opts
            .format
            .verify(&verify_opts.input, &verify_opts.signature, &verify_opts.key)?;
    Ok(result)
}

trait Sign {
    fn sign(&self, input: &str, key: &str) -> Result<String>;
}

impl Sign for SignFormat {
    fn sign(&self, input: &str, key: &str) -> Result<String> {
        match &self {
            SignFormat::ED25519 => {
                let key_pem = fs::read_to_string(key)?;
                let mut signing_key = SigningKey::from_pkcs8_pem(&key_pem).expect("get key error");
                let signature = signing_key.sign(input.as_bytes());
                Ok(signature.to_string())
            }
            SignFormat::BLAKE3 => todo!(),
        }
    }
}

trait Verify {
    fn verify(&self, input: &str, signature: &str, key: &str) -> Result<bool>;
}

impl Verify for SignFormat {
    fn verify(&self, input: &str, signature: &str, key: &str) -> Result<bool> {
        match &self {
            SignFormat::ED25519 => {
                let key_pem = fs::read_to_string(key)?;
                let verify_key =
                    VerifyingKey::from_public_key_pem(&key_pem).expect("get key error");
                let sign_bytes = hex::decode(signature)?;
                let sign_array: [u8; 64] = sign_bytes
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("invalid signature length"))?;
                let signature = Signature::from_bytes(&sign_array);
                let result = verify_key
                    .verify_strict(input.as_bytes(), &signature)
                    .is_ok();
                Ok(result)
            }
            SignFormat::BLAKE3 => todo!(),
        }
    }
}
