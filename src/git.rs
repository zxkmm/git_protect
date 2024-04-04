use std::io::{self, BufRead};
use std::str;
use std::fs::File;
use std::path::PathBuf;

pub fn read_gitconfig() -> io::Result<String> {
    let home_dir = dirs::home_dir().expect("Home directory not found");
    let mut gitconfig_path = PathBuf::from(home_dir);
    gitconfig_path.push(".gitconfig");

    let display = gitconfig_path.display();

    let file = match File::open(&gitconfig_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file);
    let mut username = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains("name") {
            let parts: Vec<&str> = line.split('=').collect();
            if parts[0].trim() == "name" {
                username = parts[1].trim().to_string();
                break;
            }
        }
    }

    Ok(username)
}

