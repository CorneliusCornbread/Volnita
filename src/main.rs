pub mod app_flags;
pub mod command;
pub mod config;
pub mod data_table;
pub mod git;
pub mod input_mode;
pub mod traits;
pub mod view_components;
pub mod views;
pub mod volnita;

use std::{error::Error, panic};

fn main() -> Result<(), Box<dyn Error>> {
    // All this error catching is done to make sure we can return the terminal to
    // it's normal state regardless of whether or not we exit nicely.
    panic::set_hook(Box::new(|_info| {
        let _ = volnita::reset_terminal();
        println!("{_info:?}")
    }));

    let _ = panic::catch_unwind(|| {
        let res = volnita::start();

        let _ = volnita::reset_terminal();

        if let Err(err) = res {
            println!("{err:?}")
        }
    });

    Ok(())
}
