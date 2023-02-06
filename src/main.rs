use std::env::set_current_dir;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use clap::Parser;
use dirs::home_dir;

mod ideas_args;

fn main() {
    let args = ideas_args::IdeaArgs::parse();

    set_current_dir(home_dir().unwrap()).unwrap();

    let file_name = "ideas";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)
        .expect("failed to init file");

    let contents = std::fs::read_to_string(file_name).expect("failed to read file");

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
                write!(file, "{}", add_command.idea).expect("failed to write to file");
            } else {
                write!(file, "{}\n{}", contents, add_command.idea)
                    .expect("failed to write to file");
            }
        }
        ideas_args::EntityType::Clear => {
            File::create(file_name).expect("could not create file");
        }
    }
}
