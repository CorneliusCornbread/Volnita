pub mod command;
pub mod commit_table;
pub mod config;
pub mod input_mode;
pub mod traits;
pub mod view_components;
pub mod views;
pub mod volnita;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    volnita::start()
}
