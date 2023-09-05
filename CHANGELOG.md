### v0.1.4 - 2023.09.06

- feat: add `Commands::Pack` -- Pack any number of files or directories (possibly both) into an archive or compressed
  package of the specified format. (Supported values are: '*.7z', '*.tar', '*.tgz', '*.tar.gz', '*.zip',
  case-insensitive)
- feat + refactor: rename `Commands::Extract` to `Commands::Unpack` -- Unpack the specified archive or compressed
  package into the specified directory. (Supported values are: '*.7z', '*.tar', '*.tgz'/'*.tar.gz', '*.zip',
  case-insensitive)

### v0.1.3 - 2023.07.22

- feat: add `Commands::Extract` -- Extract compressed or archived files
- feat: add `Commands::Image` -- Convert the specified image to the specified format and/or size
- docs: update README.md & CHANGELOG.md
- docs: sub-docs for each command

### v0.1.2 - 2023.07.17

- refactor: remove `Commands::Doctor`, add `Commands::Doc`
- refactor: refactor `Commands::Hash` to support cross-platform usage

### v0.1.1 - 2023.06.15

- fix: remove strict restrictions on ports

### v0.1.0 - 2023.06.15

- chore: first release