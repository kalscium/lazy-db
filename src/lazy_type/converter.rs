use super::*;

impl Into<u8> for LazyINumType {
    fn into(self) -> u8 {
        use LazyINumType::*;
        match self {
            I8 => 0,
            I16 => 1,
            I32 => 2,
            I64 => 3,
            I128 => 4,
        }
    }
}

impl Into<u8> for LazyUNumType {
    fn into(self) -> u8 {
        use LazyUNumType::*;
        match self {
            U8 => 0,
            U16 => 1,
            U32 => 2,
            U64 => 3,
            U128 => 4,
        }
    }
}

impl Into<u8> for LazyFloatType {
    fn into(self) -> u8 {
        use LazyFloatType::*;
        match self {
            F32 => 0,
            F64 => 1,
        }
    }
}

impl Into<[u8; 2]> for LazyType {
    fn into(self) -> [u8; 2] {
        use LazyType::*;
        match self {
            Void => [0, 0],
            Custom(x) => [1, x],
            String => [2, 0],
            INum(x) => [3, x.into()],
            UNum(x) => [3, x.into()],
            Float(x) => [4, x.into()]
        }
    }
}
