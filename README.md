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
1. Type `export LIBRARY_PATH="$LIBRARY_PATH:$(brew —prefix)/lib"` to your terminal.
1. Run `cargo build`
1. When running the game, `assets` directory and `music` directory should be in the working directory.


### Windows
TBA

## Docs
- [Glossary](docs/glossary.md)

## Copyright
Copyright (C) 2024 Yeonjin Shin, Homin Lee, Habin Song. All rights reserved.

This program, including source code, cannot be used/copied/redistributed/modified without permission.

### Assets (Image, Font, ...etc)
 - Noto Sans KR: Copyright (C) Google, licensed under SIL Open Font License
 - [Press Start](https://www.fontspace.com/press-start-2p-font-f11591): Copyright (c) 2011, Cody "CodeMan38" Boisclair (cody@zone38.net), licensed under SIL Open Font License (OFL)
 - Janggu free icon: [Korea icons created by Freepik - Flaticon](https://www.flaticon.com/free-icons/korea)