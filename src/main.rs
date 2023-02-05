use std::fs::File;
use std::io::prelude::*;

use clap::Parser;

mod ideas_args;

fn main() {
    let args = ideas_args::IdeaArgs::parse();

    let path = "$HOME/ideas.txt";
    let contents = std::fs::read_to_string(path).expect("failed to read file");

    match args.entity_type {
        ideas_args::EntityType::List => {
            let mut line_num = 0;
            for line in contents.lines() {
                line_num += 1;
                println!("[{}] {}", line_num, line);
            }
        }
        ideas_args::EntityType::Add(add_command) => {
            let mut output = File::create(path).expect("could not create file");
            if contents.is_empty() {
                write!(output, "{}", add_command.idea).expect("failed to write to file");
            } else {
                write!(output, "{}\n{}", contents, add_command.idea)
                    .expect("failed to write to file");
            }
        }
        ideas_args::EntityType::Clear => {
            File::create(path).expect("could not create file");
        }
    }
}
