pub mod command;
pub mod repo;
pub mod volnita;

use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    volnita::start()
}
