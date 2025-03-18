use std::io::Write;
use std::fs::File;
use std::path::Path;
use image::{GenericImageView, ImageFormat, imageops::FilterType};
use clap::{Command, Arg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("fim")
        .version("1.1")
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
        .arg(
            Arg::new("preserve")
                .help("Preserve aspect ratio")
                .short('p')
                .long("preserve")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("filter")
                .help("Resize filter")
                .short('f')
                .long("filter")
                .value_parser(["nearest", "triangle", "catmullrom", "gaussian", "lanczos3"])
                .default_value("lanczos3"),
        )
        .get_matches();

    // Get command line arguments
    let input = matches.get_one::<String>("input").unwrap();
    let size = matches.get_one::<String>("size").unwrap();
    let preserve_aspect = matches.get_flag("preserve");
    
    // Map filter strings to actual FilterType
    let filter_str = matches.get_one::<String>("filter").unwrap();
    let filter = match filter_str.as_str() {
        "nearest" => FilterType::Nearest,
        "triangle" => FilterType::Triangle,
        "catmullrom" => FilterType::CatmullRom,
        "gaussian" => FilterType::Gaussian,
        "lanczos3" => FilterType::Lanczos3,
        _ => FilterType::Lanczos3,
    };
    
    // Size mapping
    const SIZE_MAP: &[(&str, u32)] = &[
        ("sm", 64),
        ("md", 128),
        ("lg", 256),
    ];
    
    let target_size = SIZE_MAP
        .iter()
        .find(|(s, _)| *s == size.as_str())
        .map(|(_, dimension)| *dimension)
        .unwrap_or(128); // Default to medium if not found
    
    let input_path = Path::new(input);
    let output_path = input_path.with_extension("ico");
    
    // Open image
    let img = image::open(input_path)?;
    let (orig_width, orig_height) = img.dimensions();
    
    // Resize logic
    let resized_img = if preserve_aspect {
        // Calculate dimensions while preserving aspect ratio
        let ratio = orig_width as f32 / orig_height as f32;
        let (new_width, new_height) = if ratio > 1.0 {
            // Width is larger, constrain width
            (target_size, (target_size as f32 / ratio) as u32)
        } else {
            // Height is larger or equal, constrain height
            ((target_size as f32 * ratio) as u32, target_size)
        };
        
        // Resize with calculated dimensions
        img.resize(new_width, new_height, filter)
    } else {
        // Force exact dimensions
        img.resize_exact(target_size, target_size, filter)
    };
    
    // Create transparent background for non-square images when preserving aspect ratio
    let final_img = if preserve_aspect {
        // Create a new transparent image of target size
        let mut canvas = image::DynamicImage::new_rgba8(target_size, target_size);
        
        // Calculate position to center the resized image
        let (resized_width, resized_height) = resized_img.dimensions();
        let x_offset = (target_size - resized_width) / 2;
        let y_offset = (target_size - resized_height) / 2;
        
        // Overlay the resized image on the transparent canvas
        image::imageops::overlay(&mut canvas, &resized_img, x_offset as i64, y_offset as i64);
        canvas
    } else {
        resized_img
    };
    
    // Write the image
    let output_file = File::create(&output_path)?;
    let mut buffered_writer = std::io::BufWriter::new(output_file);
    final_img.write_to(&mut buffered_writer, ImageFormat::Ico)?;
    buffered_writer.flush()?;
    
    println!("Created .ico file successfully: {}", output_path.display());
    println!("Original size: {}x{}, New size: {}x{}", 
        orig_width, orig_height, 
        final_img.width(), final_img.height());
    
    Ok(())
}