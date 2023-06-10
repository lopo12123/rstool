use clap::Parser;
use crate::commands::{Cli, Commands};
use crate::hash::HashImpl;

mod hash;
mod parser;
mod commands;

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
        Commands::Hash { file, algorithm } => {
            println!("[Commands::Hash] file: {}, algorithm: {}", file, algorithm);

            match HashImpl::hash(&file, &algorithm) {
                Ok(res) => println!("Ok: {}", res),
                Err(err) => println!("Error: {}", err),
            };
        }
        // _ => println!("Not yet implemented!")
    }
}
