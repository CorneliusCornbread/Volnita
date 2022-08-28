pub mod command;
pub mod repo;
pub mod volnita;
pub mod commit_table;
pub mod traits;
pub mod views;

use std::{error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    volnita::start()
}
