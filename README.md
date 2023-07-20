# rstool - a cli tools written in rust

[![GitHub release (latest by date)](https://img.shields.io/github/v/release/lopo12123/rstool)](https://github.com/lopo12123/rstool/releases/latest)
![OS](https://img.shields.io/badge/os-windows-orange)
![License](https://img.shields.io/github/license/lopo12123/rstool)

A collection of simple and commonly used tools, available on windows, will soon support wasm (partially)

### Installation

Get [`rstool.exe`](https://github.com/lopo12123/rstool/releases/latest) from the release page.  
You can also clone this project directly if you want to build or modify it yourself.

I recommend that you add the path of `rstool.exe` to the environment variable after downloading.
so that it can be used directly in any directory.

### Usage

```
Usage: rstool.exe <COMMAND>                                       
                                                                  
Commands:                                                         
  doc    Open the document in the browser (default or specified)  
  hash   Get the digest of the specified source                   
  serve  Start a static resource server in the specified directory
  help   Print this message or the help of the given subcommand(s)
                                                                  
Options:                                                          
  -h, --help     Print help                                       
  -V, --version  Print version  
```

### Support Matrix

| command   | cli | wasm (node) | wasm (web) | details                                      |
|-----------|-----|-------------|------------|----------------------------------------------|
| `Doc`     | ✅   | ⏳           | ⏳          |                                              |
| `Extract` | ✅   | ⏳           | ⏳          | [Commands::extract](./src/extract/README.md) |
| `Hash`    | ✅   | ⏳           | ⏳          |                                              |
| `Serve`   | ✅   | ⏳           | ⏳          ||
| ...       | ... | ...         | ...        ||

- ✅ - **supported** (done)
- ➖ - **partially supported**
- ❌ - **not supported**
- ⏳ - **in progress**

### CHANGELOG

see at [CHANGELOG.md](./CHANGELOG.md)

---

Last modified on **2023-07-20**