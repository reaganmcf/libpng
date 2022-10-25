#[derive(Debug)]
pub enum DecodeError {
    MissingSignature,
    UnexpectedEndOfFile,
    UnknownChunkType,
    InvalidBitDepth,
    InvalidColorType,
    InvalidInterlaceMethod,
}
