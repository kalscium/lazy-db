mod converter;
pub use converter::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum LazyType {
    Void,
    String,
    Binary,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    True,
    False,
    Link,
    Array,
}