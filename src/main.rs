mod git;
mod util;

use duct::cmd;
use std::io::{self};
use crate::util::run_command;

fn main() -> io::Result<()> {
    let _username_ = git::read_gitconfig()?;
    let _destination_repo_ = run_command("git", vec!["remote", "-v"])?;

    println!("--------\nRun protected push");
    println!("--------\nYou are: {}", _username_);
    println!("--------\nDestination repos are:");
    cmd!("git", "remote", "-v").run()?;


    let _check_result_ = util::check_username_in_repo(&_username_, &_destination_repo_);
    match _check_result_ {
        true => {
            println!("--------\nLooks safe to push, pushing...\n--------");
            cmd!("git", "push").run()?;

        }
        false => {
            println!("--------\nYou are pushing into not your own repo, ABORT.");
        }
    }

    Ok(())
}
