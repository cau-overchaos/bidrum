# 바이드럼(Bidrum)
**바이드럼**(**Bidrum** in english) is a rhythm game with janggu(korean traditional drum)-style controller.

## How to build?
### Linux
1. Install rust
1. Install ffmpeg, sdl2, sdl2_mixer, sdl2_image, sdl2_ttf.
    - For instructions on sdl2 installation, see [rust-sdl2 README](https://github.com/Rust-SDL2/rust-sdl2).
1. Run `cargo build`
1. When running the game, `assets` directory and `music` directory should be in the working directory.

### Mac OS
1. Install rust
1. Install ffmpeg, sdl2, sdl2_mixer, sdl2_image, sdl2_ttf.
    - For instructions on sdl2 installation, see [rust-sdl2 README](https://github.com/Rust-SDL2/rust-sdl2).
1. Type `export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"` to your terminal.
1. Run `cargo build`
1. When running the game, `assets` directory and `music` directory should be in the working directory.


### Windows
Due to difficulties on building Rust FFmpeg library on Windows, The GNU toolchain is used instead of MSVC toolchain.

1. Install [MSYS2](https://www.msys2.org)
1. Install Rust with [rustup](https://rustup.rs)
1. Run following command on **MSYS2 SHELL**. (**NOT POWERSHELL / CMD / GIT BASH**)
    - Append `--noconfirm` parameter if you want to say "yes" to all the questions automatically
    ```bash
    pacman -S mingw-w64-x86_64-toolchain mingw-w64-x86_64-ffmpeg mingw-w64-x86_64-clang mingw-w64-x86_64-SDL2 mingw-w64-x86_64-SDL2_ttf mingw-w64-x86_64-SDL2_image mingw-w64-x86_64-SDL2_mixer
    ```
1. (Optional) Run the following commands in PowerShell or cmd and remember default host and default toolchain.
    - You can use the default host and default toolchain remembered here after building bidrum, with `rustup set default-host` and `rustup default` commands.
    ```powershell
    rustup toolchain list
    rustup show
    ```
1. Run following commands in PowerShell or cmd. If you had ran these commands before, you can skip this step.
    ```powershell
    rustup toolchain install stable-gnu
    rustup target add x86_64-pc-windows-gnu
    ```
1. Run following commands in PowerShell or cmd
    - Please note that `rustup set default-host` and `rustup default` command is permanent.
    ```powershell
    rustup set default-host x86_64-pc-windows-gnu
    rustup default stable-x86_64-pc-windows-gnu
    ```
1. Create `.cargo/config.toml` file and write the following to the file.
    - The following assumes that MSYS2 installation path is `C:\msys64`. If the installation path is different, Modify it.
    ```toml
    [target.x86_64-pc-windows-gnu]
    linker = "C:\\msys64\\mingw64\\bin\\gcc.exe"
    ar = "C:\\msys64\\mingw64\\bin\\ar.exe"
    ```
1. Add `(MSYS2 installation path)\mingw64\bin` to `PATH` environment variable
1. Run `cargo build --target x86_64-pc-windows-gnu` on PowerShell or cmd
1. When running the game, `assets` directory and `music` directory should be in the working directory.

## Docs
- [Glossary](docs/glossary.md)

## Copyright
Copyright (C) 2024 Yeonjin Shin, Homin Lee, Habin Song. All rights reserved.

This program, including source code, cannot be used/copied/redistributed/modified without permission.

### Assets (Image, Font, ...etc)
 - Noto Sans KR: Copyright (C) Google, licensed under SIL Open Font License
 - [Press Start](https://www.fontspace.com/press-start-2p-font-f11591): Copyright (c) 2011, Cody "CodeMan38" Boisclair (cody@zone38.net), licensed under SIL Open Font License (OFL)
 - Janggu free icon: [Korea icons created by Freepik - Flaticon](https://www.flaticon.com/free-icons/korea)