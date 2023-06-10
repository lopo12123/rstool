use clap::Parser;
use crate::commands::{Cli, Commands};
use crate::hash::HashImpl;

mod hash;
mod commands;
mod doctor;

fn main() {
    let cmd = Cli::parse().command;

    match cmd {
        Commands::Doctor {} => {}
        Commands::Hash { file, algorithm } => HashImpl::handle(file, algorithm),
        // _ => println!("Not yet implemented!")
    }
}
