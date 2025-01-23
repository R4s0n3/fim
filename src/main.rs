use clap::{Arg, Command};
use image::{ImageFormat};
use std::fs::File;
use std::path::Path;

fn main() {
    let matches = Command::new("fim")
        .version("1.0")
        .about("Converts images to .ico format")
        .arg(
            Arg::new("input")
                .help("Input image file (.png, .jpg)")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("size")
                .help("Icon size")
                .short('s')
                .long("size")
                .value_parser(["sm", "md", "lg"])
                .default_value("md"),
        )
        .get_matches();

    let input = matches.get_one::<String>("input").unwrap();
    let size = matches.get_one::<String>("size").unwrap();

    let (width, height) = match size.as_str() {
        "sm" => (64, 64),
        "md" => (128, 128),
        "lg" => (256, 256),
        _ => (256, 256),
    };

    let input_path = Path::new(input);
    let output_path = input_path.with_extension("ico");

    let img = image::open(input_path).expect("Failed to open input image");

    let resized_img = img.resize_exact(width, height, image::imageops::Lanczos3);

    let mut output_file = File::create(&output_path).expect("Failed to create output file");

    resized_img.write_to(&mut output_file, ImageFormat::Ico).expect("Failed to write .ico file");
    
    println!(".ico file created successfully.");
}
