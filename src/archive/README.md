# Commands::unpack & Commands::pack

### Usage -- unpack

```
$ rstool.exe unpack --help      
      
Unpack the specified archive or compressed package into the specified directory. (Supported values are: '*.7z', '*.tar', '*.tgz'/'*.tar.gz', '*.zip', case insensitive)

Usage: rstool.exe unpack <SOURCE> [DESTINATION]

Arguments:
  <SOURCE>       The path where the compressed file or archive is located (points to a file). The tool will judge its format according to its suffix, please make sure the format and suffix match. (Supported values are: '*.7z', '*.tar', '*.tgz', '*.tar.gz', '*.zip', case insensitive)
  [DESTINATION]  The path to extract the compressed file or archive (points to a folder, if the folder does not exist, it will be created automatically, default to '.') [default: .]

Options:
  -h, --help  Print help
```

### Usage -- pack

```
$ rstool.exe pack --help

Pack any number of files or directories (possibly both) into an archive or compressed package of the specified format. (Supported values are: '*.7z', '*.tar', '*.tgz', '*.tar.gz', '*.zip', case insensitive)

Usage: rstool.exe pack <DESTINATION> [SOURCES]...

Arguments:
  <DESTINATION>  The path to the archive or compressed package to be created. The tool will judge its format according to its suffix, please make sure the format and suffix match. (Supported values are: '*.7z', '*.tar', '*.tgz', '*.tar.gz', '*.zip', case insensitive)
  [SOURCES]...   The path to the directory(s) or file(s) to be packaged

Options:
  -h, --help  Print help
```

### Support Matrix (tested on windows)

| format             | support | note |
|--------------------|---------|------|
| `.7z`              | ✅       |      |
| `.gz`              | ✅       |      |
| `.tar`             | ✅       |      |
| `.tgz` / `.tar.gz` | ✅       | (1)  |
| `.zip`             | ✅       | (2)  |

1. `*.tar.gz` is exactly the same as `*.tgz`
2. After the file is decompressed, there may be garbled characters. See entries D.1 and D.2 of the ZIP file format
   specification for reasons for garbled
   behavior. [The latest ZIP standard](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT)

---

Last modified on **2023-09-06**