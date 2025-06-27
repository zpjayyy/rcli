use anyhow::Result;
use rand::seq::{IndexedRandom, SliceRandom};
use zxcvbn::zxcvbn;

use crate::opts::GenPassOpts;

const LOWERCASE: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZ";
const NUMBER: &[u8] = b"2345678";
const SYMBOL: &[u8] = b"!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

pub fn gen_password(opts: GenPassOpts) -> Result<()> {
    let mut rng = rand::rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if opts.lowercase {
        chars.extend_from_slice(LOWERCASE);
    }
    if opts.uppercase {
        chars.extend_from_slice(UPPERCASE);
    }
    if opts.number {
        chars.extend_from_slice(NUMBER);
    }
    if opts.symbol {
        chars.extend_from_slice(SYMBOL);
    }
    for _ in 0..opts.length {
        if let Some(&c) = chars.choose(&mut rng) {
            password.push(c);
        }
    }
    password.shuffle(&mut rng);
    let pass_str = String::from_utf8(password)?;
    let estimate = zxcvbn(&pass_str, &[]);
    println!("{}, {}", pass_str, estimate.score());
    Ok(())
}
