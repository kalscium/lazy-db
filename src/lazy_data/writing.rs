use super::*;

impl LazyData {
    /// Creates a new `LazyData` file with the type of `LazyType::Void`
    #[inline]
    pub fn new_void(path: impl AsRef<Path>) -> Result<(), LDBError> { Self::uninit(path)?; Ok(()) }
}