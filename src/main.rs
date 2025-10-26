use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use image::{ImageFormat, GenericImageView};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "webp2jpg")]
#[command(about = "Automatically convert WebP images to JPG", long_about = None)]
struct Args {
    /// Directory to monitor for WebP files
    #[arg(short, long, env = "WEBP2JPG_WATCH_DIR", default_value = "/home/ovc/Bilder")]
    watch_dir: PathBuf,
    
    /// Enable cropping landscape images to square
    #[arg(short, long, env = "WEBP2JPG_CROP", default_value = "false")]
    crop: bool,
}

fn convert_webp_to_jpg(webp_path: &Path, crop: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Converting: {:?}", webp_path);
    
    // Load the WebP image
    let mut img = image::open(webp_path)?;
    
    if crop {
        // Get dimensions
        let (width, height) = img.dimensions();
        
        // If width is larger than height, crop to square
        if width > height {
            let crop_amount = (width - height) / 2;
            img = img.crop_imm(crop_amount, 0, height, height);
            println!("Cropped to square: {}x{}", height, height);
        }
    }
    
    // Create output path by replacing .webp extension with .jpg
    let mut jpg_path = PathBuf::from(webp_path);
    jpg_path.set_extension("jpg");
    
    // Save as JPG with quality 90
    img.save_with_format(&jpg_path, ImageFormat::Jpeg)?;
    
    println!("Converted to: {:?}", jpg_path);
    
    // Delete the source WebP file
    std::fs::remove_file(webp_path)?;
    println!("Deleted source file: {:?}", webp_path);
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    println!("WebP to JPG converter service starting...");
    println!("Monitoring directory: {}", args.watch_dir.display());
    println!("Cropping enabled: {}", args.crop);
    
    // Create a channel to receive file system events
    let (tx, rx) = channel();
    
    // Create a watcher object
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Config::default())?;
    
    // Start watching the directory
    watcher.watch(&args.watch_dir, RecursiveMode::NonRecursive)?;
    
    println!("Watching for WebP files...");
    
    // Process events
    for res in rx {
        match res {
            Ok(event) => {
                if let Err(e) = handle_event(event, args.crop) {
                    eprintln!("Error handling event: {}", e);
                }
            }
            Err(e) => eprintln!("Watch error: {}", e),
        }
    }
    
    Ok(())
}

fn handle_event(event: Event, crop: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Only process file creation and modification events
    match event.kind {
        EventKind::Create(_) | EventKind::Modify(_) => {
            for path in event.paths {
                // Check if it's a WebP file
                if let Some(extension) = path.extension() {
                    if extension.eq_ignore_ascii_case("webp") {
                        // Give a brief moment for the file to be fully written
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        
                        if let Err(e) = convert_webp_to_jpg(&path, crop) {
                            eprintln!("Failed to convert {:?}: {}", path, e);
                        }
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}