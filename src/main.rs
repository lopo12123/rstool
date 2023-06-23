use clap::Parser;
use crate::commands::{Cli, Commands};
use crate::doctor::DoctorImpl;
use crate::fontmin::FontMinImpl;
use crate::hash::HashImpl;
use crate::serve::ServeImpl;

mod hash;
mod commands;
mod doctor;
mod serve;
mod fontmin;
mod utils;

fn main() {
    let cmd = Cli::parse().command;

    match cmd {
        Commands::Doctor { verbose } => DoctorImpl::handle(verbose),
        Commands::Hash { file, algorithm } => HashImpl::handle(file, algorithm),
        Commands::Serve { root, entry, port, mode } => {
            ServeImpl::handle(root, entry, port, mode);
        }
        Commands::FontMin { input, output, chars } => FontMinImpl::handle(input, output, chars),
        // _ => println!("Not yet implemented!")
    }
}
