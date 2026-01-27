#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

extern crate alloc;

use alloc::vec::Vec;
use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian", id_type = "u32")]
pub enum Type {
    #[deku(id = "0x01")]
    EndOfFile,
    #[deku(id = "0x02")]
    Padding,
    #[deku(id = "0x03")]
    VertexShaderHeader,
    #[deku(id = "0x05")]
    VertexShaderProgram,
    #[deku(id = "0x06")]
    PixelShaderHeader,
    #[deku(id = "0x07")]
    PixelShaderProgram,
    #[deku(id = "0x08")]
    GeometryShaderHeader,
    #[deku(id = "0x09")]
    GeometryShaderProgram,
    #[deku(id = "0x010")]
    GeometryShaderCopyProgram,
    #[deku(id = "0x011")]
    TextureHeader,
    #[deku(id = "0x012")]
    TextureImageData,
    #[deku(id = "0x013")]
    TextureMipmapData,
    #[deku(id = "0x014")]
    ComputeShaderHeader,
    #[deku(id = "0x015")]
    ComputeShaderProgram,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian", magic = b"Gfx2")]
pub struct Header {
    #[deku(assert_eq = "0x20")]
    pub size: u32,
    #[deku(assert_eq = "(7, 1)")]
    pub version: (u32, u32),
    #[deku(assert_eq = "2")]
    pub gpu: u32,
    #[deku(map = "|x: u32| -> Result<_, DekuError> { Ok( x != 0 ) }")]
    pub align: bool,
    pub padding: u64,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian", magic = b"BLK{")]
pub struct Block {
    #[deku(assert_eq = "0x20")]
    pub size: u32,
    #[deku(assert_eq = "(1, 0)")]
    pub version: (u32, u32),
    pub r#type: Type,
    #[deku(update = "self.data.len()")]
    pub len: u32,
    pub id: u32,
    pub index: u32,
    #[deku(count = "len")]
    pub data: Vec<u8>,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct GFD {
    pub header: Header,
    #[deku(until = "|b: &Block| matches!(b.r#type, Type::EndOfFile)")]
    pub blocks: Vec<Block>,
}
