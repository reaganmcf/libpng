use crate::error::DecodeError;

#[derive(Debug)]
pub enum InterlaceMethod {
    None,
    Adam7,
}

impl TryInto<InterlaceMethod> for u8 {
    type Error = DecodeError;
    fn try_into(self) -> Result<InterlaceMethod, Self::Error> {
        match self {
            0 => Ok(InterlaceMethod::None),
            1 => Ok(InterlaceMethod::Adam7),
            _ => Err(DecodeError::InvalidInterlaceMethod),
        }
    }
}
