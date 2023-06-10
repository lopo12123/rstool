use clap::Parser;
use crate::commands::{Cli, Commands};
use crate::hash::HashImpl;

mod hash;
mod commands;
mod doctor;

fn main() {
    let cmd = Cli::parse().command;

    match cmd {
        Commands::Greet { name } => {
            let person = match name {
                Some(v) => v,
                None => "stranger".to_string()
            };

            println!("[Commands::Greet] name: {}", &person);
            println!("Hello, {}!", person);
        }
        Commands::Hash { file, algorithm } => HashImpl::handle(file, algorithm),
        Commands::Doctor {} => {}
        // _ => println!("Not yet implemented!")
    }
}
