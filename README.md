# rstool - a cli tools written in rust

[![GitHub release (latest by date)](https://img.shields.io/github/v/release/lopo12123/rstool)](https://github.com/lopo12123/rstool/releases/tag/v0.1.0)
![OS](https://img.shields.io/badge/os-windows-orange)
![License](https://img.shields.io/github/license/lopo12123/rstool)

### Requirements

The following are the prerequisites for using `rstool`

- **OS**: `windows` only.
- **Environment**: `cmd.exe` must exist in the system environment variables and be accessible. (No additional settings
  are necessary as this is the default)

### Installation

Get [`rstool.exe`](https://github.com/lopo12123/rstool/releases/tag/v0.1.0) from the release page.  
You can also clone this project directly if you want to build or modify it yourself.

I recommend that you add the path of `rstool.exe` to the environment variable after downloading.
so that it can be used directly in any directory.

### Usage

```
Usage: rstool <COMMAND>

Commands:
  doctor  Check the version, availability, and other information of this tool
  hash    Get the specified hash value of the target file
  serve   Start a static resource server in the specified directory
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### CHANGELOG

see at [CHANGELOG.md](./CHANGELOG.md)

---

Last update time: 2023-06-15