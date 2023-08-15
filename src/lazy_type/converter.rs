use super::*;
use crate::LDBError;
use std::convert::TryFrom;


impl TryFrom<u8> for LazyType {
    type Error = LDBError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        use LazyType::*;
        Ok(match byte {
            0 => Void,
            1 => String,
            2 => Binary,
            3 => I8,
            4 => I16,
            5 => I32,
            6 => I64,
            7 => I128,
            8 => U8,
            9 => U16,
            10 => U32,
            11 => U64,
            12 => U128,
            13 => F32,
            14 => F64,
            15 => True,
            16 => False,
            _ => return Err(LDBError::InvalidLazyType(byte)),
        })
    }
}

impl From<LazyType> for u8 {
    fn from(value: LazyType) -> Self {
        use LazyType::*;
        match value {
            Void => 0,
            String => 1,
            Binary => 2,
            I8 => 3,
            I16 => 4,
            I32 => 5,
            I64 => 6,
            I128 => 7,
            U8 => 8,
            U16 => 9,
            U32 => 10,
            U64 => 11,
            U128 => 12,
            F32 => 13,
            F64 => 14,
            True => 15,
            False => 16,
        }
    }
}