# **FIM**

`fim` is a simple CLI tool to convert images to `.ico` format, built with Rust. 

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-blue.svg)
![Version](https://img.shields.io/badge/Version-1.0.0-brightgreen.svg)

## **Installation**

### **Download the Binary**

1. Go to the [Releases](https://github.com/r4s0n3/fim/releases) page.
2. Download the binary for your platform (e.g., `fim` for macOS).
3. Move the binary to a directory in your PATH. For example:
   
   ```sh
   mv fim /usr/local/bin
```

## **Usage**

To use the Favicon Maker, ensure that your binary is installed correctly and located in a directory included in your PATH. Then, in your terminal, you can simply type:

```sh
   fim 'path/to/your/image_file.png'
```

You can also specify the size of the icon using the `--size` option. The available sizes are:
* `sm` (64x64)
* `md` (128x128)
* `lg` (256x256, default)
If you do not specify a size, the default will be 256px.

## **Features**

- Convert `.png` or `.jpg` images to `.ico` format.
- Supports multiple icon sizes: `sm` (64x64), `md` (128x128), and `lg` (256x256).