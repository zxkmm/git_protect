use std::str;

use std::process::{Command};

pub fn run_command(cmd: &str, args: Vec<&str>) -> Result<String, std::io::Error> {
    let mut command = Command::new(cmd);

    for arg in args {
        command.arg(arg);
    }

    let output = command.output()?;

    if output.status.success() {
        let output_str = str::from_utf8(&output.stdout).unwrap();
        Ok(output_str.to_string())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "command failed"))
    }
}


pub fn check_username_in_repo(username: &str, destination_repo: &str) -> bool {
    destination_repo.contains(username)
}
