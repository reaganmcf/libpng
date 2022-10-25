use crate::error::DecodeError;

#[derive(Debug, PartialEq, Eq)]
pub enum ColorType {
    _0,
    _2,
    _3,
    _4,
    _6,
}

impl TryInto<ColorType> for u8 {
    type Error = DecodeError;
    fn try_into(self) -> Result<ColorType, Self::Error> {
        match self {
            0 => Ok(ColorType::_0),
            2 => Ok(ColorType::_2),
            3 => Ok(ColorType::_3),
            4 => Ok(ColorType::_4),
            6 => Ok(ColorType::_6),
            _ => Err(DecodeError::InvalidColorType),
        }
    }
}
