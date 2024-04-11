use std::io::{self, BufRead};
use std::str;
use std::fs::File;
use std::path::PathBuf;

pub fn read_gitconfig() -> io::Result<String> {
    let _home_dir_ = dirs::home_dir().expect("Home directory not found");
    let mut _gitconfig_path_ = PathBuf::from(_home_dir_);
    _gitconfig_path_.push(".gitconfig");

    let _display_ = _gitconfig_path_.display();

    let _file_ = match File::open(&_gitconfig_path_) {
        Err(why) => panic!("couldn't open {}: {}", _display_, why),
        Ok(file) => file,
    };

    let _reader_ = io::BufReader::new(_file_);
    let mut _username_ = String::new();

    for line in _reader_.lines() {
        let line = line?;
        if line.contains("name") {
            let parts: Vec<&str> = line.split('=').collect();
            if parts[0].trim() == "name" {
                _username_ = parts[1].trim().to_string();
                break;
            }
        }
    }

    Ok(_username_)
}


