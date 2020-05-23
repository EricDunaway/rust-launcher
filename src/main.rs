#![feature(in_band_lifetimes)]

use crate::config::Config;
use std::env::current_dir;
use std::fs::read_to_string;

mod config;
mod exec;
mod step;

fn main() -> Result<(), std::io::Error> {
    let config: Config;
    {
        config = toml::from_str(&read_to_string(
            current_dir()?.to_str().unwrap().to_owned() + "/config.toml",
        )?)?;
    }

    println!("{}", config.step[0].label);
    Ok(())
}
