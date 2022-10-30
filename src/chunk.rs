#![allow(clippy::upper_case_acronyms)]
use std::fmt::Debug;

use crate::{
    bit_depth::BitDepth, color_type::ColorType, error::DecodeError,
    interlace_method::InterlaceMethod,
};

#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ChunkType {
    IHDR,
    IDAT,
    IEND,
    gAMA,
    PLTE,
    bKGD,
}

impl TryInto<ChunkType> for &[u8] {
    type Error = DecodeError;
    fn try_into(self) -> Result<ChunkType, Self::Error> {
        match *self {
            [73, 72, 68, 82] => Ok(ChunkType::IHDR),
            [73, 68, 65, 84] => Ok(ChunkType::IDAT),
            [73, 69, 78, 68] => Ok(ChunkType::IEND),
            [103, 65, 77, 65] => Ok(ChunkType::gAMA),
            [80, 76, 84, 69] => Ok(ChunkType::PLTE),
            [98, 75, 71, 68] => Ok(ChunkType::bKGD),
            _ => {
                println!("Unknown chunk type: {:?}", self);
                Err(DecodeError::UnknownChunkType)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BackgroundData {
    Grayscale(u16),
    RGB((u16, u16, u16)),
    PaletteIndex(u8)
}

#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum ChunkData {
    IHDR {
        width: u32,
        height: u32,
        bit_depth: BitDepth,
        color_type: ColorType,
        compression_method: u8, // ignored for now
        filter_method: u8,      // ignored for now
        interlace_method: InterlaceMethod,
    },
    IDAT(Vec<u8>),
    IEND,
    gAMA {
        image_gamma: f64,
    },
    PLTE(Vec<(u8, u8, u8)>),
    bKGD(BackgroundData)
}

impl std::fmt::Debug for ChunkData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IHDR {
                width,
                height,
                bit_depth,
                color_type,
                compression_method,
                filter_method,
                interlace_method,
            } => f
                .debug_struct("IHDR")
                .field("width", width)
                .field("height", height)
                .field("bit_depth", bit_depth)
                .field("color_type", color_type)
                .field("compression_method", compression_method)
                .field("fitler_method", filter_method)
                .field("interlace_method", interlace_method)
                .finish(),
            Self::IDAT(data) => f.write_fmt(format_args!("IDAT ({} bytes)", data.len())),
            Self::IEND => f.write_str("No data"),
            Self::gAMA { image_gamma } => f
                .debug_struct("gAMA")
                .field("image_gamma", image_gamma)
                .finish(),
            Self::PLTE(entries) => f.debug_tuple("PLTE").field(entries).finish(),
            Self::bKGD(bg_data) => f.debug_tuple("bKGD").field(bg_data).finish()

        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub length: u32,
    pub ty: ChunkType,
    pub data: ChunkData,
    pub crc: u32,
}
