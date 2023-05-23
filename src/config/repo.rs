use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Repository {
    path: String,
}

#[derive(Serialize, Deserialize)]
pub struct SavedRepositories {
    repositories: Vec<Repository>,
}
