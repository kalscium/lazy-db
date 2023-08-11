use super::*;

macro_rules! new_number {
    (($name:ident) $type:ty = $lazy_type:expr) => {
        /// Creates a new `LazyData` file with an unsigned integer and type
        pub fn $name(path: impl AsRef<Path>, value: $type) -> Result<(), LDBError> {
            let bytes = value.to_be_bytes();
            let mut writer = Self::new_writer(&path, $lazy_type)?;
            writer.write(&bytes)?;
            Ok(())
        }
    };

    (signed ($name:ident) $type:ty = $lazy_type:expr) => {
        /// Creates a new `LazyData` file with a signed integer and type
        pub fn $name(path: impl AsRef<Path>, value: $type) -> Result<(), LDBError> {
            let bytes = value.to_be_bytes();
            let mut writer = Self::new_writer(&path, $lazy_type)?;
            writer.write(&bytes)?;
            Ok(())
        }
    };
}

impl LazyData {
    /// Creates a new `LazyData` file with the type of `LazyType::Void`
    pub fn new_void(path: impl AsRef<Path>) -> Result<(), LDBError> {
        Self::new_writer(path, LazyType::Void)?;
        Ok(())
    }

    #[inline]
    fn new_writer(path: impl AsRef<Path>, lazy_type: LazyType) -> Result<FileWrapper, LDBError> {
        let mut writer = FileWrapper::new_writer(unwrap_result!(std::fs::File::create(path) => |e| Err(LDBError::IOError(e))));
        writer.write(&[lazy_type.into()])?;
        Ok(writer)
    }

    /// Creates a new `LazyData` file with a `String` value and type
    pub fn new_string(path: impl AsRef<Path>, value: &str) -> Result<(), LDBError> {
        let bytes = value.as_bytes();
        let mut writer = Self::new_writer(&path, LazyType::String)?;
        writer.write(bytes)?;
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
    pub fn new_f32(path: impl AsRef<Path>, value: f32) -> Result<(), LDBError> {
        let bytes = value.to_be_bytes();
        let mut writer = Self::new_writer(&path, LazyType::F32)?;
        writer.write(&bytes)?;
        Ok(())
    }

    /// Creates a new `LazyData` file with an `f64` value and type
    pub fn new_f64(path: impl AsRef<Path>, value: f64) -> Result<(), LDBError> {
        let bytes = value.to_be_bytes();
        let mut writer = Self::new_writer(&path, LazyType::F64)?;
        writer.write(&bytes)?;
        Ok(())
    }
}