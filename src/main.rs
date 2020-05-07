use std::borrow::Cow;
use std::env::current_dir;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

use crate::config::Config;
use crate::step::{App, Step};

mod config;
mod step;

fn main() -> Result<(), std::io::Error> {
    let child = match Command::new("bash")
        .arg("-c")
        .arg("echo 'Hello World'; sleep 0.5; echo 'Bob'; sleep 2;")
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(process) => process,
    };
    let mut out = io::stdout();
    let bob = "Bob";
    {
        let reader = BufReader::new(child.stdout.unwrap());
        for x in reader.lines() {
            match x {
                Err(why) => panic!("{:?}", why),
                Ok(str) => match str {
                    _ if bob.eq(&str) => {
                        out.write_all("Bob was here\n".as_ref())?;
                        break;
                    }
                    _ => {
                        out.write_all((str + "\n").as_ref())?;
                    }
                },
            }
        }
    }
    let cwd = match current_dir() {
        Ok(path) => Cow::from(String::from(path.to_str().unwrap())),
        _ => unimplemented!(),
    };

    let mut config = Config::default();
    config.step.push(Step {
        label: Cow::from("Step 1"),
        app: vec![
            App {
                cwd: Cow::Borrowed(&*cwd),
                command: Cow::from("echo"),
                arguments: vec![Cow::from("Hello World")],
            },
            App {
                cwd: Cow::Borrowed(&*cwd),
                command: Cow::from("echo"),
                arguments: vec![Cow::from("Hello Bob")],
            },
        ],
    });

    config.step.push(Step {
        label: Cow::from("Step 2"),
        app: vec![App {
            cwd: Cow::Borrowed(&*cwd),
            command: Cow::from("echo"),
            arguments: vec![Cow::from("Hi Bob")],
        }],
    });

    let mut file = File::create(cwd.clone().into_owned() + "/config.toml")?;
    file.write_all(toml::to_string(&config).unwrap().as_bytes())?;
    file.sync_all()?;

    out.write_all(format!("\n{}\n", cwd).as_bytes())?;
    out.write_all("\nExiting...".as_ref())?;
    Ok(())
}
