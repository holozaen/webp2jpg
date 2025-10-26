# WebP to JPG Converter Service

A lightweight Rust service that automatically monitors a directory for WebP images and converts them to JPG format in real-time.

## Features

- 🔍 **Automatic monitoring** - Watches a specified directory for new WebP files
- 🔄 **Instant conversion** - Converts WebP to JPG immediately when files are detected
- ✂️ **Optional cropping** - Can automatically crop landscape images to square format
- 🗑️ **Auto-cleanup** - Deletes source WebP files after successful conversion
- ⚙️ **Configurable** - Flexible configuration via command-line arguments or environment variables
- 🚀 **Systemd integration** - Runs as a background service on Linux

## Requirements

- Rust 1.90.0 or later
- Linux with systemd (for service mode)

## Installation

1. **Clone or download the project**

2. **Build the release binary:**
   ```bash
   cargo build --release
   ```

3. **The binary will be located at:**
   ```
   target/release/webp2jpg
   ```

## Usage

### Command Line

Run directly with default settings:
```bash
./target/release/webp2jpg
```

Specify a custom directory:
```bash
./target/release/webp2jpg --watch-dir /path/to/your/folder
```

Enable cropping for landscape images:
```bash
./target/release/webp2jpg --watch-dir /path/to/your/folder --crop
```

View all options:
```bash
./target/release/webp2jpg --help
```

### As a Systemd Service

1. Edit the service file (webp2jpg.service) to configure your settings:
```ini
User=<user>
Environment="WEBP2JPG_WATCH_DIR=/home/<user>/Images"
Environment="WEBP2JPG_CROP=true"
```

2. Copy the service file to systemd:
```bash
Environment="WEBP2JPG_WATCH_DIR=/home/ovc/Bilder"
Environment="WEBP2JPG_CROP=true"
```

3. Reload systemd and enable the service:
```bash
   sudo systemctl daemon-reload
   sudo systemctl enable webp2jpg.service
```

4. Start the service:
```bash
   sudo systemctl start webp2jpg.service
```

### Configuration Options
| Option          | CLI Flag    | Environment Variable | Default          | Description                            |
|-----------------|-------------|----------------------|------------------|----------------------------------------|
| Watch Directory | --watch-dir | WEBP2JPG_WATCH_DIR   | /home/ovc/Bilder | Directory to monitor for WebP files    |
| Crop to Square  | --crop      | WEBP2JPG_CROP        | false            | Crop landscape images to square format |


### How It Works

1. The service monitors the specified directory for file system events
2. When a WebP file is created or modified, it triggers the conversion process
3. The image is loaded and optionally cropped (if enabled and width > height)
4. The image is saved as JPG with 90% quality
5. The original WebP file is automatically deleted


### Dependencies

- [notify](https://crates.io/crates/notify)
- [image](https://crates.io/crates/image)
- [clap](https://crates.io/crates/clap)

### License

This project is provided as-is for personal use.

### Troubleshooting

#### Service won't start:

* Check that the watch directory exists and is accessible
* Verify the binary path in the service file is correct
* Check logs: sudo journalctl -u webp2jpg.service -xe

#### Images not converting:

* Ensure the service is running: sudo systemctl status webp2jpg.service
* Check that files have .webp extension
* Verify file permissions in the watch directory
* Check logs for error messages

#### Permission denied errors:

* Ensure the user specified in the service file has read/write access to the watch directory
* Check that the binary has execute permissions: chmod +x target/release/webp2jpg

