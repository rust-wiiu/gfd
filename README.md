# GFD - Nintendo Wii U™ Shader Binary Format

Unlike more modern graphics APIs the Nintendo Wii U does not have a online shader compiler, so the shaders have to be pre-compiled and embedded into the program or stored in the file system. The most common format for such shader binaries is GFD (`.gsh` / `.gtx`). This crate enables easy parsing of the binary format into "Rust-native" structures (Vec & String instead of raw pointers).

**This crate does not provide a shader compiler**.

## Usage

```rust,no_run
use gfd::GFD;
use std::fs;

let bytes = fs::read("shader.gsh").unwrap();
let gfd = GFD::parse(&bytes).unwrap();
```

## Sources

- [Gfx2 - Nintendo File Formats](https://nintendo-formats.com/libs/gfd/gfx2.html)
- [decaf-emu / libgfd](https://github.com/decaf-emu/decaf-emu)
- [wut / libwhb](https://github.com/devkitPro/wut)
