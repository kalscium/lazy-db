#[derive(Debug, PartialEq, Eq)]
pub enum LazyINumType {
    I8,
    I16,
    I32,
    I64,
    I128,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LazyUNumType {
    U8,
    U16,
    U32,
    U64,
    U128,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LazyFloatType {
    F32,
    F64,
}