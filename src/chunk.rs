#![allow(clippy::upper_case_acronyms)]
use std::fmt::Debug;

use crate::{
    bit_depth::BitDepth, color_type::ColorType, error::DecodeError,
    interlace_method::InterlaceMethod,
};

#[derive(Debug, PartialEq, Eq)]
pub enum ChunkType {
    IHDR,
    IDAT,
    IEND,
}

impl TryInto<ChunkType> for &[u8] {
    type Error = DecodeError;
    fn try_into(self) -> Result<ChunkType, Self::Error> {
        match *self {
            [73, 72, 68, 82] => Ok(ChunkType::IHDR),
            [73, 68, 65, 84] => Ok(ChunkType::IDAT),
            [73, 69, 78, 68] => Ok(ChunkType::IEND),
            _ => {
                println!("Unknown chunk type: {:?}", self);
                Err(DecodeError::UnknownChunkType)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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
}

#[derive(Debug)]
pub struct Chunk {
    pub length: u32,
    pub ty: ChunkType,
    pub data: ChunkData,
    pub crc: u32,
}
