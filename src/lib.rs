use std::env::set_current_dir;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use dirs::home_dir;

pub mod ideas_args;

pub fn run(args: ideas_args::IdeaArgs) -> Result<(), Box<dyn Error>> {
    set_current_dir(home_dir().unwrap())?;

    let file_name = "ideas";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)?;

    let contents = std::fs::read_to_string(file_name)?;

    match args.entity_type {
        ideas_args::EntityType::List => {
            let mut line_num = 0;
            for line in contents.lines() {
                line_num += 1;
                println!("[{}] {}", line_num, line);
            }
        }
        ideas_args::EntityType::Add(add_command) => {
            if contents.is_empty() {
                write!(file, "{}", add_command.idea)?;
            } else {
                write!(file, "{}\n{}", contents, add_command.idea)?;
            }
        }
        ideas_args::EntityType::Clear => {
            File::create(file_name)?;
        }
    }

    Ok(())
}
