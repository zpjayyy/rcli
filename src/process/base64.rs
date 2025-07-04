use std::{
    fs::File,
    io::{self, Read},
};

use crate::{DecodeOpts, EncodeOpts, cli::base64::Base64Format};
use anyhow::Result;
use base64::{
    Engine,
    prelude::{BASE64_STANDARD, BASE64_URL_SAFE},
};

pub fn encode(opts: EncodeOpts) -> Result<String> {
    let buffer = _read_to_buffer(&opts.input)?;
    let encoded = BASE64_STANDARD.encode(buffer);
    Ok(encoded.to_string())
}

pub fn decode(opts: DecodeOpts) -> Result<String> {
    let buffer = _read_to_buffer(&opts.input)?;
    let decoded = match opts.format {
        Base64Format::Standard => BASE64_STANDARD.decode(buffer)?,
        Base64Format::UrlSafe => BASE64_URL_SAFE.decode(buffer)?,
    };
    Ok(String::from_utf8(decoded)?)
}

fn _read_to_buffer(input: &str) -> Result<Vec<u8>> {
    let mut reader = _get_reader(input)?;
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn _get_reader(input: &str) -> Result<Box<dyn Read>> {
    if input == "-" {
        Ok(Box::new(io::stdin()))
    } else {
        Ok(Box::new(File::open(input)?))
    }
}
