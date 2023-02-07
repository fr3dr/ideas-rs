use clap::Parser;

use ideas::ideas_args;

fn main() {
    let args = ideas_args::IdeaArgs::parse();

    if let Err(e) = ideas::run(args) {
        println!("Application error {e}");
    }
}
