pub mod repo;

use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

use serde::Serialize;

pub trait Config: Sized + Default {
    fn save_config(&self) -> Result<(), io::Error>;

    fn load_config() -> Option<Self>;

    fn load_or_create_config() -> Self {
        let data = Self::load_config();

        if let Some(repo) = data {
            repo
        } else {
            Self::default()
        }
    }
}

fn get_config_path() -> Option<PathBuf> {
    let mut dir = dirs::config_local_dir()?;
    dir.push("volnita");

    Some(dir)
}

fn save_config_internal(preference_file: &str, value: &impl Serialize) -> Result<(), io::Error> {
    let mut dir = {
        match get_config_path() {
            Some(v) => Ok(v),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Home directory path has not been specified in environment configuration.",
            )),
        }
    }?;
    let mut dir_build = fs::DirBuilder::new();
    dir_build.recursive(true).create(&dir)?;

    dir.push(format!("{preference_file}.toml"));

    let data =
        toml::to_string_pretty(value).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let mut file = File::create(dir)?;
    file.write_all(data.as_bytes())?;

    Ok(())
}
