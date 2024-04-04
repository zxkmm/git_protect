mod git;
mod util;

use std::io::{self};

fn main() -> io::Result<()> {
    let username = git::read_gitconfig()?;
    let destination_repo = util::run_command("git", vec!["remote", "-v"])?;
    println!("---\nRun protected push");
    println!("---\nYou are: {}", username);
    println!("---\nDestination repos are: \n{}", destination_repo);

    let check_result = util::check_username_in_repo(&username, &destination_repo);
    match check_result {
        true => {
            println!("---\nLooks safe to push, pushing...");
            let push_output = util::run_command("git", vec!["push"])?;
            println!("---\n{}", push_output);

        }
        false => {
            println!("---\nYou are pushing into not your own repo, ABORT.");
        }
    }

    Ok(())
}
