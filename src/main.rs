/*
romoxidizer
Copyright 2025 Nando Lawson

Licensed under the GPL, Version 3 <https://github.com/nandolawson/romoxidizer/blob/main/LICENSE>.
This file may not be copied, modified, or distributed except according to those terms.
*/

fn main() {
    use {
        romoxidizer::{
            check_file, detect_rom, trim,
            Type::{GBA, NDS},
        },
        std::{fs::metadata, process::exit},
    };

    // Get the path to the ROM file
    let args: Vec<String> = std::env::args().collect();

    // Check if the user provided a ROM file
    if args.len() != 2 {
        eprintln!("Usage: {} <rom>", args[0]);
        exit(1);
    }

    // Get the path to the ROM file
    let rom_path = args[1].as_str();

    // Check if the file exists and is at least 200 bytes long
    if !check_file(rom_path) {
        eprintln!("File is not supported or path is invalid");
        exit(1);
    }

    // Get the size of the ROM file
    let file_size_a = metadata(rom_path).map(|m| m.len()).unwrap_or(0);

    // Detect the type of ROM and trim it
    match detect_rom(rom_path) {
        Ok(ref rom_type) if rom_type == "GBA" => {
            if let Err(e) = trim(rom_path, GBA) {
                eprintln!("Trim failed: {e}");
            }
        }
        Ok(ref rom_type) if rom_type == "NDS" => {
            if let Err(e) = trim(rom_path, NDS) {
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
    let file_size_b = metadata(rom_path).map(|m| m.len()).unwrap_or(0);
    // Calculate the savings
    let savings = (file_size_a - file_size_b) / file_size_a * 100;

    // Print the results
    if savings == 0 {
        eprintln!("Error: No savings");
    } else {
        println!("Before: {file_size_a} | After: {file_size_b} | Savings: {savings:.2}%");
    }
}
