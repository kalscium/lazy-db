use super::*;

impl LazyData {
    /// Creates a new `LazyData` file with the type of `LazyType::Void`
    pub fn new_void(path: impl AsRef<Path>) -> Result<(), LDBError> {
        Self::new_writer(path, LazyType::Void)?;
        Ok(())
    }

    #[inline]
    fn new_writer(path: impl AsRef<Path>, lazy_type: LazyType) -> Result<FileWrapper, LDBError> {
        let mut writer = FileWrapper::new_writer(unwrap_result!(std::fs::File::create(path) => |e| Err(LDBError::IOError(e))));
        writer.write(&lazy_type.to_bytes())?;
        Ok(writer)
    }

    /// Creates a new `LazyData` file with a `String` value and type
    pub fn new_string(path: impl AsRef<Path>, value: &str) -> Result<(), LDBError> {
        let bytes = value.as_bytes();
        let mut writer = Self::new_writer(&path, LazyType::String)?;
        writer.write(bytes)?;
        Ok(())
    }
}