use clap::Parser;
use crate::commands::{Cli, Commands};
use crate::doctor::DoctorImpl;
use crate::hash::HashImpl;
use crate::serve::ServeImpl;

mod hash;
mod commands;
mod doctor;
mod serve;

fn main() {
    let cmd = Cli::parse().command;

    match cmd {
        Commands::Doctor { verbose } => DoctorImpl::handle(verbose),
        Commands::Hash { file, algorithm } => HashImpl::handle(file, algorithm),
        Commands::Serve { root, entry, port, mode } => {
            ServeImpl::handle(root, entry, port, mode);
        }
        // _ => println!("Not yet implemented!")
    }
}
