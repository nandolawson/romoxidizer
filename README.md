# romoxidizer

[![Open in Dev Container](https://img.shields.io/badge/Open%20in%20Dev%20Container-blue?style=flat&logo=docker&logoColor=%23fff)](https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/nandolawson/romoxidizer)

This software is used to trim ROMs for Nintendo Game Boy Advance and the future Nintendo DS. It is written entirely in Rust and works on Windows & Linux, regardless of whether the CPU architecture is x86 or ARM. As far as I know, this is the only open-source software of its kind.

> ⚠ DS ROMs with Nintendo Wifi Connection / Download Play functionality should not be trimmed at this time.
> ⚠ romoxidizer is currently in early development. Even though the software has been tested, only ROMs for which a backup exists should be processed.

## Usage

Pretty simple. Here is an example under Windows:

```powershell
romoxidizer.exe C:\path\to\rom
```

Done! The ROM is trimmed.

## Compiling

> ℹ It is recommended to use the provided Dev Container as it is pre-configured for this project

`just` and `cargo-edit` need to be installed via Cargo. Besides of them, you need to add `x86_64-unknown-linux-gnu` / `x86_64-pc-windows-gnu` as a target via rustup. It might also be possible that `mingw-w64` needs to be installed to compile for Windows on Linux.

Once everything is set up, run `just build`. If cross-compiling / a release build is desired, the command would be `just build release`.

## Contributing

Anyone who wants to contribute is more than welcome to do so. I would be delighted to learn from the contributions of other users. If you find a bug or have a feature in mind that you think would be useful, please feel free to create a pull request on GitHub.
If you decide to fork this project, please make sure to adhere to the [license](https://github.com/nandolawson/romoxidizer/blob/master/LICENSE) and the [contribution guidelines](https://github.com/nandolawson/romoxidizer/blob/master/CONTRIBUTING.md). Your involvement and feedback are highly appreciated!
