use super::*;

macro_rules! incorrect_type {
    ($original:expr, $expected:pat) => {
        if let $expected = $original {}
        else {
            return Err(LDBError::IncorrectType($original, stringify!($expected).to_string()));
        };
    }
}

impl LazyData {
    /// ### Expensive Action
    /// ( Reads all of the contents of the file and stores it on the heap )
    /// 
    /// ---
    /// Collects the `LazyData` as a `Box<[u8]>`.
    /// 
    /// Returns `LDBError::IncorrectType` if the LazyData type is not `LazyType::Binary`
    pub fn collect_binary(self) -> Result<Box<[u8]>, LDBError> {
        incorrect_type!(self.lazy_type, LazyType::Binary);
        self.wrapper.read_to_end()
    }

    /// ### Expensive Action
    /// ( Loads the entire file's data into memory )
    /// 
    /// ---
    /// Collects the `LazyData` as a `String`
    /// 
    /// Returns `LDBError::IncorrectType` if LazyData type is not `LazyType::String`
    /// Returns `LDBError::IOError` if there is an io error while reading file contents
    pub fn collect_string(self) -> Result<String, LDBError> {
        incorrect_type!(self.lazy_type, LazyType::String);
        // Expensive and best to be avoided if possible
        let bytes = self.wrapper.read_to_end()?;
        
        if let Ok(x) = String::from_utf8(bytes.to_vec()) {
            Ok(x)
        } else {
            Err(LDBError::InvalidUTF8String(bytes))
        }
    }
}