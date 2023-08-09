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
    /// Open the document in the default browser
    #[command(about = "Open the document in the default browser")]
    Doc,
    #[command(about = "Extract compressed or archived files")]
    Extract {
        #[arg(short, long, help = "The path where the compressed file or archive is located (points to a file)")]
        source: String,
        #[arg(short, long, help = "The path to extract the compressed file or archive (points to a folder, if the folder does not exist, it will be created automatically)", default_value = ".")]
        target: String,
        #[arg(short, long, help = "The format of the compressed or archived file, if omitted it will be automatically inferred from the file suffix. (Supported values are: 'zip', 'rar', '7z', 'tar', 'tgz'/'tar.gz', case insensitive)")]
        format: Option<String>,
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
        #[arg(short, long, help = "Supported algorithms (case insensitive):\n- md5 (MD5)\n- ripemd (Ripemd128, Ripemd160, Ripemd256, Ripemd320)\n- sha1 (SHA1)\n- sha2 (SHA224, SHA256, SHA384, SHA512, SHA512_224, SHA512_256)\n- sha3 (SHA3_224, SHA3_256, SHA3_384, SHA3_512)\n", default_value = "MD5")]
        algorithm: String,
    },
    /// Convert the specified image to the specified format and/or size (simple show the metadata of the image if both format and size are omitted)
    #[command(about = "Convert the specified image to the specified format and/or size  (simple show the metadata of the image if both format and size are omitted)")]
    Image {
        /// Path to the source image
        #[arg(help = "Path to the source image")]
        source: String,
        /// Target image format. If omitted, the format of the source image will be used. (Supported values are: 'bmp', 'gif', 'ico', 'jpg'('jpeg'), 'png', 'tiff', case insensitive)
        #[arg(short, long, help = "Target image format (Supported values are: 'bmp', 'gif', 'ico', 'jpg'('jpeg'), 'png', 'tiff', case insensitive)")]
        format: Option<String>,
        /// Target image size. This should be in the format of '(width)x(height)'. If only one of the width and height is specified, the other will be scaled proportionally. If both are omitted, the original size will be used. (e.g. '100x200' or 'x200' or '100x'.)
        #[arg(short, long, help = "Target image size. This should be in the format of '(width)x(height)'. If only one of the width and height is specified, the other will be scaled proportionally. If both are omitted, the original size will be used. (e.g. '100x200' or 'x200' or '100x'.)")]
        size: Option<String>,
    },
    /// Package the specified directory or file into an archive or compressed package of the specified format
    // Pack {
    //     #[arg(multiple, help = "The path to the directory or files to be packaged")]
    //     source: Vec<String>,
    //     target: Option<String>,
    // },
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
    /// Unpack the specified archive or compressed package into the specified directory
    #[command(about = "Unpack the specified archive or compressed package into the specified directory")]
    Unpack {
        /// The path where the compressed file or archive is located (points to a file)
        #[arg(help = "The path where the compressed file or archive is located (points to a file)")]
        source: String,
        /// The path to extract the compressed file or archive (points to a folder, if the folder does not exist, it will be created automatically, default to '.')
        #[arg(help = "The path to extract the compressed file or archive (points to a folder, if the folder does not exist, it will be created automatically, default to '.')", default_value = ".")]
        destination: String,
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