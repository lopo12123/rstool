use std::env::current_dir;
use clap::Parser;
use crate::commands::{Cli, Commands};
use crate::doc::DocImpl;
use crate::archive::{PackImpl, UnpackImpl};
use crate::hash::HashImpl;
use crate::image::ImageImpl;
use crate::serve::ServeImpl;

mod archive;
mod commands;
mod doc;
mod hash;
mod serve;
mod fontmin;
mod image;

fn main() {
    let cmd = Cli::parse().command;

    match cmd {
        Commands::Doc => {
            DocImpl::handle();
        }
        Commands::Hash { source, filemode, algorithm } => {
            HashImpl::handle(source, filemode, algorithm);
        }
        Commands::Image { source, format, size } => {
            ImageImpl::handle(source, format, size);
        }
        Commands::Pack { destination, sources } => {
            PackImpl::handle(current_dir().unwrap(), destination, sources);
        }
        Commands::Serve { root, entry, port, mode } => {
            ServeImpl::handle(root, entry, port, mode);
        }
        Commands::Unpack { source, destination } => {
            UnpackImpl::handle(source, destination);
        }
        // Commands::FontMin { input, output, chars } => FontMinImpl::handle(input, output, chars),
        // _ => println!("Not yet implemented!")
    }
}
