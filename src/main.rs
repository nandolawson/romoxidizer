use {romoxidizer::modals::Error, std::process::exit};

fn main() -> Result<(), Error> {
    // Get all arguments
    let args: Vec<String> = std::env::args().collect();

    // Check if the user provided exactly one argument
    if args.len() != 2 {
        println!("Usage: {} <rom>", args[0]);
        exit(0);
    }

    cli(args[1].as_str())?;
    Ok(())
}

fn cli(path: &str) -> Result<(), Error> {
    use {
        romoxidizer::{check_file, detect_rom, trim},
        std::fs::metadata,
    };

    // Open ROM file
    let Ok(mut file) = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
    else {
        return Err(Error::InvalidPath);
    };

    // Check if the file exists and is at least 1024 bytes long
    check_file(&mut file)?;

    // Get the size of the ROM file
    let file_size = file.metadata().unwrap().len();

    // Detect the type of ROM and trim it
    detect_rom(&mut file)?;

    trim(&mut file)?;

    // Get the size of the trimmed ROM file
    let file_size_trimmed = metadata(path).map(|m| m.len()).unwrap_or(0);

    // Print the results
    println!(
        "Before: {file_size} | After: {file_size_trimmed} | Savings: {:.2}%",
        (file_size - file_size_trimmed) / file_size * 100 // Calculate the savings
    );

    Ok(())
}
