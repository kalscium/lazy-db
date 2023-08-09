use super::*;
use crate::LDBError;

impl LazyINumType {
    pub fn to_byte(self) -> u8 {
        use LazyINumType::*;
        match self {
            I8 => 0,
            I16 => 1,
            I32 => 2,
            I64 => 3,
        }
    }
}

impl LazyUNumType {
    pub fn to_byte(self) -> u8 {
        use LazyUNumType::*;
        match self {
            U8 => 0,
            U16 => 1,
            U32 => 2,
            U64 => 3,
        }
    }
}

impl LazyFloatType {
    pub fn to_byte(self) -> u8 {
        use LazyFloatType::*;
        match self {
            F32 => 0,
            F64 => 1,
        }
    }
}

impl LazyType {
    pub fn to_bytes(self) -> [u8; 2] {
        use LazyType::*;
        match self {
            Void => [0, 0],
            Custom(x) => [1, x],
            String => [2, 0],
            INum(x) => [3, x.to_byte()],
            UNum(x) => [4, x.to_byte()],
            Float(x) => [5, x.to_byte()],
            Binary => [6, 0],
        }
    }
}

impl LazyINumType {
    pub fn from_byte(byte: u8) -> Result<Self, LDBError> {
        use LazyINumType::*;
        match byte {
            0 => Ok(I8),
            1 => Ok(I16),
            2 => Ok(I32),
            3 => Ok(I64),
            _ => Err(LDBError::InvalidLazyType(byte)),
        }
    }
}

impl LazyUNumType {
    pub fn from_byte(byte: u8) -> Result<Self, LDBError> {
        use LazyUNumType::*;
        match byte {
            0 => Ok(U8),
            1 => Ok(U16),
            2 => Ok(U32),
            3 => Ok(U64),
            _ => Err(LDBError::InvalidLazyType(byte)),
        }
    }
}

impl LazyFloatType {
    pub fn from_byte(byte: u8) -> Result<Self, LDBError> {
        use LazyFloatType::*;
        match byte {
            0 => Ok(F32),
            1 => Ok(F64),
            _ => Err(LDBError::InvalidLazyType(byte)),
        }
    }
}

impl LazyType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, LDBError> {
        use LazyType::*;
        Ok(match bytes {
            [0, _] => Void,
            [1, x] => Custom(*x),
            [2, _] => String,
            [3, x] => INum(LazyINumType::from_byte(*x)?),
            [4, x] => UNum(LazyUNumType::from_byte(*x)?),
            [5, x] => Float(LazyFloatType::from_byte(*x)?),
            [6, _] => Binary,
            [x, _] => return Err(LDBError::InvalidLazyType(*x)),
            _ => return Err(LDBError::InvalidLazyType(44)), // 44 as in 404 not found ( replace with better error handling )
        })
    }
}