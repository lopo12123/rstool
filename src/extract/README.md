# Commands::unpack & Commands::pack

### Commands::unpack Usage

```
$rstool unpack --help

Unpack the specified archive or compressed package into the specified directory

Usage: rstool.exe unpack <SOURCE> [DESTINATION]

Arguments:
  <SOURCE>       The path where the compressed file or archive is located (points to a file)
  [DESTINATION]  The path to extract the compressed file or archive (points to a folder, if the folder does not exist, it will be created automatically, default to '.') [default: .]

Options:
  -h, --help  Print help
```

### Commands::pack Usage

```
// TODO
```

### Support Matrix (tested on windows)

| format             | support | note |
|--------------------|---------|------|
| `.7z`              | ✅       |      |
| `.gz`              | ✅       |      |
| `.tar`             | ✅       |      |
| `.tgz` / `.tar.gz` | ✅       | (1)  |
| `.zip`             | ✅       | (2)  |

1. `*.tar.gz` needs to manually specify `-f tar.gz` to decompress correctly, otherwise it will be simply regarded as
   a `.gz` file and decompressed to get a `*.tar` file
2. After the file is decompressed, there may be garbled characters, and the encoding of the output file name dependson
   the terminal where the command is run

---

Last modified on **2023-08-09**