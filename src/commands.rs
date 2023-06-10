use clap::{Parser, Subcommand};

/// Cli instance
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// All supported command
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Say hello to you
    #[command(about = "Say hello to you")]
    Greet {
        /// Your name
        #[arg(long, help = "Your name")]
        name: Option<String>
    },
    /// Say hello to you
    #[command(about = "Check the version, availability, and other information of this tool")]
    Doctor {},
    /// Get the specified hash value of the target file
    #[command(about = "Get the specified hash value of the target file")]
    Hash {
        /// Path to the target file
        #[arg(help = "Path to the target file")]
        file: String,
        /// The hash algorithm used, supports 'md5', 'sha1', 'sha256', and 'sha512' (case insensitive)
        #[arg(help = "The hash algorithm used, supports 'md5', 'sha1', 'sha256', and 'sha512' (case insensitive)")]
        algorithm: String,
    },
}