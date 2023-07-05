use std::path::Path;

use git2::Repository;

use crate::config::repo::SerializedRepository;

pub struct GitRepo {
    pub seralized_data: SerializedRepository,
    pub git2_repository: Repository,
}

impl GitRepo {
    pub fn open_repo(path: &Path) -> Result<Self, git2::Error> {
        let repo = Repository::open(path)?;
        let git_repo = Self::from_git2_repo(repo);
        Ok(git_repo)
    }

    pub fn from_git2_repo(repo: Repository) -> Self {
        let git_repo = Self {
            seralized_data: SerializedRepository {
                path: repo.path().to_path_buf(),
                name: get_repo_name(repo.path()),
                repo_url: get_repo_url(&repo).unwrap_or_default(),
            },
            git2_repository: repo,
        };

        git_repo
    }

    pub fn from_serialized_repo(repo: SerializedRepository) -> Result<Self, git2::Error> {
        let git2_repo = Repository::open(&repo.path)?;

        let git_repo = Self {
            seralized_data: repo,
            git2_repository: git2_repo,
        };

        Ok(git_repo)
    }
}

fn get_repo_name(path: &Path) -> String {
    let path_str = path.to_string_lossy();
    let folders: Vec<&str> = path_str.split('/').collect();
    let name = folders
        .get(folders.len() - 2)
        .unwrap_or(&"UNNAMED")
        .to_string();
    name
}

fn get_repo_url(repo: &Repository) -> Option<String> {
    let remote = repo.find_remote("origin").ok()?;

    let url = remote.url()?.to_owned();

    Some(url)
}
