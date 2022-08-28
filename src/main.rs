pub mod command;
pub mod repo;
pub mod volnita;
pub mod commit_table;

use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    volnita::start()
}
