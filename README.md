# GFD - Nintendo Wii U™ Shader Binary Format

Unlike more modern graphics APIs the Nintendo Wii U does not have a online shader compiler, so the shaders have to be pre-compiled, stored, and embedded into the program or stored in the file system. The most common format for such shader binaries is GFD (`.gsh` / `.gtx`). This crate enables easy serializing / deserializing of the binary format.

**This crate does not provide a shader compiler**.

## Usage

```rust,no_run
use gfd::GFD;
use std::fs;

let file = fs::read("shader.gsh").unwrap();
let gfd = GFD::try_from(file.as_ref()).unwrap();
```

The crate is `no_std` by default. std features of [deku](https://docs.rs/deku/latest/deku/), the underlying binary parser, can be enabled with the `std` feature flag.

## Sources

- [Gfx2 - Nintendo File Formats](https://nintendo-formats.com/libs/gfd/gfx2.html)
- [decaf-emu / libgfd](https://github.com/decaf-emu/decaf-emu)
- [wut / libwhb](https://github.com/devkitPro/wut)
