#[derive(Debug)]
pub enum DecodeError {
    MissingSignature,
    UnexpectedEndOfFile,
    UnknownChunkType,
    InvalidIHDRLength,
    InvalidBitDepth,
    InvalidColorType,
    InvalidInterlaceMethod,
    UnexpectedtRNSChunk
}
