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
    /// Open the document in the browser (default or specified)
    #[command(about = "Open the document in the browser (default or specified)")]
    Doc {
        #[arg(short, long, help = "Target browser, supported values are: 'Firefox', 'IE' (or 'Internet Explorer', 'InternetExplorer'), 'Chrome', 'Opera', 'Safari', 'Default', case insensitive), the default is the system default browser.")]
        browser: Option<String>
    },
    /// Get the digest of the specified source
    #[command(about = "Get the digest of the specified source")]
    Hash {
        /// Path to the target file
        #[arg(help = "Source text or source file path (with 'filemode' true) to be evaluated ")]
        source: String,
        /// Whether to treat source as a file path rather than a raw string (default to 'false')
        #[arg(short, long, help = "Whether to treat source as a file path rather than a raw string (default to 'false')", default_value = "false")]
        filemode: bool,
        /// The hash algorithm used
        #[arg(short, long, help = "Supported algorithms (case insensitive):\n- md5 (MD5)\n- ripemd (Ripemd128; Ripemd160; Ripemd256; Ripemd320)\n- sha1 (SHA1)\n- sha2 (SHA224; SHA256; SHA384; SHA512; SHA512_224; SHA512_256)\n- sha3 (SHA3_224; SHA3_256; SHA3_384; SHA3_512)\n", default_value = "MD5")]
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
        /// Port (0 ~ 65535), default to '8000'
        #[arg(short, long, help = "Port (0 ~ 65535)\n", default_value = "8000")]
        port: u16,
        /// Server mode
        #[arg(short, long, help = format!("{MODE_DESC}\n{SINGLE_MODE}\n{MIXED_MODE}\n{DIRECT_MODE}\n"), default_value = "mixed")]
        mode: String,
    },
    // A tool that generates a character-specific subset for a font file and can also view the font file's metadata.
    // #[command(about = "A tool that generates a character-specific subset for a font file and can also view the font file's metadata.")]
    // FontMin {
    //     #[arg(short, long, help = "Font file input")]
    //     input: String,
    //     #[arg(short, long, help = "Font file output, default to <[input]_subset.[suffix]>")]
    //     output: Option<String>,
    //     #[arg(short, long, help = "The name of the text file containing the Unicode character set to extract.\nIf not specified, the command will only output the metadata of the font file without subsetting.")]
    //     chars: Option<String>,
    // },
}