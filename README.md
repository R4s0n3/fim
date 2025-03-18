# FIM - Fast Icon Maker

A cross-platform command-line utility to convert images to Windows/macOS/Linux `.ico` format with flexible sizing options.

## Version 1.1.0

FIM (Fast Icon Maker) allows you to quickly convert your PNG, JPG, or other image formats to ICO files with customizable size options and aspect ratio handling.

## Features

- Convert various image formats to `.ico`
- Multiple predefined size options (small, medium, large)
- Preserve aspect ratio with transparent padding
- Customizable resampling filters for optimal quality
- Works on Windows, macOS, and Linux platforms

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/fim.git
cd fim

# Build with cargo
cargo build --release

# Optional: Move binary to your path
# On macOS/Linux:
cp target/release/fim /usr/local/bin/
# On Windows:
# copy target\release\fim.exe to a directory in your PATH
```

## Usage

Basic usage:

```bash
fim input.png
```

This will convert `input.png` to `input.ico` using default settings (medium size 128x128).

### Command Line Arguments

```
USAGE:
    fim [OPTIONS] <input>

ARGS:
    <input>    Input image file (.png, .jpg)

OPTIONS:
    -s, --size <SIZE>      Icon size [possible values: sm, md, lg] [default: md]
    -p, --preserve         Preserve aspect ratio
    -f, --filter <FILTER>  Resize filter [possible values: nearest, triangle, catmullrom, gaussian, lanczos3] [default: lanczos3]
    -h, --help             Print help information
    -V, --version          Print version information
```

### Size Options

- `sm`: Small (64x64 pixels)
- `md`: Medium (128x128 pixels)
- `lg`: Large (256x256 pixels)

### Filter Options

- `nearest`: Fastest, lowest quality
- `triangle`: Fast with decent quality
- `catmullrom`: Good quality with some sharpening
- `gaussian`: Smooth results, may blur details
- `lanczos3`: Highest quality (default)

## Examples

Convert to large icon:
```bash
fim image.png --size lg
```

Preserve aspect ratio:
```bash
fim image.png --size lg --preserve
```

Use different filter:
```bash
fim image.png --filter gaussian
```

## What's New in Version 1.1

- Added aspect ratio preservation with `-p/--preserve` flag
- Added customizable resize filters with `-f/--filter` option
- Improved image quality with transparent background for non-square images
- Added detailed output information

## Platform Support

This tool works on:
- Windows
- macOS
- Linux

ICO files are primarily used for Windows applications, but this tool allows you to create them on any platform.

## License

[MIT License](LICENSE)

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## TODO

 - transform .svg to .ico ?