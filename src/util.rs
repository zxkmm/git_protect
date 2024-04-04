use std::str;

use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

pub fn run_command(cmd: &str, args: Vec<&str>) -> std::io::Result<()> {
    let mut command = Command::new(cmd);

    for arg in args {
        command.arg(arg);
    }

    command.stdout(Stdio::piped());
    let mut child = command.spawn()?;
    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        println!("{}", line?);
    }

    let status = child.wait()?;

    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "command failed"))
    }
}


pub fn check_username_in_repo(username: &str, destination_repo: &str) -> bool {
    destination_repo.contains(username)
}