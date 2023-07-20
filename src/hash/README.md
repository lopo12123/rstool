# Commands::hash

### Usage

```
$rstool.exe hash --help

Get the digest of the specified source

Usage: rstool.exe hash [OPTIONS] <SOURCE>

Arguments:
  <SOURCE>  Source text or source file path (with 'filemode' true) to be evaluated

Options:
  -f, --filemode               Whether to treat source as a file path rather than a raw string (default to 'false')
  -a, --algorithm <ALGORITHM>  Supported algorithms (case insensitive):
                               - md5 (MD5)
                               - ripemd (Ripemd128; Ripemd160; Ripemd256; Ripemd320)
                               - sha1 (SHA1)
                               - sha2 (SHA224; SHA256; SHA384; SHA512; SHA512_224; SHA512_256)
                               - sha3 (SHA3_224; SHA3_256; SHA3_384; SHA3_512)
                                [default: MD5]
  -h, --help                   Print help
```

### Support Matrix (tested on windows)

| algorithm | support |
|-----------|---------|
| `md5`     | ✅       |
| `ripemd`  | ✅       |
| `sha1`    | ✅       |
| `sha2`    | ✅       |
| `sha3`    | ✅       |

---

Last modified on **2023-07-20**