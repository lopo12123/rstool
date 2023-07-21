# Commands::extract

### Usage

```
$rstool extract --help

Extract compressed or archived files

Usage: rstool.exe extract [OPTIONS] --source <SOURCE>

Options:
  -s, --source <SOURCE>  The path where the compressed file or archive is located (points to a file)
  -t, --target <TARGET>  The path to extract the compressed file or archive (points to a folder, if the folder does not exist, it will be created automatically) [default: .]
  -f, --format <FORMAT>  The format of the compressed or archived file, if omitted it will be automatically inferred from the file suffix. (Supported values are: 'zip', 'rar', '7z', 'tar', 'tgz'/'tar.gz', case insensitive)
  -h, --help             Print help
```

### Support Matrix (tested on windows)

| format             | support | note |
|--------------------|---------|------|
| `.7z`              | ✅       |      |
| `.gz`              | ✅       |      |
| `.rar`             | ⏳       | (1)  |
| `.tar`             | ✅       |      |
| `.tgz` / `.tar.gz` | ✅       | (2)  |
| `.zip`             | ✅       | (3)  |

1. There is no rar package for me to test, so I can't get it done
2. `*.tar.gz` needs to manually specify `-f tar.gz` to decompress correctly, otherwise it will be simply regarded as
   a `.gz` file and decompressed to get a `*.tar` file
3. After the file is decompressed, there may be garbled characters, and the encoding of the output file name dependson
   the terminal where the command is run

---

Last modified on **2023-07-22**