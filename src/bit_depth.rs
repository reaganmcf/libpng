use crate::error::DecodeError;

#[derive(Debug, PartialEq, Eq)]
pub enum BitDepth {
    _1,
    _2,
    _4,
    _8,
    _16,
}

impl TryInto<BitDepth> for u8 {
    type Error = DecodeError;
    fn try_into(self) -> Result<BitDepth, Self::Error> {
        match self {
            1 => Ok(BitDepth::_1),
            2 => Ok(BitDepth::_2),
            4 => Ok(BitDepth::_4),
            8 => Ok(BitDepth::_8),
            16 => Ok(BitDepth::_16),
            _ => Err(DecodeError::InvalidBitDepth),
        }
    }
}
