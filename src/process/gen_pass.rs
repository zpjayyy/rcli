use anyhow::Result;

use crate::opts::GenPassOpts;

pub fn gen_password(opts: GenPassOpts) -> Result<()> {
    println!("{:?}", opts);
    Ok(())
}
