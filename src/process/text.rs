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
use crate::process::gen_pass::gen_pass;

pub fn generate_key(opts: GenerateKeyOpts) -> Result<String> {
    let result = opts.format.gen_key(&opts.output)?;
    Ok(result)
}

pub fn sign(sign_opts: SignOpts) -> Result<String> {
    let signature = sign_opts
        .format
        .sign(&sign_opts.input, &sign_opts.private_key)?;
    Ok(signature)
}

pub fn verify(verify_opts: VerifyOpts) -> Result<bool> {
    let result = verify_opts.format.verify(
        &verify_opts.input,
        &verify_opts.signature,
        &verify_opts.public_key,
    )?;
    Ok(result)
}

trait Sign {
    fn sign(&self, input: &str, key: &Option<String>) -> Result<String>;
}

impl Sign for SignFormat {
    fn sign(&self, input: &str, key: &Option<String>) -> Result<String> {
        match &self {
            SignFormat::ED25519 => {
                let private_key = key.as_ref().expect("ED25519 must provide a private key");
                let key_pem = fs::read_to_string(private_key)?;
                let mut signing_key = SigningKey::from_pkcs8_pem(&key_pem).expect("get key error");
                let signature = signing_key.sign(input.as_bytes());
                Ok(signature.to_string())
            }
            SignFormat::BLAKE3 => {
                let hash = blake3::hash(input.as_bytes());
                Ok(hash.to_string())
            }
        }
    }
}

trait Verify {
    fn verify(&self, input: &str, signature: &str, key: &Option<String>) -> Result<bool>;
}

impl Verify for SignFormat {
    fn verify(&self, input: &str, signature: &str, key: &Option<String>) -> Result<bool> {
        match &self {
            SignFormat::ED25519 => {
                let public_key = key.as_ref().expect("ED25519 must provide public key");
                let key_pem = fs::read_to_string(public_key)?;
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
            SignFormat::BLAKE3 => {
                let hash = blake3::hash(input.as_bytes());
                let verify_result = hash.to_string() == signature;
                Ok(verify_result)
            }
        }
    }
}

trait GenKey {
    fn gen_key(&self, output: &str) -> Result<String>;
}

impl GenKey for SignFormat {
    fn gen_key(&self, output: &str) -> Result<String> {
        let dir = Path::new(&output);
        match &self {
            SignFormat::BLAKE3 => {
                let key_path = dir.join("key");
                let key = gen_pass(32, true, true, true, true)?;
                fs::write(&key_path, key)?;
                Ok(format!("key: {}", &key_path.display()))
            }
            SignFormat::ED25519 => {
                let private_path = dir.join("private_pem");
                let public_path = dir.join("public_pem");

                let mut csprng = OsRng.unwrap_err();
                let signing_key = SigningKey::generate(&mut csprng);
                let pem = signing_key
                    .to_pkcs8_pem(LineEnding::LF)
                    .expect("Failed to convert to PEM");
                fs::write(&private_path, pem)?;

                let verify_key = signing_key.verifying_key();
                let public_pem = verify_key
                    .to_public_key_pem(LineEnding::LF)
                    .expect("Failed to convert to PEM");
                fs::write(&public_path, public_pem)?;
                Ok(format!(
                    "private key: {}\npublic key: {}",
                    &private_path.display(),
                    &public_path.display()
                ))
            }
        }
    }
}
