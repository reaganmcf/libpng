use crate::{bitdepth::BitDepth, color_type::ColorType, error::DecodeError, interlace_method::InterlaceMethod};

#[derive(Debug)]
pub enum ChunkType {
    IHDR,
}

impl TryInto<ChunkType> for &[u8] {
    type Error = DecodeError;
    fn try_into(self: Self) -> Result<ChunkType, Self::Error> {
        match self {
            &[73, 72, 68, 82] => Ok(ChunkType::IHDR),
            _ => Err(DecodeError::UnknownHeaderType),
        }
    }
}

#[derive(Debug)]
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
}

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    ty: ChunkType,
    data: Option<ChunkData>,
    crc: u32,
}
