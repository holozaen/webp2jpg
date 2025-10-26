use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use image::ImageFormat;

const WATCH_PATH: &str = "/home/ovc/Bilder";

fn convert_webp_to_jpg(webp_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Converting: {:?}", webp_path);

    // Load the WebP image
    let img = image::open(webp_path)?;

    // Create output path by replacing .webp extension with .jpg
    let mut jpg_path = PathBuf::from(webp_path);
    jpg_path.set_extension("jpg");

    // Save as JPG with quality 90
    img.save_with_format(&jpg_path, ImageFormat::Jpeg)?;

    println!("Converted to: {:?}", jpg_path);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("WebP to JPG converter service starting...");
    println!("Monitoring directory: {}", WATCH_PATH);

    // Create a channel to receive file system events
    let (tx, rx) = channel();

    // Create a watcher object
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Config::default())?;

    // Start watching the directory
    watcher.watch(Path::new(WATCH_PATH), RecursiveMode::NonRecursive)?;

    println!("Watching for WebP files...");

    // Process events
    for res in rx {
        match res {
            Ok(event) => {
                if let Err(e) = handle_event(event) {
                    eprintln!("Error handling event: {}", e);
                }
            }
            Err(e) => eprintln!("Watch error: {}", e),
        }
    }

    Ok(())
}

fn handle_event(event: Event) -> Result<(), Box<dyn std::error::Error>> {
    // Only process file creation and modification events
    match event.kind {
        EventKind::Create(_) | EventKind::Modify(_) => {
            for path in event.paths {
                // Check if it's a WebP file
                if let Some(extension) = path.extension() {
                    if extension.eq_ignore_ascii_case("webp") {
                        // Give a brief moment for the file to be fully written
                        std::thread::sleep(std::time::Duration::from_millis(100));

                        if let Err(e) = convert_webp_to_jpg(&path) {
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