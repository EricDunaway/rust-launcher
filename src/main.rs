use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn main() {
    let child = match Command::new("bash")
        .arg("-c")
        .arg("echo 'Hello World'; sleep 0.5; echo 'Bob'; sleep 2;code .")
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
                        out.write("Bob was here\n".as_ref()).unwrap();
                        break;
                    }
                    _ => {
                        out.write((str + "\n").as_ref()).unwrap();
                    }
                },
            }
        }
    }
    out.write("\nExiting...".as_ref()).unwrap();
}
