use std::io;

use git2::Repository;

pub fn start() {
    println!("Input path to repo");

    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("failed to readline");

    let len = path.trim_end_matches(&['\r', '\n'][..]).len();
    path.truncate(len);
    drop(len);

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

    let len = branch_name.trim_end_matches(&['\r', '\n'][..]).len();
    branch_name.truncate(len);
    drop(len);

    let mut remote = repo.find_remote("origin").unwrap();
    remote.connect(git2::Direction::Fetch).unwrap();

    println!("{}", remote.default_branch().unwrap().as_str().unwrap());
}