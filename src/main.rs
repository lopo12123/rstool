use clap::Parser;
use crate::commands::{Cli, Commands};
use crate::doctor::DoctorImpl;
use crate::hash::HashImpl;

mod hash;
mod commands;
mod doctor;

fn main() {
    let cmd = Cli::parse().command;

    match cmd {
        Commands::Doctor { verbose } => DoctorImpl::handle(verbose),
        Commands::Hash { file, algorithm } => HashImpl::handle(file, algorithm),
        // _ => println!("Not yet implemented!")
    }
}
