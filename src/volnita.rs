use std::io;

use git2::Repository;
use super::command::git_command;

pub fn start() {
    println!("Input path to repo");

    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("failed to readline");

    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let head = repo.head().unwrap();
    let name = head.name().unwrap();
    println!("{}", name);

    println!("Input name of branch");
    
    let mut branch_name = String::new();
    io::stdin().read_line(&mut branch_name).expect("failed to readline");

    repo.branch_remote_name(&branch_name);
}