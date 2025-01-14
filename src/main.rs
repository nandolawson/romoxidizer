fn main() -> std::io::Result<()> {
    use {
        romoxidizer::{
            check_file, detect_rom, trim,
            Type::{GBA, NDS},
        },
        std::{fs::metadata, process::exit},
    };

    // Get the path to the ROM file
    let args: Vec<String> = std::env::args().collect();

    // Check if the user provided an argument
    if args.len() != 2 {
        eprintln!("Usage: {} <rom>", args[0]);
        exit(1);
    }

    // Get the path to the ROM file
    let rom_path = args[1].as_str();

    // Check if the file exists and is at least 1024 bytes long
    if !check_file(rom_path) {
        eprintln!("File is not supported or path is invalid");
        exit(1);
    }

    // Get the size of the ROM file
    let file_size = metadata(rom_path).map(|m| m.len()).unwrap_or(0);

    // Open ROM file
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(rom_path)?;

    // Detect the type of ROM and trim it
    match detect_rom(&mut file) {
        Ok(ref rom_type) if rom_type == "GBA" => {
            if let Err(e) = trim(&mut file, GBA) {
                eprintln!("Trim failed: {e}");
            }
        }
        Ok(ref rom_type) if rom_type == "NDS" => {
            if let Err(e) = trim(&mut file, NDS) {
                eprintln!("Trim failed: {e}");
            }
        }
        // Print an error if the ROM is not supported
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1)
        }
        Ok(_) => unreachable!(),
    }

    // Get the size of the trimmed ROM file
    let file_size_trimmed = metadata(rom_path).map(|m| m.len()).unwrap_or(0);

    // Print the results
    if file_size == file_size_trimmed {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "No savings"));
    }
    println!(
        "Before: {file_size} | After: {file_size_trimmed} | Savings: {:.2}%",
        (file_size - file_size_trimmed) / file_size * 100 // Calculate the savings
    );
    Ok(())
}
