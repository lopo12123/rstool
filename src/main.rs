use clap::Parser;
use crate::commands::{Cli, Commands};
use crate::doc::DocImpl;
use crate::extract::ExtractImpl;
use crate::hash::HashImpl;
use crate::image::ImageImpl;
use crate::serve::ServeImpl;

mod commands;
mod doc;
mod extract;
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
        Commands::Extract { source, target, format } => {
            ExtractImpl::handle(source, target, format);
        }
        Commands::Hash { source, filemode, algorithm } => {
            HashImpl::handle(source, filemode, algorithm);
        }
        Commands::Image { source, format, size } => {
            ImageImpl::handle(source, format, size);
        }
        Commands::Serve { root, entry, port, mode } => {
            ServeImpl::handle(root, entry, port, mode);
        }
        // Commands::FontMin { input, output, chars } => FontMinImpl::handle(input, output, chars),
        // _ => println!("Not yet implemented!")
    }
}
