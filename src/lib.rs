#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://docs.rs/-/rustdoc.static/favicon-32x32-422f7d1d52889060.png",
    issue_tracker_base_url = "https://github.com/nandolawson/keyforge95/issues",
    html_no_source
)]
#![doc = include_str!("../README.md")] // Adding the README to the documentation

use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};

/// The type of ROM that is being trimmed.
#[derive(Clone, Copy, Debug)]
pub enum Type {
    GBA,
    NDS,
}

/// Checks if the file exists and is at least 200 bytes long.
/// If both conditions are met, the function returns `true`.
///
/// # Arguments
/// - `path`: Path to a ROM file
#[must_use]
pub fn check_file(path: &str) -> bool {
    // Open the file and check if it is at least 1024 bytes long
    match File::open(path).and_then(|file| file.metadata()) {
        Ok(metadata) => metadata.len() >= 1024,
        Err(_) => false,
    }
}

/// Detects the type of ROM based on the SHA256 hash of Nintendo logo within the ROM header.
/// A Game Boy Advance ROM will contain the Nintendo Logo at 0x004.
/// A DS ROM will contain the Nintendo Logo at 0x0C0.
///
/// # Arguments
/// - `file`: A reference to an open file
///
/// # Returns
/// - `Ok`: Returns the type of the ROM as a static string (`"GBA"` or `"NDS"`).
/// - `Err`: Returns an `io::Error` if the ROM is not supported.
///
/// # Errors
/// This function will return an error if the file isn't supported by this software.
pub fn detect_rom(file: &mut File) -> Result<String, io::Error> {
    // Hash of the Nintendo logo
    // The hash is the same for both ROM types
    let nintendo_logo_hash = "08a0153cfd6b0ea54b938f7d209933fa849da0d56f5a34c481060c9ff2fad818";

    // Check if the GBA Nintendo logo is present
    if hash(file, 0x004) == nintendo_logo_hash {
        Ok("GBA".to_string())
    // Check if the a DS Nintendo logo is present
    } else if hash(file, 0x0C0) == nintendo_logo_hash {
        Ok("NDS".to_string())
    // Return an error if the file is not supported
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "File not supported",
        ))
    }
}

/// Hashes a portion of the ROM file.
/// The hash is calculated using the SHA256 algorithm.
///
/// # Arguments
/// - `file`: A reference to an open file
/// - `address`: Portion of the ROM that should be hashed
fn hash(file: &mut File, address: u64) -> String {
    use {
        sha2::{Digest, Sha256},
        std::io::{Seek, SeekFrom},
    };

    // Read specific portion of the ROM
    file.seek(SeekFrom::Start(address)).unwrap();
    let mut buffer = vec![0; 156];
    file.read_exact(&mut buffer).unwrap();

    // Hash the specific portion of the ROM
    let mut hasher = Sha256::new();
    hasher.update(&buffer);

    // Return the hash as a hexadecimal string
    format!("{:x}", hasher.finalize())
}

/// Trims the ROM file by removing 0x00 & 0xFF 0xFF at the end of the file.
///
/// # Arguments
/// - `file`: A reference to an open file with write permissions
/// # Errors
/// This function will return an error if it is unable to determine the length of the ROM or if something goes wrong with the file I/O.
pub fn trim(file: &mut File, _rom_type: Type) -> std::io::Result<()> {
    // Read file to buffer
    file.seek(SeekFrom::Start(0))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Find last relevant position in the file
    let trim_position = buffer
        .iter()
        .rposition(|&byte| byte != 0x00 && byte != 0xFF)
        .unwrap_or(0)
        + 1;

    // Trim the file
    file.set_len(trim_position as u64)?;

    Ok(())
}
