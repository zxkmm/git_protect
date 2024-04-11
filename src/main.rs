mod git;
mod util;

use duct::cmd;
// use std::io::{self};
use crate::util::run_command;

use std::env;
use std::io;
use std::process::Command;

fn main() -> io::Result<()> {
    let username = git::read_gitconfig()?;
    let destination_repo = run_command("git", vec!["remote", "-v"])?;

    println!("--------\nProtecting git, you are: {}", username);
    println!("--------\nDestination repos are:");
    cmd!("git", "remote", "-v").run()?;

    let args: Vec<String> = env::args().collect();
    let git_args = args[1..].to_vec();

    let check_result = util::check_username_in_repo(&username, &destination_repo);
    match check_result {
        true => {
            let mut combined_command = Command::new("git");
            combined_command.args(git_args);
            println!("--------\nLooks safe, running: {:?}\n--------", combined_command);
            combined_command.spawn()?.wait()?;
        }
        false => {
            println!("--------\nYou are running in not your own repo, ABORT.");
        }
    }

    Ok(())
}

