use std::{fs, io};

use serde::{Deserialize, Serialize};

use super::{get_config_path, Config};

const FILE_NAME: &str = "saved_repos";
const FILE_NAME_WITH_EXT: &str = "saved_repos.toml";

#[derive(Serialize, Deserialize)]
pub struct Repository {
    pub path: String,
    pub name: String,
    pub repo_url: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SavedRepositories {
    pub recent_repositories: Vec<Repository>,
}

impl Config for SavedRepositories {
    fn load_config() -> Option<SavedRepositories> {
        let mut path = get_config_path()?;
        path.push(FILE_NAME_WITH_EXT);

        let string = fs::read_to_string(path).unwrap();
        let data = toml::from_str::<SavedRepositories>(&string).ok()?;
        Some(data)
    }

    fn save_config(&self) -> Result<(), io::Error> {
        super::save_config_internal(FILE_NAME, self)?;
        Ok(())
    }
}
