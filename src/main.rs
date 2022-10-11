pub mod command;
pub mod commit_table;
pub mod input_mode;
pub mod repo;
pub mod traits;
pub mod views;
pub mod volnita;
pub mod view_components;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    volnita::start()
}
