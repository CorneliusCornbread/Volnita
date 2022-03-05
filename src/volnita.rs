use git2::Repository;

pub fn start() {
    let repo = match Repository::open("/path/to/a/repo") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
}