use std::io::Result;

fn main() -> Result<()> {
    use std::env;

    // Add resources to Windows executable
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        add_windows_resources()?;
    }

    Ok(())
}

fn add_windows_resources() -> Result<()> {
    use winresource::{
        VersionInfo::{FILEOS, FILETYPE, FILEVERSION},
        WindowsResource,
    };

    let mut res = WindowsResource::new();

    let version_info = [
        (FILEOS, 0x40004), // Flags executable as compatible with Windows NT
        (FILETYPE, 1),     // Flags file as executable
        (FILEVERSION, 0x0001_0000_0000_0000), // Unimportant file version number
    ];

    res.set_icon("assets/logo.ico") // Icon for the executable
        .set_language(0x0009) // English
        .set_manifest_file("assets/manifest.xml"); // Some information about priviliges
    for (key, value) in version_info {
        res.set_version_info(key, value); // Some infos found in the variable "version_info"
    }

    res.compile()?;

    Ok(())
}
