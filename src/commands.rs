use clap::{Parser, Subcommand};

const MODE_DESC: &str = "Service mode, which determines how the server handles responses to various requests";
const SINGLE_MODE: &str = "- single: All requests will get the entry file as a response.";
const MIXED_MODE: &str = "- mixed: Requests with a suffix will be considered as required resources and try to load the target resource, and the rest of the requests will be directed to the entry file.";
const DIRECT_MODE: &str = "- direct: First try to find the resource in the corresponding path under the root directory, if the resource exists and is a file type, return the resource, otherwise return the entry file.";

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
    #[command(about = "Check the version, availability, and other information of this tool")]
    Doctor {
        #[arg(short, long, help = "Whether to output complete information")]
        verbose: bool
    },
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
    /// Start a static resource server in the specified directory
    #[command(about = "Start a static resource server in the specified directory")]
    Serve {
        /// Root directory, default to '.'
        #[arg(short, long, help = "Root directory\n", default_value = ".")]
        root: String,
        /// Entry file, default to 'index.html'
        #[arg(short, long, help = "Entry file\n", default_value = "index.html")]
        entry: String,
        /// Port (1024 ~ 65535), default to '8000'
        #[arg(short, long, help = "Port (1024 ~ 65535)\n", default_value = "8000")]
        port: u16,
        #[arg(short, long, help = format!("{MODE_DESC}\n{SINGLE_MODE}\n{MIXED_MODE}\n{DIRECT_MODE}\n"), default_value = "mixed")]
        mode: String,
    },
    /// A tool that generates a character-specific subset for a font file and can also view the font file's metadata.
    #[command(about = "A tool that generates a character-specific subset for a font file and can also view the font file's metadata.")]
    FontMin {
        #[arg(short, long, help = "Font file input")]
        input: String,
        #[arg(short, long, help = "Font file output, default to <[input]_subset.[suffix]>")]
        output: Option<String>,
        #[arg(short, long, help = "The name of the text file containing the Unicode character set to extract.\nIf not specified, the command will only output the metadata of the font file without subsetting.")]
        chars: Option<String>,
    },
}