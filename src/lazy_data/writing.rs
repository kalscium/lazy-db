use super::*;

macro_rules! new_number {
    (($name:ident) $type:ty = $lazy_type:expr) => {
        /// Creates a new `LazyData` file with an unsigned integer and type
        pub fn $name(mut file: FileWrapper, value: $type) -> Result<(), LDBError> {
            let bytes = value.to_be_bytes();
            file.write(&[$lazy_type.into()])?;
            file.write(&bytes)?;
            Ok(())
        }
    };

    (signed ($name:ident) $type:ty = $lazy_type:expr) => {
        /// Creates a new `LazyData` file with a signed integer and type
        pub fn $name(mut file: FileWrapper, value: $type) -> Result<(), LDBError> {
            let bytes = value.to_be_bytes();
            file.write(&[$lazy_type.into()])?;
            file.write(&bytes)?;
            Ok(())
        }
    };
}

impl LazyData {
    /// Creates a new `LazyData` file with the type of `LazyType::Void`
    pub fn new_void(mut file: FileWrapper, _value: ()) -> Result<(), LDBError> {
        file.write(&[LazyType::Void.into()])?;
        Ok(())
    }

    /// Creates a new `LazyData` file with a `String` value and type
    pub fn new_string(mut file: FileWrapper, value: &str) -> Result<(), LDBError> {
        let bytes = value.as_bytes();
        file.write(&[LazyType::String.into()])?;
        file.write(bytes)?;
        Ok(())
    }

    // Signed Integers
    new_number!(signed (new_i8) i8 = LazyType::I8);
    new_number!(signed (new_i16) i16 = LazyType::I16);
    new_number!(signed (new_i32) i32 = LazyType::I32);
    new_number!(signed (new_i64) i64 = LazyType::I64);
    new_number!(signed (new_i128) i128 = LazyType::I128);

    // Unsigned Integers
    new_number!((new_u8) u8 = LazyType::U8);
    new_number!((new_u16) u16 = LazyType::U16);
    new_number!((new_u32) u32 = LazyType::U32);
    new_number!((new_u64) u64 = LazyType::U64);
    new_number!((new_u128) u128 = LazyType::U128);

    /* Floating point numbers */

    /// Creates a new `LazyData` file with an `f32` value and type
    pub fn new_f32(mut file: FileWrapper, value: f32) -> Result<(), LDBError> {
        let bytes = value.to_be_bytes();
        file.write(&[LazyType::F32.into()])?;
        file.write(&bytes)?;
        Ok(())
    }

    /// Creates a new `LazyData` file with an `f64` value and type
    pub fn new_f64(mut file: FileWrapper, value: f64) -> Result<(), LDBError> {
        let bytes = value.to_be_bytes();
        file.write(&[LazyType::F64.into()])?;
        file.write(&bytes)?;
        Ok(())
    }

    /// Creates a new `LazyData` file with a `binary` value and type
    pub fn new_binary(mut file: FileWrapper, value: &[u8]) -> Result<(), LDBError> {
        file.write(&[LazyType::Binary.into()])?;
        file.write(value)
    }
}