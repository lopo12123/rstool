# Commands::serve

### Usage

```
$rstool.exe serve --help

Start a static resource server in the specified directory

Usage: rstool.exe serve [OPTIONS]

Options:
  -r, --root <ROOT>    Root directory
                        [default: .]
  -e, --entry <ENTRY>  Entry file
                        [default: index.html]
  -p, --port <PORT>    Port (0 ~ 65535)
                        [default: 8000]
  -m, --mode <MODE>    Service mode, which determines how the server handles responses to various requests
                       - single: All requests will get the entry file as a response.
                       - mixed: Requests with a suffix will be considered as required resources and try to load the target resource, and the rest of the requests will be directed to the entry file.
                       - direct: First try to find the resource in the corresponding path under the root directory, if the resource exists and is a file type, return the resource, otherwise return the entry file.
                        [default: mixed]
  -h, --help           Print help
```

---

Last modified on **2023-07-20**