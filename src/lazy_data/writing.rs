use super::*;

impl LazyData {
    /// Creates a new `LazyData` file with the type of `LazyType::Void`
    #[inline]
    pub fn new_void(path: impl AsRef<Path>) -> Result<(), LDBError> { Self::uninit(path)?; Ok(()) }

    /// Creates a new `LazyData` file with a `String` value and type
    pub fn new_string(path: impl AsRef<Path>, value: &str) -> Result<(), LDBError> {
        let mut lazy_data = Self::uninit(path)?;
        lazy_data.lazy_type = LazyType::String;
        todo!();
        // for i in value.as_bytes() {
        //     if let Err(OFileError::IOError(e)) = ofile.write(*i) {
        //         return Err(LDBError::IOError(e))
        //     };
        // }; Ok(())
    }
}