use std::str;

use std::process::{Command};

pub fn run_command(_cmd_: &str, _args_: Vec<&str>) -> Result<String, std::io::Error> {
    let mut _command_ = Command::new(_cmd_);

    for arg in _args_ {
        _command_.arg(arg);
    }

    let _output_ = _command_.output()?;

    if _output_.status.success() {
        let _output_str_ = str::from_utf8(&_output_.stdout).unwrap();
        Ok(_output_str_.to_string())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "command failed"))
    }
}


pub fn check_username_in_repo(_username_: &str, _destination_repo_: &str) -> bool {
    _destination_repo_.contains(_username_)
}
