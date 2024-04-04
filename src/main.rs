mod git;
mod util;

use duct::cmd;
use std::io::{self};
use crate::util::run_command;

fn main() -> io::Result<()> {
    let username = git::read_gitconfig()?;
    let destination_repo = run_command("git", vec!["remote", "-v"])?;

    println!("---\nRun protected push");
    println!("---\nYou are: {}", username);
    println!("---\nDestination repos are:");
    cmd!("git", "remote", "-v").run()?;


    let check_result = util::check_username_in_repo(&username, &destination_repo);
    match check_result {
        true => {
            println!("---\nLooks safe to push, pushing...\n---");
            cmd!("git", "push").run()?;

        }
        false => {
            println!("---\nYou are pushing into not your own repo, ABORT.");
        }
    }

    Ok(())
}
