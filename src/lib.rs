#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://docs.rs/-/rustdoc.static/favicon-32x32-422f7d1d52889060.png",
    issue_tracker_base_url = "https://github.com/nandolawson/keyforge95/issues",
    html_no_source
)]
#![doc = include_str!("../README.md")] // Adding the README to the documentation

pub mod modals;

use {
    modals::{
        Error,
        Platform::{self, Gba, Nds},
    },
    std::{
        fs::File,
        io::{Read, Seek, SeekFrom},
    },
};

/// Checks if the file exists and is at least 1024 bytes long.
/// Also detects the type of ROM based on the SHA256 hash of the Nintendo logo within the ROM header.
/// A Game Boy Advance ROM will contain the Logo at 0x004.
/// A DS ROM will contain the Logo at 0x0C0.
///
/// # Arguments
/// - `file`: A reference to an open file
///
/// # Returns
/// - `Ok`: Returns if the file is a legitimate ROM file
/// - `Err`: Returns an error if the file is not supported
///
/// # Errors
/// This function will return an error if the file isn't a legitimate GBA / DS ROM.
/// That is the case if the file is under 1024 bytes in size or if the Nintendo Logo can't be found.
pub fn detect_rom(file: &mut File) -> Result<(), Error> {
    // Check if the file contains at least 1024 bytes
    if file.metadata().map(|m| m.len() < 0x400).unwrap_or(true) {
        return Err(Error::TooSmall); // File is too small
    }

    // Hash of the Nintendo logo
    // The hash is the same for both ROM types
    let nintendo_logo_hash = "08a0153cfd6b0ea54b938f7d209933fa849da0d56f5a34c481060c9ff2fad818";

    // Check if the GBA or NDS Nintendo logo is present
    if hash(file, Gba) == nintendo_logo_hash || hash(file, Nds) == nintendo_logo_hash {
        Ok(())
    } else {
        Err(Error::NotSupported) // Return an error if the file is not supported
    }
}

/// Hashes a portion of the ROM file.
/// The hash is calculated using the SHA256 algorithm.
///
/// # Arguments
/// - `file`: A reference to an open file
/// - `platform`: Type of ROM. `Gba` & `Nds` are supported.
fn hash(file: &mut File, platform: Platform) -> String {
    use {
        sha2::{Digest, Sha256},
        std::io::{Seek, SeekFrom},
    };
    // Read specific portion of the ROM
    file.seek(SeekFrom::Start(platform.logoaddress())).unwrap();
    let mut buffer = vec![0x000; 0x09C];
    file.read_exact(&mut buffer).unwrap();

    // Hash the specific portion of the ROM
    let mut hasher = Sha256::new();
    hasher.update(&buffer);

    // Return the hash as a hexadecimal string
    format!("{:x}", hasher.finalize())
}

/// Trims the ROM file by removing 0x00 & 0xFF at the end of the file.
///
/// # Arguments
/// - `file`: A reference to an open file with write permissions
/// # Errors
/// This function will return an error if there is nothing to trim within the file. It will also return an error if any IO error returns.
pub fn trim(file: &mut File) -> Result<(), Error> {
    // Read file to buffer
    file.seek(SeekFrom::Start(0x000)).map_err(|_| Error::Io)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|_| Error::Io)?;

    // Find last relevant position in the file
    let trim_position = buffer
        .iter()
        .rposition(|&byte| byte != 0x00 && byte != 0xFF)
        .unwrap_or(0)
        + 0x001;

    // Check if the file is already trimmed
    if trim_position == buffer.len() {
        return Err(Error::AlreadyTrimmed);
    }

    // Trim the file
    if file.set_len(trim_position as u64).is_err() {
        return Err(Error::Io);
    }
    Ok(())
}
