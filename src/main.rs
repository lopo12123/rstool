use clap::Parser;
use crate::commands::{Cli, Commands};
use crate::doc::DocImpl;
use crate::extract::ExtractImpl;
use crate::hash::HashImpl;
use crate::serve::ServeImpl;

mod commands;
mod doc;
mod extract;
mod hash;
mod serve;
mod fontmin;

fn main() {
    let cmd = Cli::parse().command;

    match cmd {
        Commands::Doc { browser } => {
            DocImpl::handle(browser);
        }
        Commands::Hash { source, filemode, algorithm } => {
            HashImpl::handle(source, filemode, algorithm);
        }
        Commands::Serve { root, entry, port, mode } => {
            ServeImpl::handle(root, entry, port, mode);
        }
        Commands::Extract { source, target, format } => {
            ExtractImpl::handle(source, target, format);
        }
        // Commands::FontMin { input, output, chars } => FontMinImpl::handle(input, output, chars),
        // _ => println!("Not yet implemented!")
    }
}
