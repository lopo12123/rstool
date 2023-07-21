# Commands::image

### Usage

```
$rstool.exe image --help

Convert the specified image to the specified format and/or size  (simple show the metadata of the image if both format and size are omitted)

Usage: rstool.exe image [OPTIONS] <SOURCE>

Arguments:
  <SOURCE>  Path to the source image

Options:
  -f, --format <FORMAT>  Target image format (Supported values are: 'bmp', 'gif', 'ico', 'jpg'('jpeg'), 'png', 'tiff', case insensitive)
  -s, --size <SIZE>      Target image size. This should be in the format of '(width)x(height)'. If only one of the width and height is specified, the other will be scaled proportionally. If both are omitted, the original s
ize will be used. (e.g. '100x200' or 'x200' or '100x'.)
  -h, --help             Print help
```

### Support Matrix (tested on windows)

- meta data

| format        | support | note |
|---------------|---------|------|
| `.bmp`        | ✅       |      |
| `.gif`        | ✅       |      |
| `.ico` (1)    | ➖       | (2)  |
| `.jpg`/`jpeg` | ✅       | (3)  |
| `.png`        | ✅       |      |
| `.tiff`       | ✅       |      |

- convert (include `resize` and `format`)

| source\target | `.bmp` | `.gif` | `.ico` | `.jpg`/`.jpeg` | `.png` | `.tiff` |
|---------------|--------|--------|--------|----------------|--------|---------|
| `.bmp`        | ✅      | ✅      | ✅      | ✅              | ✅      | ✅       |
| `.gif`        | ✅      | ✅      | ✅      | ✅              | ✅      | ✅       |
| `.ico` (1)    | ➖      | ➖      | ➖      | ➖              | ➖      | ➖       |
| `.jpg`/`jpeg` | ✅      | ✅      | ✅      | ✅              | ✅      | ✅       |
| `.png`        | ✅      | ✅      | ✅      | ✅              | ✅      | ✅       |
| `.tiff`       | ✅      | ✅      | ✅      | ✅              | ✅      | ✅       |

1. Processing of `ico` files with 24-bit color will fail, but 32-bit color can be fully supported
2. In the definition document of the ico format, it is clearly pointed out that the width and height limit of the
   embedded image is one byte, that is, the maximum is 256 pixels. Click this link for a detailed description on
   Wikipedia: [ICO (file format)](https://en.wikipedia.org/wiki/ICO_(file_format))
3. Converting from an image with an alpha channel to `jpeg` will lose information, and previously transparent parts will
   become completely black. This is because `jpeg` does not support alpha channels. For details, please refer to
   the [JPEG](https://en.wikipedia.org/wiki/JPEG)
