use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

use serde::Serialize;

fn get_config_path() -> Option<PathBuf> {
    let mut dir = dirs::home_dir()?;
    dir.push(".confg");
    dir.push("volnita");

    Some(dir)
}

pub fn save_preference(key: &str, value: &impl Serialize) -> Result<(), io::Error> {
    let mut dir = get_config_path().ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "Home directory path has not been specified in environment configuration.",
    ))?;
    dir.push("preferences");

    let data =
        toml::to_string_pretty(value).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let mut file = File::create(dir)?;
    file.write_all(data.as_bytes())?;

    Ok(())
}
