# rstool - a cli tools written in rust

[![GitHub release (latest by date)](https://img.shields.io/github/v/release/lopo12123/rstool)](https://github.com/lopo12123/rstool/releases/latest)
![cli](https://img.shields.io/badge/cli-supported-green)
![node-addon](https://img.shields.io/badge/node--addon-in--progress-yellow)
![cli](https://img.shields.io/badge/wasm-in--progress-yellow)
![License](https://img.shields.io/github/license/lopo12123/rstool)

A collection of simple and commonly used tools, available
as [cli](https://en.wikipedia.org/wiki/Command-line_interface), will soon
support [node-addon](https://nodejs.org/api/addons.html) and [wasm](https://webassembly.org/).

### Installation

Get [`rstool.exe`](https://github.com/lopo12123/rstool/releases/latest) from the release page.  
You can also clone this project directly if you want to build or modify it yourself.

### Usage

```
$rstool.exe --help

a cli tools written in rust

Usage: rstool.exe <COMMAND>

Commands:
  doc     Open the document in the default browser
  hash    Get the digest of the specified source
  image   Convert the specified image to the specified format and/or size  (simple show the metadata of the image if both format and size are omitted)
  pack    Pack any number of files or directories (possibly both) into an archive or compressed package of the specified format. (Supported values are: '*.7z', '*.tar', '*.tgz', '*.tar.gz', '*.zip', case insensitive)
  serve   Start a static resource server in the specified directory
  unpack  Unpack the specified archive or compressed package into the specified directory. (Supported values are: '*.7z', '*.tar', '*.tgz'/'*.tar.gz', '*.zip', case insensitive)
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Support Matrix

| command           | cli | node-addon | wasm | details                                                    |
|-------------------|-----|------------|------|------------------------------------------------------------|
| `Doc`             | ✅   | ⏳          | ⏳    | [Commands::doc](./src/doc/README.md)                       |
| `Hash`            | ✅   | ⏳          | ⏳    | [Commands::hash](./src/hash/README.md)                     |
| `Image`           | ✅   | ⏳          | ⏳    | [Commands::image](./src/image/README.md)                   |
| `Pack` / `Unpack` | ✅   | ⏳          | ⏳    | [Commands::pack / Commands::unpack](src/archive/README.md) |
| `Serve`           | ✅   | ⏳          | ⏳    | [Commands::serve](./src/serve/README.md)                   |
| ...               | ... | ...        | ...  |                                                            |

- ✅ - **supported**
- ➖ - **partially supported**
- ❌ - **not supported**
- ⏳ - **in progress**

### CHANGELOG

see at [CHANGELOG.md](./CHANGELOG.md)

---

Last modified on **2023-09-06**