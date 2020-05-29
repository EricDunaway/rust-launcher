use crate::exec::Executor;
use std::env::current_dir;
use std::fs::read_to_string;

mod config;
mod exec;
mod message;
mod step;

fn main() -> Result<(), std::io::Error> {
    let x;
    {
        let s = read_to_string(current_dir()?.to_str().unwrap().to_owned() + "/config.toml")?;
        x = Executor::load(&toml::from_str(s.as_ref())?);
    }

    println!("{}", x.steps[0].label);
    Ok(())
}
