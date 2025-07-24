use anyhow::{Ok, Result};
use rand::seq::{IndexedRandom, SliceRandom};
use zxcvbn::zxcvbn;

use crate::GenPassSubCommand;

const LOWERCASE: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZ";
const NUMBER: &[u8] = b"2345678";
const SYMBOL: &[u8] = b"!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

pub fn gen_password(opts: GenPassSubCommand) -> Result<String> {
    let result = gen_pass(
        opts.length,
        opts.lowercase,
        opts.uppercase,
        opts.number,
        opts.symbol,
    )?;
    Ok(result)
}

pub fn gen_pass(
    length: u8,
    lowercase: bool,
    uppercase: bool,
    number: bool,
    symbol: bool,
) -> Result<String> {
    let mut rng = rand::rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if lowercase {
        chars.extend_from_slice(LOWERCASE);
    }
    if uppercase {
        chars.extend_from_slice(UPPERCASE);
    }
    if number {
        chars.extend_from_slice(NUMBER);
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
    }
    for _ in 0..length {
        if let Some(&c) = chars.choose(&mut rng) {
            password.push(c);
        }
    }
    password.shuffle(&mut rng);
    let pass_str = String::from_utf8(password)?;
    let estimate = zxcvbn(&pass_str, &[]);
    println!("{}, {}", pass_str, estimate.score());
    Ok(pass_str)
}
