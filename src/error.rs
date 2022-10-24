#[derive(Debug)]
pub enum DecodeError {
    MissingSignature,
    UnexpectedEndOfFile,
    UnknownHeaderType,
    InvalidBitDepth,
    InvalidColorType,
    InvalidInterlaceMethod,
}
