use clap::Parser;
use crate::commands::{Cli, Commands};
use crate::doc::DocImpl;
use crate::fontmin::FontMinImpl;
use crate::hash::HashImpl;
use crate::serve::ServeImpl;

mod hash;
mod commands;
mod doc;
mod serve;
mod fontmin;
mod utils;

fn main() {
    let cmd = Cli::parse().command;

    match cmd {
        Commands::Doc { browser } => DocImpl::handle(browser),
        Commands::Hash { file, algorithm } => HashImpl::handle(file, algorithm),
        Commands::Serve { root, entry, port, mode } => {
            ServeImpl::handle(root, entry, port, mode);
        }
        // Commands::FontMin { input, output, chars } => FontMinImpl::handle(input, output, chars),
        _ => println!("Not yet implemented!")
    }
}
